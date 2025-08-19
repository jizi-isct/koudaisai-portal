"use client";

import {ContentRow} from "@/components/generic/ContentRow";
import {NotificationRead} from "@/lib";
import {useState} from "react";
import {Modal} from "@/components/generic";
import Markdown from "react-markdown";

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
  const [isModalOpen, setIsModalOpen] = useState(false);

  return (
    <>
      <ContentRow
        content={
          {
            title: notification.title,
            date: new Date(notification.created_at)
            .toLocaleDateString('ja-JP', { year: 'numeric', month: '2-digit', day: '2-digit' }).replace(/\//g, '.'),
            onClick: () => {
              setIsModalOpen(true)
            }
          }
        }
      />
      <Modal isOpen={isModalOpen} setOpen={setIsModalOpen}>
        <h1>{notification.title}</h1>
        <Markdown>
          {notification.type_markdown.content}
        </Markdown>
      </Modal>
    </>
  )
}