import { DeleteOutlined, PlusOutlined } from '@ant-design/icons';
import type { NotificationRead } from '@koudaisai/shared-types';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Button, Flex, Popconfirm, Table, Tag } from 'antd';
import type { TableProps } from 'antd';
import { useState } from 'react';
import { $api } from '@/features/api/api';

export function NotificationsPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <NotificationsTable />
    </QueryClientProvider>
  );
}

function formatDate(value: string) {
  return new Date(value).toLocaleString('ja-JP', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}

function NotificationsTable() {
  const { data, isLoading, refetch } = $api.useQuery('get', '/notifications');
  const { mutateAsync: mutateNotificationDelete } = $api.useMutation(
    'delete',
    '/notifications/{id}',
  );

  const handleDelete = (id: string) => async () => {
    await mutateNotificationDelete({
      params: {
        path: {
          id,
        },
      },
    });
    await refetch();
  };

  const columns: TableProps<NotificationRead>['columns'] = [
    {
      key: 'title',
      title: 'タイトル',
      rowScope: 'row',
      render: (_value, record) => {
        if (record.type === 'markdown') {
          return (
            <a
              style={{ textDecoration: 'underline' }}
              href={`/notifications/edit?notification_id=${record.id}`}
            >
              {record.title}
            </a>
          );
        }

        if (record.type === 'approval_request') {
          return (
            <a
              style={{ textDecoration: 'underline' }}
              href={`/approval_requests/review?approval_request_id=${record.approval_request_id}`}
            >
              承認申請結果
            </a>
          );
        }

        return <span>不明</span>;
      },
    },
    {
      key: 'type',
      title: '種類',
      render: (_value, record) => {
        if (record.type === 'markdown') return <Tag color="green">MD</Tag>;
        if (record.type === 'approval_request')
          return <Tag color="blue">承認申請結果</Tag>;
        return <Tag color="red">不明</Tag>;
      },
    },
    {
      key: 'created_at',
      title: '作成日時',
      dataIndex: 'created_at',
      rowScope: 'row',
      render: formatDate,
    },
    {
      key: 'updated_at',
      title: '更新日時',
      dataIndex: 'updated_at',
      rowScope: 'row',
      render: formatDate,
    },
    {
      key: 'created_by',
      title: '作成者',
      dataIndex: 'created_by',
      rowScope: 'row',
    },
    {
      key: 'updated_by',
      title: '更新者',
      dataIndex: 'updated_by',
      rowScope: 'row',
    },
    {
      key: 'actions',
      title: '',
      dataIndex: 'id',
      fixed: 'right',
      render: (value) => (
        <Flex gap={5}>
          <Popconfirm
            title="通知を削除"
            description="あなたは本当にこの通知を削除する気ですか！？"
            onConfirm={handleDelete(value)}
            okText="はい"
            cancelText="いいえ"
          >
            <Button danger>
              <DeleteOutlined />
            </Button>
          </Popconfirm>
        </Flex>
      ),
    },
  ];

  if (isLoading) return <LoadingScreen />;
  if (!data) return <Heading1 emoji="⚠️">エラーです</Heading1>;

  return (
    <>
      <Heading1 emoji="🔔">通知管理画面</Heading1>
      <Flex gap={8} vertical>
        <Button style={{ width: 'fit-content' }} href="/notifications/new">
          <PlusOutlined />
          新規作成
        </Button>
        <Table<NotificationRead>
          dataSource={data.map((item) => ({ ...item, key: item.id }))}
          columns={columns}
          bordered
          scroll={{ x: 'max-content' }}
        />
      </Flex>
    </>
  );
}
