import { useCallback, useState } from 'react';
import { authFetchClient } from '../api/api';
import membersLogo from '@/features/header/assets/members_logo.png';
import { NextPhaseButton } from '@/features/plain-form/NextPhaseButton';
import styles from '@/features/plain-form/PlainForm.module.css';

export function ResetPasswordRequestForm() {
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = useCallback(
    async (event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault();

      const formData = new FormData(event.currentTarget);
      const mAddress = formData.get('mAddress');
      if (typeof mAddress !== 'string') return;

      setError(null);
      setIsSubmitting(true);

      try {
        const response = await authFetchClient.POST('/password/reset', {
          body: {
            m_address: mAddress,
          },
        });

        if (response.error) {
          setError(`${response.error}`);
          setIsSubmitting(false);
          return;
        }
      } catch (caughtError) {
        setError(`${caughtError}`);
        setIsSubmitting(false);
        return;
      }

      window.location.assign('/reset-password/sent/');
    },
    [],
  );

  return (
    <form
      className={styles.root}
      action="/reset-password/"
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
      <p className={styles.description}>
        あなたのmアドレスを入力してください。
        パスワードリセット用のメールを送信します。
      </p>
      <label className={styles.field}>
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
      {error && (
        <p className={styles.error} aria-live="polite">
          {error}
        </p>
      )}
      <NextPhaseButton
        type="submit"
        label="工大祭ポータルに進む"
        disabled={isSubmitting}
        aria-busy={isSubmitting}
      />
    </form>
  );
}
