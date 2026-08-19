import { Heading1 } from '@koudaisai/shared-ui';
import { useState } from 'react';
import { api } from '@/features/api/api';
import styles from './planForm.module.css';

type Props = {
  refetch: () => Promise<void>;
  initDescription: string;
  /** 申請の対象団体。1 人が複数の団体に所属しうるので申請時に明示する。 */
  groupId: string;
};

/** presigned URL を発行してアイコン画像を S3 に置き、申請に載せるキーを返す。 */
async function uploadIcon(file: File): Promise<string> {
  const { data, error } = await api.POST('/files/upload', {
    body: { file_name: file.name },
  });

  if (error || !data) {
    throw new Error('アイコン画像のアップロードに失敗しました。');
  }

  const response = await fetch(data.presigned_url, {
    method: 'PUT',
    body: file,
  });

  if (!response.ok) {
    throw new Error('アイコン画像のアップロードに失敗しました。');
  }

  return data.key;
}

export function EditIssueForm({ refetch, initDescription, groupId }: Props) {
  const [description, setDescription] = useState(initDescription);
  const [iconFile, setIconFile] = useState<File | null>(null);
  const [issueReason, setIssueReason] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async () => {
    setIsSubmitting(true);
    setError(null);

    try {
      // 変更がない項目は申請に載せない(旧実装と同じ挙動)。
      const iconKey = iconFile ? await uploadIcon(iconFile) : undefined;
      const { error: createError } = await api.POST('/approval-requests', {
        body: {
          type: 'edit_exhibition_info',
          group_id: groupId,
          description:
            description === initDescription ? undefined : description,
          icon_key: iconKey,
          issue_reason: issueReason,
        },
      });

      if (createError) {
        throw new Error(`${createError}`);
      }

      await refetch();
    } catch (caughtError) {
      setError(`${caughtError}`);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div>
      <Heading1 emoji="📝">企画情報の訂正</Heading1>
      <div className={styles.form}>
        <div className={styles.field}>
          <p className={styles.label}>企画概要</p>
          <textarea
            className={styles.textarea}
            rows={4}
            value={description}
            onChange={(event) => setDescription(event.target.value)}
          />
        </div>
        <div className={styles.field}>
          <p className={styles.label}>アイコン画像</p>
          <input
            type="file"
            accept="image/*"
            onChange={(event) => setIconFile(event.target.files?.[0] ?? null)}
          />
          <p className={styles.note}>変更しない場合は選択しないでください。</p>
        </div>
        <div className={styles.field}>
          <p className={styles.label}>訂正理由</p>
          <textarea
            className={styles.textarea}
            rows={3}
            placeholder="理由を入力してください"
            value={issueReason}
            onChange={(event) => setIssueReason(event.target.value)}
          />
        </div>
        {error && <p className={styles.error}>{error}</p>}
        <div className={styles.actions}>
          <button
            type="button"
            className={`${styles.button} ${styles.primary}`}
            disabled={isSubmitting || issueReason === ''}
            onClick={() => {
              handleSubmit().catch((caughtError) => setError(`${caughtError}`));
            }}
          >
            {isSubmitting ? '送信中...' : '企画情報の訂正を申請する'}
          </button>
        </div>
      </div>
    </div>
  );
}
