import { useState, type ChangeEvent } from 'react';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { $api } from '@/features/api/api';
import type { UserRead } from '@koudaisai/shared-types';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import formatDate from './modules/formatDate';
import { Table, Tag, Flex, Input } from 'antd';
import type { TableProps } from 'antd';

export default function ManageUserPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <Heading1 emoji="">ユーザー管理画面</Heading1>
      <UserTable />
    </QueryClientProvider>
  );
}

function UserTable() {
  const { data, isLoading } = $api.useQuery('get', '/users');
  const userDataSet = data ?? [];
  const [keyWords, setKeyWords] = useState('');

  if (isLoading) {
    return <LoadingScreen />;
  }
  if (!data) {
    return <Heading1 emoji="⚠️">エラーです</Heading1>;
  }

  const handleOnChange = (e: ChangeEvent<HTMLInputElement>) => {
    setKeyWords(e.target.value);
  };

  const targetedUsers = userDataSet.filter((item) =>
    item.name.includes(keyWords),
  );

  const columns: TableProps<UserRead>['columns'] = [
    {
      key: 'title',
      title: 'ユーザー名',
      // rowScope: 'row',
      render: (_value, record) => {
        return (
          <a
            style={{ textDecoration: 'underline' }}
            href={`/kebab-case/view?user_id=${record.id}`}
          >
            {record.name}
          </a>
        );
      },
    },
    {
      key: 'm_address',
      title: 'メールアドレス',
      // rowScope: 'row',
      render: (_value, record) => {
        return <p>{record.m_address}</p>;
      },
    },
    {
      key: 'status',
      title: '状態',
      // rowScope: 'row',
      render: (_value, record) => {
        if (record.status !== 'deactivated') {
          return record.status === 'active' ? (
            <Tag color="green">有効化済み</Tag>
          ) : (
            <Tag color="blue">登録済み</Tag>
          );
        } else {
          return <Tag color="gray">無効化済み</Tag>;
        }
      },
    },
    {
      key: 'id',
      title: 'ユーザーID',
      // rowScope: 'row',
      render: (_value, record) => {
        return <p>{record.id}</p>;
      },
    },
    {
      key: 'updated_at',
      title: '更新日時',
      // rowScope: 'row',
      render: (_value, record) => {
        return formatDate(record.updated_at);
      },
    },
  ];

  return (
    <Flex gap={8} vertical>
      <Input
        placeholder="名前を検索"
        onChange={handleOnChange}
        style={{ width: 200 }}
        value={keyWords}
      />
      <Table<UserRead>
        dataSource={targetedUsers.map((item) => ({ ...item, key: item.id }))}
        columns={columns}
        bordered
        scroll={{ x: 'max-content' }}
      />
    </Flex>
  );
}
