use crate::application::transaction::{Transaction, TransactionError};
use async_trait::async_trait;
use sqlx::{Sqlite, SqliteConnection, SqlitePool};

/// sqlx の SQLite トランザクションをアプリケーション層の `Transaction` ポートに適合させる実装。
///
/// `begin` で保持中のプールから接続を取得してトランザクションを開始し，
/// `commit` で確定する。リポジトリの `*_in` メソッドは `conn` 経由でこの
/// トランザクション上の接続を取得してクエリを実行する。
pub struct SqliteTransaction {
    pool: SqlitePool,
    tx: Option<sqlx::Transaction<'static, Sqlite>>,
}

impl SqliteTransaction {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool, tx: None }
    }

    /// 開始済みトランザクション上の接続を返す。未開始なら error。
    pub(crate) fn conn(&mut self) -> anyhow::Result<&mut SqliteConnection> {
        self.tx
            .as_deref_mut()
            .ok_or_else(|| anyhow::anyhow!("transaction has not been started"))
    }
}

#[async_trait]
impl Transaction for SqliteTransaction {
    async fn begin(&mut self) -> Result<(), TransactionError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionError::Failed(e.to_string()))?;
        self.tx = Some(tx);
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), TransactionError> {
        if let Some(tx) = self.tx.take() {
            tx.commit()
                .await
                .map_err(|e| TransactionError::Failed(e.to_string()))?;
        }
        Ok(())
    }
}
