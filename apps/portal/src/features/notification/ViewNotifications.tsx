import { decodeAccessToken } from '@koudaisai/shared-auth';
import type { NotificationRead } from '@koudaisai/shared-types';
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

type MarkdownNotification = Extract<NotificationRead, { type: 'markdown' }>;
type ApprovalRequestNotification = Extract<
  NotificationRead,
  { type: 'approval_request' }
>;

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
  const [notifications, setNotifications] = useState<NotificationRead[] | null>(
    null,
  );
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const userId = getUserIdFromAccessToken();

    if (!userId) {
      setNotifications([]);
      return;
    }

    (async () => {
      const { data, error } = await api.GET('/notifications');

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
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
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
      contents={sortedNotifications.map((notification) => (
        <NotificationRow key={notification.id} notification={notification} />
      ))}
    />
  );
}

function NotificationRow({ notification }: { notification: NotificationRead }) {
  if (notification.type === 'markdown') {
    return <MarkdownNotificationRow notification={notification} />;
  }

  if (notification.type === 'approval_request') {
    return <ApprovalRequestNotificationRow notification={notification} />;
  }

  return null;
}

function MarkdownNotificationRow({
  notification,
}: {
  notification: MarkdownNotification;
}) {
  const [isModalOpen, setIsModalOpen] = useState(false);

  return (
    <>
      <ContentRow
        content={{
          title: notification.title,
          date: formatDate(notification.created_at),
          onClick: () => setIsModalOpen(true),
        }}
      />
      <Modal isOpen={isModalOpen} setOpen={setIsModalOpen}>
        <div className={styles.modalContent}>
          <h1>{notification.title}</h1>
          <Markdown>{notification.content}</Markdown>
        </div>
      </Modal>
    </>
  );
}

function ApprovalRequestNotificationRow({
  notification,
}: {
  notification: ApprovalRequestNotification;
}) {
  const [approvalStatus, setApprovalStatus] = useState<
    keyof typeof statusMapping | null
  >(null);
  const [approvalReason, setApprovalReason] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const fetchApprovalRequest = useCallback(async () => {
    const { data } = await api.GET('/approval-requests/{id}', {
      params: {
        path: {
          id: notification.approval_request_id,
        },
      },
    });

    if (data) {
      setApprovalStatus(data.status);
      setApprovalReason(
        data.status === 'approved'
          ? (data.approval_reason ?? null)
          : data.status === 'rejected'
            ? (data.rejection_reason ?? null)
            : null,
      );
    }
  }, [notification.approval_request_id]);

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
