'use client'; // クライアントサイドコンポーネントとして実行するために追加

import React, {useMemo} from "react";
import { $apiMembers } from "@/lib/api";
import { Modal,LoadingScreen } from "@koudaisai/shared-ui";
import {EditIssueForm} from "@/components/plan/IssueForm";
import {EditPendingForm} from "@/components/plan/EditPendingForm";


type EditModalProps = {
  planId: string;
  initDescription: string;
  modal: boolean;
  setModal: (isOpen: boolean) => void;
};

export const EditModal = (
  {
    planId,
    initDescription,
    modal,
    setModal
  }: EditModalProps) => {
  const {data: approvalRequests, refetch} = $apiMembers.useQuery("get", "/users/{user_id}/approval_requests", {
    params: {
      path: {
        user_id: "me"
      }
    }
  })
  const pendingPlanEditRequests = useMemo(() => {
    return approvalRequests?.filter((value) =>
      value.type_edit_exhibition_info && value.status === "pending")
  }, [approvalRequests])

  if (!approvalRequests || !pendingPlanEditRequests) {
    return <LoadingScreen/>
  }

  return (
    <Modal
      isOpen={modal}
      setOpen={setModal}
    >
      {
        pendingPlanEditRequests?.length === 0 ?
          <EditIssueForm
            refetch={async () => {
              await refetch()
            }}
            initDescription={initDescription}
          /> :
          <EditPendingForm
            planId={planId}
            refetch={async () => {
              await refetch()
            }}
            approvalRequest={pendingPlanEditRequests[0]}
          />
      }
    </Modal>
  );
};