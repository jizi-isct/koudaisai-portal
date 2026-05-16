import {login} from "@koudaisai/shared-auth-members";
import {useCallback, useState} from "react";
import {authFetchClient} from "../api/api";
import membersLogo from "@/features/header/assets/members_logo.png";
import styles from "./LoginForm.module.css";

export function LoginForm() {
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = useCallback(async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
    const mAddress = formData.get("mAddress");
    const password = formData.get("password");

    if (typeof mAddress !== "string" || typeof password !== "string") return;

    setError(null);
    setIsSubmitting(true);

    try {
      await login(authFetchClient, mAddress, password);
      window.location.assign("/");
    } catch (caughtError) {
      setError(`${caughtError}`);
      setIsSubmitting(false);
    }
  }, []);

  return (
    <form className={styles.root} action="/login/" method="post" onSubmit={handleSubmit}>
      <div className={styles.logo}>
        <img src={membersLogo.src} alt="Koudaisai Portal Logo" width="40" height="40" />
      </div>
      <h1 className={styles.title}>ログイン</h1>
      <p className={styles.description}>ログインに必要な情報を入力してください</p>

      <label className={styles.label}>
        mアドレス
        <input
          className={styles.input}
          placeholder="メールアドレスを入力してください"
          required
          name="mAddress"
          type="email"
          autoComplete="email"
        />
      </label>

      <label className={styles.label}>
        パスワード
        <input
          className={styles.input}
          placeholder="パスワードを入力してください"
          required
          name="password"
          type="password"
          autoComplete="current-password"
        />
      </label>

      <div className={styles.links}>
        <a href="/">トップページに戻る</a>
        <a href="/reset-password/">パスワードを忘れた場合</a>
      </div>

      {error && (
        <p className={styles.error} aria-live="polite">
          {error}
        </p>
      )}

      <button className={styles.submit} type="submit" disabled={isSubmitting} aria-busy={isSubmitting}>
        <span className={styles.submitLabel}>工大祭ポータルに進む</span>
        <span className={styles.submitIcon} aria-hidden="true">
          <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
              d="M6.66667 16H25.3333M25.3333 16L16 6.66666M25.3333 16L16 25.3333"
              stroke="#253661"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </span>
      </button>
    </form>
  );
}
