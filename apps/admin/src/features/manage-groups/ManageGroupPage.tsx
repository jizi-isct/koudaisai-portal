import { useMemo, useState, type ChangeEvent } from 'react';
import { PlusOutlined } from '@ant-design/icons';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { formatDate } from '@koudaisai/shared-utils';
import { $api } from '@/features/api/api';
import type { GroupRead } from '@koudaisai/shared-types';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Button, Table, type TableProps, Tag, Flex, Input } from 'antd';

export default function ManageGroupPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <Heading1 emoji="">参加団体管理画面</Heading1>
      <GroupTable />
    </QueryClientProvider>
  );
}

function GroupTable() {
  const { data, isLoading } = $api.useQuery('get', '/groups');
  const groupDataSet: GroupRead[] = data ?? [];
  const [filterKey, setFilterKey] = useState<string>('');
  const targetedGroups: GroupRead[] = useMemo<GroupRead[]>(
    () => groupDataSet.filter((item) => item.id.includes(filterKey)),
    [groupDataSet, filterKey],
  );

  if (isLoading) {
    return <LoadingScreen />;
  }
  if (!data) {
    return <Heading1 emoji="⚠️">エラーです</Heading1>;
  }

  const handleOnChange = (e: ChangeEvent<HTMLInputElement>) => {
    setFilterKey(e.target.value);
  };

  const typeOfGroups = (groupType: GroupRead['type']) => {
    switch (groupType) {
      case 'booth_project':
        return <Tag color="green">模擬店</Tag>;
      case 'stage_project':
        return <Tag color="yellow">ステージ</Tag>;
      case 'general_project':
        return <Tag color="blue">一般</Tag>;
      case 'lab_project':
        return <Tag color="orange">研究室</Tag>;
      case 'press':
        return <Tag color="pink">学内取材</Tag>;
      default:
        return <Tag color="gray">不明</Tag>;
    }
  };

  const columns: TableProps<GroupRead>['columns'] = [
    {
      key: 'group_id',
      title: '団体ID',
      render: (_value, record) => {
        return (
          <a
            style={{ textDecoration: 'underline' }}
            href={`/manage-groups/view?group_id=${record.id}`}
          >
            {record.id}
          </a>
        );
      },
    },
    {
      key: 'name',
      title: '団体名',
      render: (_value, record) => {
        return <p>{record.name}</p>;
      },
    },
    {
      key: 'type',
      title: '団体種別',
      render: (_value, record) => {
        return typeOfGroups(record.type);
      },
    },
    {
      key: 'updated_at',
      title: '更新日時',
      render: (_value, record) => {
        return formatDate(record.updated_at);
      },
    },
  ];

  return (
    <Flex gap={8} vertical>
      <Button style={{ width: 'fit-content' }} href="/manage-groups/add">
        <PlusOutlined />
        新規作成
      </Button>
      <Input
        placeholder="団体IDを検索"
        onChange={handleOnChange}
        style={{ width: 200 }}
        value={filterKey}
      />
      <Table<GroupRead>
        dataSource={targetedGroups.map((item) => ({ ...item, key: item.id }))}
        columns={columns}
        bordered
        scroll={{ x: 'max-content' }}
      />
    </Flex>
  );
}
