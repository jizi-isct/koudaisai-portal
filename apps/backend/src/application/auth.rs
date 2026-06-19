//! 認証ユースケース(`AuthApp`)。
//!
//! 事前認証エンドポイント(activate/login/refresh/reset)は `ActorContext` を取らない。
//! `change_password`/`logout_all` のみ認証済み actor を取る。複数書込は
//! `GroupApp::create_group` 規約に従い，ハンドラから `mut tx: Tx` を受け取る。
//!
//! 副作用(reuse 検知時の Discord 通知・リセットメール送信)は本層では行わず，
//! 戻り値/エラー(特に [`AuthError::ReuseDetected`] と [`PasswordResetIssued`])を介して
//! HTTP 層がリクエストパス外で実行する。

use crate::application::ports::access_token_issuer::AccessTokenIssuer;
use crate::application::ports::clock::Clock;
use crate::application::ports::password_hasher::{
    PasswordHashError, PasswordHasher, VerifyOutcome,
};
use crate::application::ports::repositories::one_time_token_repo::OneTimeTokenRepo;
use crate::application::ports::repositories::session_repo::SessionRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::ports::secret_generator::SecretGenerator;
use crate::application::transaction::Transaction;
use crate::domain::email_address::EmailAddress;
use crate::domain::one_time_token::{OneTimeToken, OneTimeTokenPurpose};
use crate::domain::one_time_token_id::OneTimeTokenId;
use crate::domain::password_credentials::PasswordCredentials;
use crate::domain::plaintext_password::PlaintextPassword;
use crate::domain::session::{RevocationReason, Session, TokenStatus};
use crate::domain::session_id::SessionId;
use crate::domain::token_id::TokenId;
use crate::domain::user::{User, UserStatus};
use crate::domain::user_id::UserId;
use chrono::{DateTime, Duration, Utc};
use std::marker::PhantomData;
use uuid::Uuid;

/// 認証ユースケースのエラー。HTTP 層がステータスコードへ写す。
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// 401。ユーザー不在 / パスワード不一致 / 未有効化(汎用化して列挙を防ぐ)。
    #[error("invalid credentials")]
    InvalidCredentials,
    /// 401。refresh / activation / reset トークンが不正・失効・消費済み。
    #[error("invalid or expired token")]
    InvalidToken,
    /// 401。リフレッシュトークン盗難(reuse)を検知しファミリを失効させた。
    #[error("token reuse detected")]
    ReuseDetected,
    /// 400。パスワードポリシー違反など。
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// 409。例: activation 済みのユーザーを再度有効化しようとした。
    #[error("conflict")]
    Conflict,
    /// 401/403。actor が未認証。
    #[error("unauthorized")]
    Unauthorized,
    #[error("internal error: {0}")]
    Internal(anyhow::Error),
}

impl From<anyhow::Error> for AuthError {
    fn from(e: anyhow::Error) -> Self {
        AuthError::Internal(e)
    }
}

impl From<PasswordHashError> for AuthError {
    fn from(e: PasswordHashError) -> Self {
        match e {
            PasswordHashError::Internal(e) => AuthError::Internal(e),
        }
    }
}

fn internal<E: Into<anyhow::Error>>(e: E) -> AuthError {
    AuthError::Internal(e.into())
}

/// 不透明トークン `"{id}.{secret}"` を分解する。
fn parse_opaque(s: &str) -> Option<(Uuid, &str)> {
    let (id, secret) = s.split_once('.')?;
    let id = Uuid::parse_str(id).ok()?;
    if secret.is_empty() {
        return None;
    }
    Some((id, secret))
}

/// 認証関連の TTL・上限値。
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub access_ttl: Duration,
    pub session_absolute_ttl: Duration,
    pub session_idle_ttl: Duration,
    pub max_sessions_per_user: usize,
    pub activation_ttl: Duration,
    pub reset_ttl: Duration,
}

/// ログイン/リフレッシュ成功時に発行されるトークン群。
#[derive(Debug, Clone)]
pub struct IssuedTokens {
    pub access_token: String,
    /// 不透明リフレッシュトークン `"{token_id}.{secret}"`。HTTP 層が `__Host-` Cookie に載せる。
    pub refresh_token: String,
    pub session_id: SessionId,
    pub access_ttl: Duration,
    /// ファミリの絶対期限。Cookie の Max-Age 算出に使う。
    pub refresh_expires_at: DateTime<Utc>,
}

/// パスワードリセット要求が受理された(=ユーザーが存在し Active)ときの結果。
/// HTTP 層がこれを使ってリクエストパス外でメールを送る。存在しなければ `None`。
#[derive(Debug, Clone)]
pub struct PasswordResetIssued {
    pub user_id: crate::domain::user_id::UserId,
    pub m_address: EmailAddress,
    pub raw_token: String,
    pub expires_at: DateTime<Utc>,
}

pub struct AuthApp<'a, Tx: Transaction, C: Clock> {
    _phantom: PhantomData<&'a Tx>,
    user_repo: &'a (dyn UserRepo<Tx> + Send + Sync),
    one_time_token_repo: &'a (dyn OneTimeTokenRepo<Tx> + Send + Sync),
    session_repo: &'a (dyn SessionRepo<Tx> + Send + Sync),
    password_hasher: &'a dyn PasswordHasher,
    secret_generator: &'a dyn SecretGenerator,
    access_token_issuer: &'a dyn AccessTokenIssuer,
    clock: &'a C,
    config: AuthConfig,
    /// 定数時間ログインのためのダミー PHC(現行パラメータで起動時に生成して渡す)。
    dummy_phc: String,
}

impl<'a, Tx: Transaction + Send, C: Clock + Send + Sync> AuthApp<'a, Tx, C> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_repo: &'a (dyn UserRepo<Tx> + Send + Sync),
        one_time_token_repo: &'a (dyn OneTimeTokenRepo<Tx> + Send + Sync),
        session_repo: &'a (dyn SessionRepo<Tx> + Send + Sync),
        password_hasher: &'a dyn PasswordHasher,
        secret_generator: &'a dyn SecretGenerator,
        access_token_issuer: &'a dyn AccessTokenIssuer,
        clock: &'a C,
        config: AuthConfig,
        dummy_phc: String,
    ) -> Self {
        Self {
            _phantom: PhantomData,
            user_repo,
            one_time_token_repo,
            session_repo,
            password_hasher,
            secret_generator,
            access_token_issuer,
            clock,
            config,
            dummy_phc,
        }
    }

    fn issue_for(
        &self,
        user_id: crate::domain::user_id::UserId,
        session_id: SessionId,
        token_id: TokenId,
        secret: String,
        refresh_expires_at: DateTime<Utc>,
    ) -> Result<IssuedTokens, AuthError> {
        let access_token =
            self.access_token_issuer
                .issue(user_id, session_id, self.config.access_ttl)?;
        Ok(IssuedTokens {
            access_token,
            refresh_token: format!("{token_id}.{secret}"),
            session_id,
            access_ttl: self.config.access_ttl,
            refresh_expires_at,
        })
    }

    /// begin → revoke_family_in → commit を 1 トランザクションで行う小ヘルパー。
    async fn revoke_family(
        &self,
        tx: &mut Tx,
        session_id: SessionId,
        reason: RevocationReason,
        now: DateTime<Utc>,
    ) -> Result<(), AuthError> {
        tx.begin().await.map_err(internal)?;
        self.session_repo
            .revoke_family_in(tx, session_id, reason, now)
            .await?;
        tx.commit().await.map_err(internal)?;
        Ok(())
    }

    /// 有効化(activation)。ワンタイムトークンを検証し，初回パスワードを設定する。
    pub async fn activate(
        &self,
        mut tx: Tx,
        token_str: &str,
        password: &str,
    ) -> Result<(), AuthError> {
        let (ott_id, secret) = parse_opaque(token_str).ok_or(AuthError::InvalidToken)?;
        let ott_id = OneTimeTokenId::new(ott_id);
        let token = self
            .one_time_token_repo
            .find_by_id(ott_id)
            .await
            .map_err(internal)?
            .ok_or(AuthError::InvalidToken)?;
        // 秘密を定数時間照合(id は秘密ではなくルックアップキー)。
        if !self
            .secret_generator
            .verify_secret(secret, token.token_hash())
        {
            return Err(AuthError::InvalidToken);
        }
        if token.purpose() != OneTimeTokenPurpose::Activation || !token.is_consumable(self.clock) {
            return Err(AuthError::InvalidToken);
        }

        let mut user = self
            .user_repo
            .find_by_id(token.user_id())
            .await
            .map_err(internal)?
            .ok_or(AuthError::InvalidToken)?;

        // トークンが宛先に束縛されている場合、現在の m_address と一致しなければ拒否する
        // (email 変更後に旧宛先宛のトークンでアカウントを乗っ取られるのを防ぐ)。
        if let Some(bound) = token.m_address()
            && bound != user.m_address()
        {
            return Err(AuthError::InvalidToken);
        }

        let plaintext = PlaintextPassword::new(password.to_string())
            .map_err(|e| AuthError::InvalidInput(e.to_string()))?;
        let phc = self.password_hasher.hash(&plaintext).await?;
        let creds = PasswordCredentials::new(phc, self.clock)
            .map_err(|e| AuthError::Internal(anyhow::anyhow!(e.to_string())))?;
        // Registered 以外(既に Active 等)は Conflict。
        user.activate(creds, self.clock)
            .map_err(|_| AuthError::Conflict)?;

        let now = self.clock.now();
        tx.begin().await.map_err(internal)?;
        let consumed = self
            .one_time_token_repo
            .consume_in(&mut tx, ott_id, now)
            .await?;
        if consumed == 0 {
            tx.commit().await.map_err(internal)?;
            return Err(AuthError::InvalidToken);
        }
        self.user_repo.update_in(&mut tx, &user).await?;
        tx.commit().await.map_err(internal)?;
        Ok(())
    }

    /// ログイン。検証成功でセッションを開設し，アクセス+リフレッシュを発行する。
    /// 全失敗分岐でダミー検証を回し，応答時間で存在・状態を漏らさない。
    pub async fn login(
        &self,
        mut tx: Tx,
        m_address: &EmailAddress,
        password: &str,
    ) -> Result<IssuedTokens, AuthError> {
        let user_opt = self
            .user_repo
            .find_by_m_address(m_address)
            .await
            .map_err(internal)?;
        // 照合対象 PHC を決める。Active 以外/不在はダミー PHC に対して検証して時間を平準化する。
        let (phc, active_user): (String, Option<User>) = match user_opt {
            Some(u) if matches!(u.status(), UserStatus::Active { .. }) => (
                u.password_phc().expect("active has phc").to_string(),
                Some(u),
            ),
            _ => (self.dummy_phc.clone(), None),
        };
        let outcome = self.password_hasher.verify(password, &phc).await?;

        let Some(user) = active_user else {
            return Err(AuthError::InvalidCredentials);
        };
        // 旧パラメータのハッシュは透過昇格する(下のトランザクション内で楽観的に永続化)。
        let rehash_to: Option<String> = match outcome {
            VerifyOutcome::Mismatch => return Err(AuthError::InvalidCredentials),
            VerifyOutcome::Verified => None,
            VerifyOutcome::VerifiedNeedsRehash { new_phc } => Some(new_phc),
        };
        let user_id = user.id();
        let now = self.clock.now();

        // LRU: 上限到達なら最古の Active ファミリを退避。
        let actives = self
            .session_repo
            .find_active_sessions_by_user(user_id)
            .await
            .map_err(internal)?;

        let secret = self.secret_generator.generate_secret();
        let secret_hash = self.secret_generator.hash_secret(&secret);
        let session_id = SessionId::new(self.secret_generator.new_id());
        let token_id = TokenId::new(self.secret_generator.new_id());
        let (session, token) = Session::start(
            session_id,
            token_id,
            user_id,
            secret_hash,
            self.config.session_absolute_ttl,
            self.config.session_idle_ttl,
            self.clock,
        );

        tx.begin().await.map_err(internal)?;
        if actives.len() >= self.config.max_sessions_per_user
            && let Some(oldest) = actives.iter().min_by_key(|s| *s.created_at())
        {
            self.session_repo
                .revoke_family_in(&mut tx, oldest.id(), RevocationReason::EvictedLru, now)
                .await?;
        }
        if let Some(new_phc) = &rehash_to {
            // 並行パスワード変更を巻き戻さないため楽観的に rehash する
            // (password_phc が検証時のままの Active ユーザにだけ適用。不一致なら no-op)。
            self.user_repo
                .rehash_password_in(&mut tx, user_id, &phc, new_phc)
                .await?;
        }
        self.session_repo
            .insert_session_in(&mut tx, &session, &token)
            .await?;
        tx.commit().await.map_err(internal)?;

        self.issue_for(
            user_id,
            session_id,
            token_id,
            secret,
            *session.absolute_expires_at(),
        )
    }

    /// リフレッシュ。回転 + reuse(盗難)検知。
    pub async fn refresh(
        &self,
        mut tx: Tx,
        refresh_token: &str,
    ) -> Result<IssuedTokens, AuthError> {
        let (token_id, secret) = parse_opaque(refresh_token).ok_or(AuthError::InvalidToken)?;
        let token_id = TokenId::new(token_id);
        let token = self
            .session_repo
            .find_token_by_id(token_id)
            .await
            .map_err(internal)?
            .ok_or(AuthError::InvalidToken)?;
        // 秘密不一致 → 401。ファミリは触らない(token_id を当てる DoS を防ぐ)。
        if !self
            .secret_generator
            .verify_secret(secret, token.secret_hash())
        {
            return Err(AuthError::InvalidToken);
        }
        let session = self
            .session_repo
            .find_session_by_id(token.session_id())
            .await
            .map_err(internal)?
            .ok_or(AuthError::InvalidToken)?;
        let now = self.clock.now();

        if !session.is_active() {
            // 既に失効済み(過去の reuse 等)。再通知しない。
            return Err(AuthError::InvalidToken);
        }
        if now >= *session.absolute_expires_at() {
            self.revoke_family(&mut tx, session.id(), RevocationReason::Expired, now)
                .await?;
            return Err(AuthError::InvalidToken);
        }

        match token.status() {
            TokenStatus::Issued => {
                if now >= *token.idle_expires_at() {
                    self.revoke_family(&mut tx, session.id(), RevocationReason::Expired, now)
                        .await?;
                    return Err(AuthError::InvalidToken);
                }
                // 回転。
                let new_secret = self.secret_generator.generate_secret();
                let new_hash = self.secret_generator.hash_secret(&new_secret);
                let new_token_id = TokenId::new(self.secret_generator.new_id());
                let new_token = token.rotate(
                    new_token_id,
                    new_hash,
                    self.config.session_idle_ttl,
                    *session.absolute_expires_at(),
                    self.clock,
                );
                tx.begin().await.map_err(internal)?;
                let rows = self
                    .session_repo
                    .rotate_in(&mut tx, token.id(), &new_token)
                    .await?;
                if rows == 0 {
                    // 並行回転 = reuse 扱い(grace=0)。
                    self.session_repo
                        .revoke_family_in(
                            &mut tx,
                            session.id(),
                            RevocationReason::ReuseDetected,
                            now,
                        )
                        .await?;
                    tx.commit().await.map_err(internal)?;
                    return Err(AuthError::ReuseDetected);
                }
                tx.commit().await.map_err(internal)?;
                self.issue_for(
                    session.user_id(),
                    session.id(),
                    new_token_id,
                    new_secret,
                    *session.absolute_expires_at(),
                )
            }
            // 旧 secret の再提示 かつ secret 一致 = 盗難。ファミリを失効。
            TokenStatus::Rotated | TokenStatus::Revoked => {
                self.revoke_family(&mut tx, session.id(), RevocationReason::ReuseDetected, now)
                    .await?;
                Err(AuthError::ReuseDetected)
            }
        }
    }

    /// パスワード変更(本人)。旧パスワード照合 → 新パスワード設定 → 全セッション失効。
    pub async fn change_password(
        &self,
        mut tx: Tx,
        user_id: UserId,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), AuthError> {
        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(internal)?
            .ok_or(AuthError::Unauthorized)?;
        let phc = user
            .password_phc()
            .ok_or(AuthError::InvalidCredentials)?
            .to_string();
        match self.password_hasher.verify(old_password, &phc).await? {
            VerifyOutcome::Verified | VerifyOutcome::VerifiedNeedsRehash { .. } => {}
            VerifyOutcome::Mismatch => return Err(AuthError::InvalidCredentials),
        }
        let plaintext = PlaintextPassword::new(new_password.to_string())
            .map_err(|e| AuthError::InvalidInput(e.to_string()))?;
        let new_phc = self.password_hasher.hash(&plaintext).await?;
        user.change_password(new_phc, self.clock)
            .map_err(|_| AuthError::InvalidCredentials)?;

        let now = self.clock.now();
        tx.begin().await.map_err(internal)?;
        self.user_repo.update_in(&mut tx, &user).await?;
        self.session_repo
            .revoke_all_for_user_in(&mut tx, user_id, RevocationReason::PasswordChanged, now)
            .await?;
        tx.commit().await.map_err(internal)?;
        Ok(())
    }

    /// パスワードリセット要求。常に成功(202)させるため，存在しなければ `Ok(None)`。
    /// `Some` のとき HTTP 層がリクエストパス外でメールを送る。
    pub async fn request_password_reset(
        &self,
        mut tx: Tx,
        m_address: &EmailAddress,
    ) -> Result<Option<PasswordResetIssued>, AuthError> {
        let Some(user) = self
            .user_repo
            .find_by_m_address(m_address)
            .await
            .map_err(internal)?
        else {
            return Ok(None);
        };
        if !matches!(user.status(), UserStatus::Active { .. }) {
            return Ok(None);
        }
        let user_id = user.id();
        let secret = self.secret_generator.generate_secret();
        let token_hash = self.secret_generator.hash_secret(&secret);
        let ott_id = OneTimeTokenId::new(self.secret_generator.new_id());
        let token = OneTimeToken::issue(
            ott_id,
            user_id,
            OneTimeTokenPurpose::PasswordReset,
            token_hash,
            Some(user.m_address().clone()),
            self.config.reset_ttl,
            self.clock,
        );
        let now = self.clock.now();
        tx.begin().await.map_err(internal)?;
        self.one_time_token_repo
            .invalidate_existing_for_in(&mut tx, user_id, OneTimeTokenPurpose::PasswordReset, now)
            .await?;
        self.one_time_token_repo.insert_in(&mut tx, &token).await?;
        tx.commit().await.map_err(internal)?;

        Ok(Some(PasswordResetIssued {
            user_id,
            m_address: user.m_address().clone(),
            raw_token: format!("{ott_id}.{secret}"),
            expires_at: *token.expires_at(),
        }))
    }

    /// 有効化(activation)トークンを発行する。ユーザー作成時・m アドレス変更時に
    /// 呼び、初回ログイン用の不透明トークン `"{id}.{secret}"` を返す。
    /// 既存の未消費 activation トークンは無効化し、現在の m_address に束縛する。
    pub async fn issue_activation_token(
        &self,
        mut tx: Tx,
        user_id: UserId,
    ) -> Result<String, AuthError> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(internal)?
            .ok_or_else(|| AuthError::InvalidInput("user not found".to_string()))?;
        let secret = self.secret_generator.generate_secret();
        let token_hash = self.secret_generator.hash_secret(&secret);
        let ott_id = OneTimeTokenId::new(self.secret_generator.new_id());
        let token = OneTimeToken::issue(
            ott_id,
            user_id,
            OneTimeTokenPurpose::Activation,
            token_hash,
            Some(user.m_address().clone()),
            self.config.activation_ttl,
            self.clock,
        );
        let now = self.clock.now();
        tx.begin().await.map_err(internal)?;
        self.one_time_token_repo
            .invalidate_existing_for_in(&mut tx, user_id, OneTimeTokenPurpose::Activation, now)
            .await?;
        self.one_time_token_repo.insert_in(&mut tx, &token).await?;
        tx.commit().await.map_err(internal)?;
        Ok(format!("{ott_id}.{secret}"))
    }

    /// パスワードリセット確定。トークン検証 → 新パスワード設定 → 全セッション失効。
    pub async fn confirm_password_reset(
        &self,
        mut tx: Tx,
        reset_token: &str,
        new_password: &str,
    ) -> Result<(), AuthError> {
        let (ott_id, secret) = parse_opaque(reset_token).ok_or(AuthError::InvalidToken)?;
        let ott_id = OneTimeTokenId::new(ott_id);
        let token = self
            .one_time_token_repo
            .find_by_id(ott_id)
            .await
            .map_err(internal)?
            .ok_or(AuthError::InvalidToken)?;
        if !self
            .secret_generator
            .verify_secret(secret, token.token_hash())
        {
            return Err(AuthError::InvalidToken);
        }
        if token.purpose() != OneTimeTokenPurpose::PasswordReset || !token.is_consumable(self.clock)
        {
            return Err(AuthError::InvalidToken);
        }
        let mut user = self
            .user_repo
            .find_by_id(token.user_id())
            .await
            .map_err(internal)?
            .ok_or(AuthError::InvalidToken)?;
        // 宛先束縛の検証(email 変更後の旧宛先トークンを拒否)。
        if let Some(bound) = token.m_address()
            && bound != user.m_address()
        {
            return Err(AuthError::InvalidToken);
        }
        let user_id = user.id();
        let plaintext = PlaintextPassword::new(new_password.to_string())
            .map_err(|e| AuthError::InvalidInput(e.to_string()))?;
        let new_phc = self.password_hasher.hash(&plaintext).await?;
        user.change_password(new_phc, self.clock)
            .map_err(|_| AuthError::InvalidToken)?;

        let now = self.clock.now();
        tx.begin().await.map_err(internal)?;
        let consumed = self
            .one_time_token_repo
            .consume_in(&mut tx, ott_id, now)
            .await?;
        if consumed == 0 {
            tx.commit().await.map_err(internal)?;
            return Err(AuthError::InvalidToken);
        }
        self.user_repo.update_in(&mut tx, &user).await?;
        self.session_repo
            .revoke_all_for_user_in(&mut tx, user_id, RevocationReason::PasswordChanged, now)
            .await?;
        tx.commit().await.map_err(internal)?;
        Ok(())
    }

    /// ログアウト(当該ファミリのみ失効)。冪等。
    pub async fn logout(&self, mut tx: Tx, refresh_token: &str) -> Result<(), AuthError> {
        let Some((token_id, secret)) = parse_opaque(refresh_token) else {
            return Ok(());
        };
        let token_id = TokenId::new(token_id);
        let Some(token) = self
            .session_repo
            .find_token_by_id(token_id)
            .await
            .map_err(internal)?
        else {
            return Ok(());
        };
        // 秘密が一致しなければ何もしない(冪等・他人のファミリを壊さない)。
        if !self
            .secret_generator
            .verify_secret(secret, token.secret_hash())
        {
            return Ok(());
        }
        let now = self.clock.now();
        self.revoke_family(&mut tx, token.session_id(), RevocationReason::Logout, now)
            .await?;
        Ok(())
    }

    /// 全デバイスからのログアウト(当該ユーザーの全セッション失効)。
    pub async fn logout_all(&self, mut tx: Tx, user_id: UserId) -> Result<(), AuthError> {
        let now = self.clock.now();
        tx.begin().await.map_err(internal)?;
        self.session_repo
            .revoke_all_for_user_in(&mut tx, user_id, RevocationReason::LogoutAll, now)
            .await?;
        tx.commit().await.map_err(internal)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_id::UserId;
    use crate::infra::memory::access_token_issuer_impl::MemoryAccessTokenIssuer;
    use crate::infra::memory::one_time_token_repo_impl::MemoryOneTimeTokenRepo;
    use crate::infra::memory::password_hasher_impl::MemoryPasswordHasher;
    use crate::infra::memory::secret_generator_impl::MemorySecretGenerator;
    use crate::infra::memory::session_repo_impl::MemorySessionRepo;
    use crate::infra::memory::transaction_impl::MemoryTransaction;
    use crate::infra::memory::user_repo_impl::MemoryUserRepo;
    use chrono::TimeZone;

    struct TestClock {
        now: DateTime<Utc>,
    }
    impl Clock for TestClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    const PW: &str = "correct horse battery staple";
    const NEW_PW: &str = "another long password 12345";

    fn config() -> AuthConfig {
        AuthConfig {
            access_ttl: Duration::minutes(5),
            session_absolute_ttl: Duration::days(180),
            session_idle_ttl: Duration::days(14),
            max_sessions_per_user: 10,
            activation_ttl: Duration::hours(48),
            reset_ttl: Duration::minutes(30),
        }
    }

    struct Fixture {
        user_repo: MemoryUserRepo,
        ott_repo: MemoryOneTimeTokenRepo,
        session_repo: MemorySessionRepo,
        hasher: MemoryPasswordHasher,
        secret_gen: MemorySecretGenerator,
        issuer: MemoryAccessTokenIssuer,
        clock: TestClock,
    }

    impl Fixture {
        fn new(now: DateTime<Utc>) -> Self {
            Self {
                user_repo: MemoryUserRepo::new(),
                ott_repo: MemoryOneTimeTokenRepo::new(),
                session_repo: MemorySessionRepo::new(),
                hasher: MemoryPasswordHasher::new(),
                secret_gen: MemorySecretGenerator::new(),
                issuer: MemoryAccessTokenIssuer::new(),
                clock: TestClock { now },
            }
        }

        fn app(&self) -> AuthApp<'_, MemoryTransaction, TestClock> {
            AuthApp::new(
                &self.user_repo,
                &self.ott_repo,
                &self.session_repo,
                &self.hasher,
                &self.secret_gen,
                &self.issuer,
                &self.clock,
                config(),
                "$fake$__no_such_password__".to_string(),
            )
        }
    }

    fn email() -> EmailAddress {
        EmailAddress::new("u@example.com".to_string()).unwrap()
    }

    async fn seed_active_user(fx: &Fixture) -> UserId {
        let id = UserId::new(Uuid::new_v4());
        let mut user = User::register(id, "U".to_string(), email(), &fx.clock).unwrap();
        let creds = PasswordCredentials::new(format!("$fake${PW}"), &fx.clock).unwrap();
        user.activate(creds, &fx.clock).unwrap();
        fx.user_repo.insert(&user).await.unwrap();
        id
    }

    #[tokio::test]
    async fn login_success_creates_session() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let user_id = seed_active_user(&fx).await;
        let tokens = fx
            .app()
            .login(MemoryTransaction::new(), &email(), PW)
            .await
            .unwrap();
        assert!(tokens.refresh_token.contains('.'));
        let actives = fx
            .session_repo
            .find_active_sessions_by_user(user_id)
            .await
            .unwrap();
        assert_eq!(actives.len(), 1);
    }

    #[tokio::test]
    async fn login_wrong_password_fails() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        seed_active_user(&fx).await;
        let err = fx
            .app()
            .login(
                MemoryTransaction::new(),
                &email(),
                "wrong wrong wrong wrong",
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AuthError::InvalidCredentials));
    }

    #[tokio::test]
    async fn login_unknown_user_fails() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let err = fx
            .app()
            .login(MemoryTransaction::new(), &email(), PW)
            .await
            .unwrap_err();
        assert!(matches!(err, AuthError::InvalidCredentials));
    }

    #[tokio::test]
    async fn refresh_rotates_and_reuse_revokes_family() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let user_id = seed_active_user(&fx).await;
        let app = fx.app();
        let t1 = app
            .login(MemoryTransaction::new(), &email(), PW)
            .await
            .unwrap();

        // 正常な回転。新しいリフレッシュトークンが返り，旧とは異なる。
        let t2 = app
            .refresh(MemoryTransaction::new(), &t1.refresh_token)
            .await
            .unwrap();
        assert_ne!(t1.refresh_token, t2.refresh_token);

        // 旧リフレッシュトークンの再提示 = reuse 検知 → ファミリ失効。
        let err = app
            .refresh(MemoryTransaction::new(), &t1.refresh_token)
            .await
            .unwrap_err();
        assert!(matches!(err, AuthError::ReuseDetected));
        let actives = fx
            .session_repo
            .find_active_sessions_by_user(user_id)
            .await
            .unwrap();
        assert_eq!(actives.len(), 0);

        // 失効後は最新トークンでもリフレッシュ不可。
        let err2 = app
            .refresh(MemoryTransaction::new(), &t2.refresh_token)
            .await
            .unwrap_err();
        assert!(matches!(err2, AuthError::InvalidToken));
    }

    #[tokio::test]
    async fn refresh_with_garbage_does_not_touch_family() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let user_id = seed_active_user(&fx).await;
        let app = fx.app();
        let t1 = app
            .login(MemoryTransaction::new(), &email(), PW)
            .await
            .unwrap();
        // 既存 token_id だが secret 不一致 → 401 かつファミリは生存。
        let token_id = t1.refresh_token.split_once('.').unwrap().0;
        let forged = format!("{token_id}.not-the-real-secret");
        let err = app
            .refresh(MemoryTransaction::new(), &forged)
            .await
            .unwrap_err();
        assert!(matches!(err, AuthError::InvalidToken));
        assert_eq!(
            fx.session_repo
                .find_active_sessions_by_user(user_id)
                .await
                .unwrap()
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn logout_revokes_family() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let user_id = seed_active_user(&fx).await;
        let app = fx.app();
        let t1 = app
            .login(MemoryTransaction::new(), &email(), PW)
            .await
            .unwrap();
        app.logout(MemoryTransaction::new(), &t1.refresh_token)
            .await
            .unwrap();
        assert_eq!(
            fx.session_repo
                .find_active_sessions_by_user(user_id)
                .await
                .unwrap()
                .len(),
            0
        );
        // 冪等。
        app.logout(MemoryTransaction::new(), &t1.refresh_token)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn change_password_revokes_sessions_and_sets_new() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let user_id = seed_active_user(&fx).await;
        let app = fx.app();
        app.login(MemoryTransaction::new(), &email(), PW)
            .await
            .unwrap();

        app.change_password(MemoryTransaction::new(), user_id, PW, NEW_PW)
            .await
            .unwrap();

        // 既存セッションは失効。
        assert_eq!(
            fx.session_repo
                .find_active_sessions_by_user(user_id)
                .await
                .unwrap()
                .len(),
            0
        );
        // 新パスワードでログインでき，旧パスワードは不可。
        assert!(
            app.login(MemoryTransaction::new(), &email(), NEW_PW)
                .await
                .is_ok()
        );
        assert!(matches!(
            app.login(MemoryTransaction::new(), &email(), PW)
                .await
                .unwrap_err(),
            AuthError::InvalidCredentials
        ));
    }

    #[tokio::test]
    async fn activate_sets_password_and_is_single_use() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        // Registered ユーザーを用意。
        let id = UserId::new(Uuid::new_v4());
        let user = User::register(id, "U".to_string(), email(), &fx.clock).unwrap();
        fx.user_repo.insert(&user).await.unwrap();

        // activation トークンを発行(secret/hash は secret_gen で生成)。
        let secret = fx.secret_gen.generate_secret();
        let token_hash = fx.secret_gen.hash_secret(&secret);
        let ott_id = OneTimeTokenId::new(fx.secret_gen.new_id());
        let token = OneTimeToken::issue(
            ott_id,
            id,
            OneTimeTokenPurpose::Activation,
            token_hash,
            Some(email()),
            Duration::hours(48),
            &fx.clock,
        );
        {
            let mut tx = MemoryTransaction::new();
            fx.ott_repo.insert_in(&mut tx, &token).await.unwrap();
        }
        let token_str = format!("{ott_id}.{secret}");

        let app = fx.app();
        app.activate(MemoryTransaction::new(), &token_str, PW)
            .await
            .unwrap();
        // 有効化後はログインできる。
        assert!(
            app.login(MemoryTransaction::new(), &email(), PW)
                .await
                .is_ok()
        );
        // トークンは単一使用 → 2 回目は失敗。
        assert!(matches!(
            app.activate(MemoryTransaction::new(), &token_str, PW)
                .await
                .unwrap_err(),
            AuthError::InvalidToken
        ));
    }

    #[tokio::test]
    async fn activate_rejects_token_bound_to_stale_address() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let id = UserId::new(Uuid::new_v4());
        let addr_a = EmailAddress::new("a@example.com".to_string()).unwrap();
        let mut user = User::register(id, "U".to_string(), addr_a.clone(), &fx.clock).unwrap();
        fx.user_repo.insert(&user).await.unwrap();

        // activation トークンは旧アドレス A に束縛して発行。
        let secret = fx.secret_gen.generate_secret();
        let token_hash = fx.secret_gen.hash_secret(&secret);
        let ott_id = OneTimeTokenId::new(fx.secret_gen.new_id());
        let token = OneTimeToken::issue(
            ott_id,
            id,
            OneTimeTokenPurpose::Activation,
            token_hash,
            Some(addr_a),
            Duration::hours(48),
            &fx.clock,
        );
        {
            let mut tx = MemoryTransaction::new();
            fx.ott_repo.insert_in(&mut tx, &token).await.unwrap();
        }

        // 管理者が email を B に変更。
        let addr_b = EmailAddress::new("b@example.com".to_string()).unwrap();
        user.change_m_address(addr_b, &fx.clock);
        fx.user_repo.update(&user).await.unwrap();

        // 旧 A 宛トークンでの有効化は拒否される。
        let token_str = format!("{ott_id}.{secret}");
        assert!(matches!(
            fx.app()
                .activate(MemoryTransaction::new(), &token_str, PW)
                .await
                .unwrap_err(),
            AuthError::InvalidToken
        ));
    }

    #[tokio::test]
    async fn password_reset_flow() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let user_id = seed_active_user(&fx).await;
        let app = fx.app();
        app.login(MemoryTransaction::new(), &email(), PW)
            .await
            .unwrap();

        let issued = app
            .request_password_reset(MemoryTransaction::new(), &email())
            .await
            .unwrap()
            .expect("active user should get a reset token");
        app.confirm_password_reset(MemoryTransaction::new(), &issued.raw_token, NEW_PW)
            .await
            .unwrap();

        // リセットで全セッション失効 + 新パスワードでログイン可。
        assert_eq!(
            fx.session_repo
                .find_active_sessions_by_user(user_id)
                .await
                .unwrap()
                .len(),
            0
        );
        assert!(
            app.login(MemoryTransaction::new(), &email(), NEW_PW)
                .await
                .is_ok()
        );
        // 使用済みリセットトークンは再利用不可。
        assert!(matches!(
            app.confirm_password_reset(MemoryTransaction::new(), &issued.raw_token, NEW_PW)
                .await
                .unwrap_err(),
            AuthError::InvalidToken
        ));
    }

    #[tokio::test]
    async fn request_reset_for_unknown_is_none() {
        let fx = Fixture::new(Utc.with_ymd_and_hms(2026, 6, 1, 0, 0, 0).unwrap());
        let out = fx
            .app()
            .request_password_reset(MemoryTransaction::new(), &email())
            .await
            .unwrap();
        assert!(out.is_none());
    }
}
