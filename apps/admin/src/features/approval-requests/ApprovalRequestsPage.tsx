import { MoreOutlined } from '@ant-design/icons';
import type { ApprovalRequestRead } from '@koudaisai/shared-types';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Button, Flex, Table, Tag, Tooltip } from 'antd';
import type { TableProps } from 'antd';
import { useMemo, useState } from 'react';
import { $api } from '@/features/api/api';

export function ApprovalRequestsPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <ApprovalRequestsTable />
    </QueryClientProvider>
  );
}

function ApprovalRequestsTable() {
  const { data, isLoading } = $api.useQuery('get', '/approval-requests');
  const { data: users } = $api.useQuery('get', '/users');
  const userNameMap = useMemo(() => {
    if (!users) return {};

    return users.reduce<Record<string, string>>((acc, user) => {
      acc[user.id] = `${user.group_id}の${user.name}`;
      return acc;
    }, {});
  }, [users]);

  const columns: TableProps<ApprovalRequestRead>['columns'] = [
    {
      key: 'issued_by',
      title: '申請者',
      dataIndex: 'issued_by',
      rowScope: 'row',
      render: (value) => <span>{userNameMap[value] || value}</span>,
    },
    {
      key: 'type',
      title: '種類',
      render: (_value, record) => {
        if (record.type_edit_exhibition_info) {
          return <Tag color="green">企画情報更新</Tag>;
        }

        return <Tag color="red">不明</Tag>;
      },
    },
    {
      key: 'status',
      title: 'ステータス',
      dataIndex: 'status',
      rowScope: 'row',
      filters: [
        {
          text: '審査中',
          value: 'pending',
        },
        {
          text: '承認済み',
          value: 'approved',
        },
        {
          text: '却下済み',
          value: 'rejected',
        },
        {
          text: '取り下げ済み',
          value: 'closed',
        },
      ],
      onFilter: (value, record) => record.status === value,
      render: (value) => {
        switch (value) {
          case 'pending':
            return <Tag color="blue">審査中</Tag>;
          case 'approved':
            return <Tag color="green">承認済み</Tag>;
          case 'rejected':
            return <Tag color="red">却下済み</Tag>;
          case 'closed':
            return <Tag color="purple">取り下げ済み</Tag>;
          default:
            return <Tag color="grey">不明</Tag>;
        }
      },
    },
    {
      key: 'issued_at',
      title: '発行日時',
      dataIndex: 'issued_at',
      rowScope: 'row',
      render: (value) =>
        new Date(value).toLocaleString('ja-JP', {
          year: 'numeric',
          month: '2-digit',
          day: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
        }),
    },
    {
      key: 'actions',
      title: '操作',
      dataIndex: 'id',
      fixed: 'right',
      render: (value) => (
        <Tooltip title="詳細">
          <Button
            href={`/approval_requests/review?approval_request_id=${value}`}
          >
            <MoreOutlined /> 詳細
          </Button>
        </Tooltip>
      ),
    },
  ];

  if (isLoading) return <LoadingScreen />;
  if (!data) return <Heading1 emoji="⚠️">エラーです</Heading1>;

  return (
    <>
      <Heading1 emoji="📃">承認申請一覧</Heading1>
      <Flex gap={8} vertical>
        <Table<ApprovalRequestRead>
          dataSource={data.map((item) => ({ ...item, key: item.id }))}
          columns={columns}
          bordered
          scroll={{ x: 'max-content' }}
        />
      </Flex>
    </>
  );
}
