"use client";

import { NotificationRead } from "@koudaisai/shared-types";
import {ContentRowNotificationTypeMarkdown} from "@/components/notification/ContentRowNotificationTypeMarkdown";
import {
  ContentRowNotificationTypeApprovalRequest
} from "@/components/notification/ContentRowNotificationTypeApprovalRequest";

export type Content = {
  title: string,
  date?: string,
  author?: string,
  onClick: (() => void | Promise<void>)
}

type ContentRowNotificationProps = {
  notification: NotificationRead
}

export function ContentRowNotification({notification}: ContentRowNotificationProps) {
  if ("type_markdown" in notification) {
    return <ContentRowNotificationTypeMarkdown notification={notification} markdown={notification.type_markdown}/>
  } else if ("type_approval_request" in notification) {
    return <ContentRowNotificationTypeApprovalRequest notification={notification}
                                                      notificationApprovalRequest={notification.type_approval_request}/>
  } else {
    return <></>
  }
}