import type { ApprovalRequestRead } from '@koudaisai/shared-types';
import { useEffect, useState } from 'react';
import { api } from '@/features/api/api';
import type { Project } from './types';
import styles from './ViewPendingEditExhibitionInfoRequest.module.css';

type Props = {
  approvalRequest: ApprovalRequestRead;
  project: Project;
};

export function ViewPendingEditExhibitionInfoRequest({
  approvalRequest,
  project,
}: Props) {
  const iconKey = approvalRequest.icon_key;
  const [newIconUrl, setNewIconUrl] = useState<string | null>(null);

  useEffect(() => {
    if (!iconKey) {
      setNewIconUrl(null);
      return;
    }

    // 申請中のアイコンは S3 のキーでしか持たないため、presigned URL を発行して表示する。
    (async () => {
      const { data } = await api.GET('/files/download', {
        params: { query: { key: iconKey, file_name: iconKey } },
      });
      setNewIconUrl(data?.presigned_url ?? null);
    })().catch(() => setNewIconUrl(null));
  }, [iconKey]);

  return (
    <div className={styles.root}>
      <h2>企画内容紹介文</h2>
      {approvalRequest.description ? (
        <div>
          <h3>変更前</h3>
          <p className={styles.description}>{project.description}</p>
          <h3>変更後</h3>
          <p className={styles.description}>{approvalRequest.description}</p>
        </div>
      ) : (
        <div>変更なし</div>
      )}

      <h2>企画アイコン</h2>
      {iconKey ? (
        // 企画情報API(events26)にアイコンの取得口が無いため、変更前は表示できない。
        <div>
          <h3>申請中のアイコン</h3>
          {newIconUrl ? (
            <img
              src={newIconUrl}
              alt="新しい企画アイコン"
              width={128}
              height={128}
              className={styles.icon}
            />
          ) : (
            <p className={styles.description}>読み込み中...</p>
          )}
        </div>
      ) : (
        <div>変更なし</div>
      )}
    </div>
  );
}
