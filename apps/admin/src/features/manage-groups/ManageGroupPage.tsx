import { useState, type ChangeEvent } from 'react';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { formatDate } from '@koudaisai/shared-utils';
import { $api } from '@/features/api/api';
import type { UserRead } from '@koudaisai/shared-types';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Table, Tag, Flex, Input } from 'antd';
import type { TableProps } from 'antd';

export default function ManageGroupPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <Heading1 emoji="">参加団体管理画面</Heading1>
      <GroupTable />
    </QueryClientProvider>
  );
}

function GroupTable(){
    return(
        <Heading1 emoji="">実装中</Heading1>
    );
}