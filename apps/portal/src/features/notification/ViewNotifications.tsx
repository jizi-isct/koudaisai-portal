import { decodeAccessToken } from '@koudaisai/shared-auth';
import type {
  NotificationRead,
  NotificationReadTypeApprovalRequest,
  NotificationReadTypeMarkdown,
} from '@koudaisai/shared-types';
import {
  ContentList,
  ContentRow,
  LoadingScreen,
  Modal,
} from '@koudaisai/shared-ui';
import { useCallback, useEffect, useMemo, useState } from 'react';
import Markdown from 'react-markdown';
import { api } from '@/features/api/api';
import styles from './ViewNotifications.module.css';

type NotificationWithReadState = {
  notification: NotificationRead;
  is_read: boolean;
};

const statusMapping = {
  approved: '承認',
  rejected: '却下',
  pending: 'error',
  closed: 'error',
};

const formatDate = (date: string) => {
  return new Date(date)
    .toLocaleDateString('ja-JP', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    })
    .replace(/\//g, '.');
};

const getUserIdFromAccessToken = () => {
  const accessToken = localStorage.getItem('exhibitor_access_token');
  if (!accessToken) return undefined;

  const payload = decodeAccessToken(accessToken);
  return typeof payload.sub === 'string' ? payload.sub : undefined;
};

export function ViewNotifications() {
  const [notifications, setNotifications] = useState<
    NotificationWithReadState[] | null
  >(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const userId = getUserIdFromAccessToken();

    if (!userId) {
      setNotifications([]);
      return;
    }

    (async () => {
      const { data, error } = await api.GET('/users/{user_id}/notifications', {
        params: {
          path: {
            user_id: userId,
          },
        },
      });

      if (error) {
        setError(`${error}`);
        setNotifications([]);
        return;
      }

      setNotifications(data ?? []);
    })().catch((caughtError) => {
      setError(`${caughtError}`);
      setNotifications([]);
    });
  }, []);

  const sortedNotifications = useMemo(() => {
    return [...(notifications ?? [])].sort((a, b) => {
      return (
        new Date(b.notification.created_at).getTime() -
        new Date(a.notification.created_at).getTime()
      );
    });
  }, [notifications]);

  if (!notifications) {
    return <LoadingScreen />;
  }

  if (error) {
    return (
      <div className={styles.empty}>
        <p>{error}</p>
      </div>
    );
  }

  if (sortedNotifications.length === 0) {
    return (
      <div className={styles.empty}>
        <p>実行委員会からの通知はありません</p>
      </div>
    );
  }

  return (
    <ContentList
      pagination
      pageSize={5}
      contents={sortedNotifications.map(({ notification }) => (
        <NotificationRow key={notification.id} notification={notification} />
      ))}
    />
  );
}

function NotificationRow({ notification }: { notification: NotificationRead }) {
  if ('type_markdown' in notification) {
    return (
      <MarkdownNotificationRow
        notification={notification}
        markdown={notification.type_markdown}
      />
    );
  }

  if ('type_approval_request' in notification) {
    return (
      <ApprovalRequestNotificationRow
        notification={notification}
        notificationApprovalRequest={notification.type_approval_request}
      />
    );
  }

  return null;
}

function MarkdownNotificationRow({
  notification,
  markdown,
}: {
  notification: NotificationRead;
  markdown: NotificationReadTypeMarkdown;
}) {
  const [isModalOpen, setIsModalOpen] = useState(false);

  return (
    <>
      <ContentRow
        content={{
          title: markdown.title,
          date: formatDate(notification.created_at),
          onClick: () => setIsModalOpen(true),
        }}
      />
      <Modal isOpen={isModalOpen} setOpen={setIsModalOpen}>
        <div className={styles.modalContent}>
          <h1>{markdown.title}</h1>
          <Markdown>{markdown.content}</Markdown>
        </div>
      </Modal>
    </>
  );
}

function ApprovalRequestNotificationRow({
  notification,
  notificationApprovalRequest,
}: {
  notification: NotificationRead;
  notificationApprovalRequest: NotificationReadTypeApprovalRequest;
}) {
  const [approvalStatus, setApprovalStatus] = useState<
    keyof typeof statusMapping | null
  >(null);
  const [approvalReason, setApprovalReason] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const fetchApprovalRequest = useCallback(async () => {
    const userId = getUserIdFromAccessToken();
    if (!userId) return;

    const { data } = await api.GET(
      '/users/{user_id}/approval_requests/{request_id}',
      {
        params: {
          path: {
            user_id: userId,
            request_id: notificationApprovalRequest.approval_request_id,
          },
        },
      },
    );

    if (data) {
      setApprovalStatus(data.status);
      setApprovalReason(data.approval_reason);
    }
  }, [notificationApprovalRequest.approval_request_id]);

  useEffect(() => {
    fetchApprovalRequest().catch(() => undefined);
  }, [fetchApprovalRequest]);

  const statusLabel = approvalStatus ? statusMapping[approvalStatus] : '';

  return (
    <>
      <ContentRow
        content={{
          title: `企画情報訂正申請が${statusLabel}されました。`,
          date: formatDate(notification.created_at),
          onClick: () => setIsModalOpen(true),
        }}
      />
      <Modal isOpen={isModalOpen} setOpen={setIsModalOpen}>
        <div className={styles.modalContent}>
          <h2>企画情報訂正申請の結果</h2>
          <p>企画情報訂正申請が{statusLabel}されました。</p>
          {approvalStatus === 'approved' && (
            <p>
              企画情報が完全に反映されるには最大で2日かかる可能性があります。
            </p>
          )}
          {approvalReason && (
            <>
              <h3>理由</h3>
              <p>{approvalReason}</p>
            </>
          )}
        </div>
      </Modal>
    </>
  );
}
