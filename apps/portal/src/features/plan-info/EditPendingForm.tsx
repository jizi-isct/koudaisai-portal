import type { ApprovalRequestRead } from '@koudaisai/shared-types';
import { Heading1 } from '@koudaisai/shared-ui';
import { useState } from 'react';
import { api } from '@/features/api/api';
import styles from './planForm.module.css';
import type { Project } from './types';
import { ViewPendingEditExhibitionInfoRequest } from './ViewPendingEditExhibitionInfoRequest';

type Props = {
  refetch: () => Promise<void>;
  project: Project;
  approvalRequest: ApprovalRequestRead;
};

export function EditPendingForm({ refetch, project, approvalRequest }: Props) {
  const [isClosing, setIsClosing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleClose = async () => {
    setIsClosing(true);
    setError(null);

    const { error: closeError } = await api.POST(
      '/approval-requests/{id}/close',
      { params: { path: { id: approvalRequest.id } } },
    );

    if (closeError) {
      setError(`${closeError}`);
      setIsClosing(false);
      return;
    }

    await refetch();
    setIsClosing(false);
  };

  return (
    <div>
      <Heading1 emoji="📝">企画情報訂正申請は現在審査中です</Heading1>

      <ViewPendingEditExhibitionInfoRequest
        approvalRequest={approvalRequest}
        project={project}
      />

      {error && <p className={styles.error}>{error}</p>}
      <div className={styles.actions}>
        <button
          type="button"
          className={`${styles.button} ${styles.primary}`}
          disabled={isClosing}
          onClick={() => {
            handleClose().catch((caughtError) => setError(`${caughtError}`));
          }}
        >
          {isClosing ? '取り下げ中...' : '企画情報訂正申請を取り下げる'}
        </button>
      </div>
    </div>
  );
}
