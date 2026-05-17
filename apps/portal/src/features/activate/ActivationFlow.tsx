import {validatePassword} from "@koudaisai/shared-utils";
import {login} from "@koudaisai/shared-auth-members";
import {useCallback, useEffect, useRef, useState} from "react";
import type {FormEvent} from "react";
import {authFetchClient} from "../api/api";
import membersLogo from "@/features/header/assets/members_logo.png";
import {NextPhaseButton} from "@/features/plain-form/NextPhaseButton";
import formStyles from "@/features/plain-form/PlainForm.module.css";
import styles from "./ActivationFlow.module.css";

type ActivationPhase = "first" | "m_address" | "password" | "activating" | "done";

export function ActivationFlow() {
  const [token, setToken] = useState<string | null>(null);
  const [hasLoadedToken, setHasLoadedToken] = useState(false);
  const [phase, setPhase] = useState<ActivationPhase>("first");
  const [mAddress, setMAddress] = useState("");
  const [password, setPassword] = useState("");

  useEffect(() => {
    setToken(new URLSearchParams(window.location.search).get("token"));
    setHasLoadedToken(true);
  }, []);

  const handleMAddressChange = useCallback((nextMAddress: string) => {
    setMAddress(nextMAddress);
    setPhase("password");
  }, []);

  const handlePasswordChange = useCallback((nextPassword: string) => {
    setPassword(nextPassword);
    setPhase("activating");
  }, []);

  if (!hasLoadedToken) {
    return null;
  }

  if (!token) {
    return (
      <main className={formStyles.root}>
        <h1 className={formStyles.title}>クエリパラメータに不足があります</h1>
        <p className={formStyles.description}>tokenが指定されていません。URLに?token=xxxxのように指定してください。</p>
      </main>
    );
  }

  switch (phase) {
    case "first":
      return <ActivationPhaseFirst next={() => setPhase("m_address")} />;
    case "m_address":
      return <ActivationPhaseMAddress next={handleMAddressChange} />;
    case "password":
      return <ActivationPhasePassword mAddress={mAddress} next={handlePasswordChange} />;
    case "activating":
      return <ActivationPhaseActivating mAddress={mAddress} password={password} token={token} next={() => setPhase("done")} />;
    case "done":
      return <ActivationPhaseDone mAddress={mAddress} password={password} />;
  }
}

function Logo({size}: {size: "small" | "big"}) {
  return (
    <div className={size === "small" ? formStyles.logoSmall : formStyles.logoBig}>
      <img src={membersLogo.src} alt="Koudaisai Portal Logo" width={size === "small" ? 40 : 300} height={size === "small" ? 40 : 300} />
    </div>
  );
}

function ActivationPhaseFirst({next}: {next: () => void}) {
  return (
    <main className={formStyles.root}>
      <div className={styles.item}>
        <Logo size="big" />
      </div>
      <h1 className={`${formStyles.title} ${styles.item}`}>工大祭ポータルへようこそ</h1>
      <p className={`${formStyles.description} ${styles.item}`}>アカウントを有効化しましょう</p>
      <div className={styles.item}>
        <NextPhaseButton label="次の画面へ進む" onClick={next} />
      </div>
    </main>
  );
}

function ActivationPhaseMAddress({next}: {next: (mAddress: string) => void}) {
  const handleSubmit = useCallback((event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
    const mAddress = formData.get("mAddress");

    if (typeof mAddress === "string") {
      next(mAddress);
    }
  }, [next]);

  return (
    <form className={formStyles.root} action="/activate/" method="post" onSubmit={handleSubmit}>
      <div className={styles.item}>
        <Logo size="small" />
      </div>
      <h1 className={`${formStyles.title} ${styles.item}`}>工大祭ポータルへようこそ</h1>
      <p className={`${formStyles.description} ${styles.item}`}>参加申請フォームに入力したmアドレスを入力してください。</p>
      <label className={`${formStyles.field} ${styles.item}`}>
        mアドレス
        <input
          className={formStyles.input}
          placeholder="mアドレスを入力してください"
          required
          name="mAddress"
          type="email"
          autoComplete="email"
        />
      </label>
      <div className={styles.item}>
        <NextPhaseButton type="submit" label="次の画面へ進む" />
      </div>
    </form>
  );
}

function ActivationPhasePassword({mAddress, next}: {mAddress: string; next: (password: string) => void}) {
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = useCallback((event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
    const password = formData.get("password");
    const passwordConfirm = formData.get("passwordConfirm");

    if (typeof password !== "string" || typeof passwordConfirm !== "string") return;

    if (password !== passwordConfirm) {
      setError("パスワードが一致しません。");
      return;
    }

    if (!validatePassword(password)) {
      setError("パスワードは8文字以上、英大文字、英小文字、数字、記号を含む必要があります。");
      return;
    }

    next(password);
  }, [next]);

  return (
    <form className={formStyles.root} action="/activate/" method="post" onSubmit={handleSubmit}>
      <div className={styles.item}>
        <Logo size="small" />
      </div>
      <h1 className={`${formStyles.title} ${styles.item}`}>工大祭ポータルへようこそ</h1>
      <p className={`${formStyles.description} ${styles.item}`}>ログインに使用するパスワードを決めましょう</p>
      <input name="username" type="email" value={mAddress} readOnly hidden />
      <label className={`${formStyles.field} ${styles.item}`}>
        パスワード
        <input
          className={formStyles.input}
          placeholder="パスワードを入力してください"
          required
          name="password"
          type="password"
          autoComplete="new-password"
        />
      </label>
      <label className={`${formStyles.field} ${styles.item}`}>
        パスワード確認
        <input
          className={formStyles.input}
          placeholder="パスワードをもう一度入力してください"
          required
          name="passwordConfirm"
          type="password"
          autoComplete="new-password"
        />
      </label>
      {error && (
        <p className={formStyles.error} aria-live="polite">
          {error}
        </p>
      )}
      <div className={styles.item}>
        <NextPhaseButton type="submit" label="次の画面へ進む" />
      </div>
    </form>
  );
}

function ActivationPhaseActivating({mAddress, password, token, next}: {
  mAddress: string;
  password: string;
  token: string;
  next: () => void;
}) {
  const [error, setError] = useState<string | null>(null);
  const hasActivated = useRef(false);

  useEffect(() => {
    if (hasActivated.current) return;
    hasActivated.current = true;

    (async () => {
      const {response} = await authFetchClient.POST("/activate", {
        body: {
          m_address: mAddress,
          token,
          password,
        },
      });

      if (response.ok) {
        next();
        return;
      }

      switch (response.status) {
        case 401:
          setError("有効化トークンあるいはmアドレスが無効です。もう一度やり直してください。");
          break;
        case 404:
          setError("指定されたmアドレスが見つかりません。");
          break;
        case 409:
          setError("指定されたmアドレスはすでに登録されています。");
          break;
        case 429:
          setError("リクエストが多すぎます。しばらく待ってから再度お試しください。");
          break;
        default:
          setError("アカウントの有効化に失敗しました。");
          break;
      }
    })().catch((caughtError) => {
      setError(`${caughtError}`);
    });
  }, [mAddress, next, password, token]);

  if (error) {
    return (
      <main className={formStyles.root}>
        <div className={styles.item}>
          <Logo size="big" />
        </div>
        <p className={`${formStyles.error} ${styles.item}`}>アカウントの有効化に失敗しました</p>
        <p className={`${formStyles.description} ${styles.item}`}>{error}</p>
      </main>
    );
  }

  return (
    <main className={formStyles.root}>
      <div className={styles.item}>
        <Logo size="big" />
      </div>
      <p className={`${formStyles.description} ${styles.item}`}>アカウントを有効化中</p>
      <div className={`${styles.spinner} ${styles.item}`} aria-label="読み込み中" />
    </main>
  );
}

function ActivationPhaseDone({mAddress, password}: {mAddress: string; password: string}) {
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleClick = useCallback(async () => {
    setError(null);
    setIsSubmitting(true);

    try {
      await login(authFetchClient, mAddress, password);
      window.location.assign("/");
    } catch (caughtError) {
      setError(`${caughtError}`);
      setIsSubmitting(false);
    }
  }, [mAddress, password]);

  return (
    <main className={formStyles.root}>
      <div className={styles.item}>
        <Logo size="big" />
      </div>
      <p className={`${formStyles.description} ${styles.item}`}>
        アカウントは有効化されました
        <br />
        我々と一緒に工大祭を盛り上げましょう！
      </p>
      <div className={styles.item}>
        <NextPhaseButton label="工大祭ポータルを開く" onClick={handleClick} disabled={isSubmitting} aria-busy={isSubmitting} />
      </div>
      {error && (
        <p className={formStyles.error} aria-live="polite">
          {error}
        </p>
      )}
    </main>
  );
}
