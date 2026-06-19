import { validatePassword } from '@koudaisai/shared-utils';
import { useCallback, useEffect, useState } from 'react';
import { authFetchClient } from '../api/api';
import membersLogo from '@/features/header/assets/members_logo.png';
import { NextPhaseButton } from '@/features/plain-form/NextPhaseButton';
import styles from '@/features/plain-form/PlainForm.module.css';

export function ResetPasswordConfirmForm() {
  const [token, setToken] = useState<string | null>(null);
  const [hasLoadedToken, setHasLoadedToken] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    setToken(new URLSearchParams(window.location.search).get('token'));
    setHasLoadedToken(true);
  }, []);

  const handleSubmit = useCallback(
    async (event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault();

      if (!token) return;

      const formData = new FormData(event.currentTarget);
      const password = formData.get('password');
      const passwordConfirm = formData.get('passwordConfirm');

      if (typeof password !== 'string' || typeof passwordConfirm !== 'string')
        return;

      if (password !== passwordConfirm) {
        setError('パスワードが一致しません。');
        return;
      }

      if (!validatePassword(password)) {
        setError(
          'パスワードは8文字以上、英大文字、英小文字、数字、記号を含む必要があります。',
        );
        return;
      }

      setError(null);
      setIsSubmitting(true);

      try {
        const response = await authFetchClient.POST('/password/reset/confirm', {
          body: {
            token,
            new_password: password,
          },
        });

        if (response.error) {
          setError(`エラー：${response.error}`);
          setIsSubmitting(false);
          return;
        }
      } catch (caughtError) {
        setError(`エラー：${caughtError}`);
        setIsSubmitting(false);
        return;
      }

      window.location.assign('/reset-password/confirm/success/');
    },
    [token],
  );

  if (!hasLoadedToken) {
    return null;
  }

  if (!token) {
    return (
      <section className={styles.root}>
        <h1 className={styles.title}>クエリパラメータに不足があります</h1>
        <p className={styles.description}>
          tokenが指定されていません。URLに?token=xxxxのように指定してください。
        </p>
      </section>
    );
  }

  return (
    <form
      className={styles.root}
      action="/reset-password/confirm/"
      method="post"
      onSubmit={handleSubmit}
    >
      <div className={styles.logoSmall}>
        <img
          src={membersLogo.src}
          alt="Koudaisai Portal Logo"
          width="40"
          height="40"
        />
      </div>
      <h1 className={styles.title}>パスワードのリセット</h1>
      <p className={styles.description}>新しいパスワードを入力してください</p>
      <label className={styles.field}>
        パスワード
        <input
          className={styles.input}
          placeholder="パスワードを入力してください"
          required
          name="password"
          type="password"
          autoComplete="new-password"
        />
      </label>
      <label className={styles.field}>
        パスワード確認
        <input
          className={styles.input}
          placeholder="パスワードをもう一度入力してください"
          required
          name="passwordConfirm"
          type="password"
          autoComplete="new-password"
        />
      </label>
      {error && (
        <p className={styles.error} aria-live="polite">
          {error}
        </p>
      )}
      <NextPhaseButton
        type="submit"
        label="パスワードをリセット"
        disabled={isSubmitting}
        aria-busy={isSubmitting}
      />
    </form>
  );
}
