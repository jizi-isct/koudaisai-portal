import type { ApprovalRequestRead } from '@koudaisai/shared-types';
import { LoadingScreen, Modal } from '@koudaisai/shared-ui';
import { useCallback, useEffect, useState } from 'react';
import { api } from '@/features/api/api';
import { EditIssueForm } from './EditIssueForm';
import { EditPendingForm } from './EditPendingForm';
import type { Project } from './types';

type Props = {
  project: Project;
  /** 企画を持つ団体の ID。申請の対象団体として送り、審査中の申請の絞り込みにも使う。 */
  groupId: string;
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
};

/**
 * 企画情報の訂正申請モーダル。
 * 審査中の申請があればその内容と取り下げ、なければ新規申請フォームを出す。
 */
export function EditPlanInfoModal({
  project,
  groupId,
  isOpen,
  setOpen,
}: Props) {
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

  // 同じ団体の申請だけを見る。申請者が複数の団体に所属していると、
  // 別の団体宛ての申請まで混ざるため。
  const pendingRequest = approvalRequests?.find(
    (request) =>
      request.group_id === groupId &&
      request.type === 'edit_exhibition_info' &&
      request.status === 'pending',
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
          groupId={groupId}
          refetch={fetchApprovalRequests}
        />
      )}
    </Modal>
  );
}
