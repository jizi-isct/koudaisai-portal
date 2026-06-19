import { DeleteOutlined, PlusOutlined } from '@ant-design/icons';
import type { FormRead } from '@koudaisai/shared-types';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Button, Flex, message, Popconfirm, Table, Tag } from 'antd';
import type { TableProps } from 'antd';
import { useState } from 'react';
import { $api } from '@/features/api/api';

export function FormsPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <FormsTable />
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

function FormsTable() {
  const [messageApi, contextHolder] = message.useMessage();
  const { data, isLoading, refetch } = $api.useQuery('get', '/forms');
  const { mutateAsync: mutateFormDelete } = $api.useMutation(
    'delete',
    '/forms/{id}',
  );

  const handleDelete = (id: string) => async () => {
    messageApi.loading('削除中...');
    try {
      await mutateFormDelete({
        params: {
          path: {
            id,
          },
        },
      });
      messageApi.destroy();
      messageApi.success('削除しました');
    } catch (e) {
      messageApi.destroy();
      messageApi.error(`削除に失敗しました: ${String(e)}`);
      return;
    } finally {
      await refetch();
    }
  };

  const columns: TableProps<FormRead>['columns'] = [
    {
      key: 'title',
      title: 'タイトル',
      dataIndex: 'name',
      rowScope: 'row',
      render: (value, record) => (
        <a
          style={{ textDecoration: 'underline' }}
          href={`/forms/edit?form_id=${record.id}`}
        >
          {value}
        </a>
      ),
    },
    {
      key: 'type',
      title: '種類',
      render: (_value, record) => {
        if (record.type === 'external') return <Tag color="green">外部</Tag>;
        return <Tag color="blue">不明</Tag>;
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
            title="フォームを削除"
            description="あなたは本当にこのフォームを削除する気ですか！？"
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
      <Heading1 emoji="📃">フォーム管理画面</Heading1>
      <Flex gap={8} vertical>
        <Button style={{ width: 'fit-content' }} href="/forms/new">
          <PlusOutlined />
          新規作成
        </Button>
        <Table<FormRead>
          dataSource={data.map((item) => ({ ...item, key: item.id }))}
          columns={columns}
          bordered
          scroll={{ x: 'max-content' }}
        />
      </Flex>
      {contextHolder}
    </>
  );
}
