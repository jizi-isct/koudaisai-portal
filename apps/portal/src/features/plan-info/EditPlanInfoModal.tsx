import type { ApprovalRequestRead } from '@koudaisai/shared-types';
import { LoadingScreen, Modal } from '@koudaisai/shared-ui';
import { useCallback, useEffect, useState } from 'react';
import { api } from '@/features/api/api';
import { EditIssueForm } from './EditIssueForm';
import { EditPendingForm } from './EditPendingForm';
import type { Project } from './types';

type Props = {
  project: Project;
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
};

/**
 * 企画情報の訂正申請モーダル。
 * 審査中の申請があればその内容と取り下げ、なければ新規申請フォームを出す。
 */
export function EditPlanInfoModal({ project, isOpen, setOpen }: Props) {
  const [approvalRequests, setApprovalRequests] = useState<
    ApprovalRequestRead[] | null
  >(null);

  const fetchApprovalRequests = useCallback(async () => {
    const { data } = await api.GET('/approval-requests');
    setApprovalRequests(data ?? []);
  }, []);

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    fetchApprovalRequests().catch(() => setApprovalRequests([]));
  }, [isOpen, fetchApprovalRequests]);

  const pendingRequest = approvalRequests?.find(
    (request) =>
      request.type === 'edit_exhibition_info' && request.status === 'pending',
  );

  return (
    <Modal isOpen={isOpen} setOpen={setOpen}>
      {!approvalRequests ? (
        <LoadingScreen />
      ) : pendingRequest ? (
        <EditPendingForm
          project={project}
          approvalRequest={pendingRequest}
          refetch={fetchApprovalRequests}
        />
      ) : (
        <EditIssueForm
          initDescription={project.description}
          refetch={fetchApprovalRequests}
        />
      )}
    </Modal>
  );
}
