"use client";

import {Modal,ContentRow} from "@koudaisai/shared-ui";
import {$apiMembers} from "@/lib/api";
import { NotificationRead,NotificationReadTypeApprovalRequest} from "@koudaisai/shared-types";
import {useState} from "react";

export type Content = {
  title: string,
  date?: string,
  author?: string,
  onClick: (() => void | Promise<void>)
}

const statusMapping = {
  "approved": "承認",
  "rejected": "却下",
  "pending": "error",
  "closed": "error",
}

type Props = {
  notification: NotificationRead,
  notificationApprovalRequest: NotificationReadTypeApprovalRequest
}

export function ContentRowNotificationTypeApprovalRequest({notification, notificationApprovalRequest}: Props) {
  const {data: approvalRequest} = $apiMembers.useQuery("get", "/users/{user_id}/approval_requests/{request_id}",
    {
      params: {
        path: {
          user_id: "me",
          request_id: notificationApprovalRequest.approval_request_id
        }
      }
    }
  )
  const [isModalOpen, setIsModalOpen] = useState(false);

  console.log(approvalRequest)
  return (
    <>
      <ContentRow
        content={
          {
            title: `企画情報訂正申請が${approvalRequest && statusMapping[approvalRequest.status]}されました。`,
            date: new Date(notification.created_at)
              .toLocaleDateString('ja-JP', {year: 'numeric', month: '2-digit', day: '2-digit'}).replace(/\//g, '.'),
            onClick: () => {
              setIsModalOpen(true)
            }
          }
        }
      />
      <Modal isOpen={isModalOpen} setOpen={setIsModalOpen}>
        <h2>企画情報訂正申請の結果</h2>
        企画情報訂正申請が{approvalRequest && statusMapping[approvalRequest.status]}されました。
        {
          approvalRequest?.status === "approved" && <>企画情報が完全に反映されるには最大で2日かかる可能性があります。</>
        }

        {
          approvalRequest?.approval_reason && <><h3>理由</h3>{approvalRequest.approval_reason}</>
        }
      </Modal>
    </>
  )
}