import { $api } from '@/features/api/api';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { useState, useEffect } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { formatDate } from '@koudaisai/shared-utils';
import {
  Descriptions,
  Tag,
  Flex,
  Button,
  Result,
  type DescriptionsProps,
} from 'antd';

export function ViewUserInfoPage() {
  const [queryClient] = useState(() => new QueryClient());
  const [userId, setUserId] = useState<string | null>();

  useEffect(() => {
    setUserId(new URLSearchParams(window.location.search).get('user_id'));
  }, []);

  if (userId === undefined) {
    return <LoadingScreen />;
  }

  if (!userId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="ユーザーIDが指定されていません。URLに?user_id=xxxxのように指定してください。"
        extra={
          <Button href="/manage-users/" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <UserInfo userId={userId} />
    </QueryClientProvider>
  );
}

const roleNames = {
  representative: '企画責任者',
  operator: '企画実施担当者',
  first_responsible: '第1責任者',
  second_responsible: '第2責任者',
  third_responsible: '第3責任者',
  noRole: 'このユーザーはまだグループと紐づいていません',
  error: 'ユーザーの役割の取得に失敗しました',
};

function UserInfo({ userId }: { userId: string }) {
  const { data: userInfo, isLoading: isLoadingUsers } = $api.useQuery(
    'get',
    '/users/{id}',
    {
      params: {
        path: {
          id: userId,
        },
      },
    },
  );

  const { data: groupData, isLoading: isLoadingGroups } = $api.useQuery(
    'get',
    '/groups/{id}',
    {
      params: {
        path: {
          id: userInfo?.group_id ?? '',
        },
      },
    },
    { enabled: Boolean(userInfo && userInfo.group_id) },
  );

  const { data: groupMember, isLoading: isLoadingMember } = $api.useQuery(
    'get',
    '/groups/{id}/members',
    {
      params: {
        path: {
          id: userInfo?.group_id ?? '',
        },
      },
    },
    { enabled: Boolean(userInfo && userInfo.group_id) },
  );

  if (isLoadingUsers || isLoadingGroups || isLoadingMember) {
    return <LoadingScreen />;
  }

  if (userInfo === undefined) {
    return (
      <Result
        status="error"
        title="ユーザー情報の取得に失敗しました"
        subTitle="ユーザーが存在しないか、通信エラーによりユーザー情報を取得できませんでした。再読み込みしてください。"
        extra={
          <>
            <Button href="/manage-users/" type="default">
              戻る
            </Button>
            <Button
              href={`/manage-users/view?user_id=${userId}`}
              type="primary"
            >
              再読み込み
            </Button>
          </>
        }
      />
    );
  }

  const userStatus = () => {
    if (userInfo.status !== 'deactivated') {
      return userInfo.status === 'active' ? (
        <Tag color="green">有効化済み</Tag>
      ) : (
        <Tag color="blue">登録済み</Tag>
      );
    } else {
      return <Tag color="gray">無効化済み</Tag>;
    }
  };

  const groupInfo = () => {
    if (!userInfo.group_id) {
      return {
        id: 'このユーザーはまだグループと紐づいていません',
        name: 'このユーザーはまだグループと紐づいていません',
      };
    } else {
      return !groupData
        ? {
            id: 'グループ情報の取得に失敗しました',
            name: 'グループ情報の取得に失敗しました',
          }
        : {
            id: groupData.id,
            name: groupData.name,
          };
    }
  };

  const userRole = (): keyof typeof roleNames => {
    if (!userInfo.group_id) {
      return 'noRole';
    }

    if (!groupMember) {
      return 'error';
    }

    const targetUserRole = groupMember.find(
      (member) => member.user_id === userId,
    );
    return (targetUserRole?.role ?? 'error') as keyof typeof roleNames;
  };

  const userInfoData: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ユーザーID',
      children: userInfo.id,
    },
    {
      key: 'm_address',
      label: 'メールアドレス',
      children: userInfo.m_address,
    },
    {
      key: 'status',
      label: '状態',
      children: userStatus(),
    },
    {
      key: 'groupsId',
      label: '所属グループID',
      children: groupInfo().id,
    },
    {
      key: 'groupsName',
      label: '所属団体名',
      children: groupInfo().name,
    },
    {
      key: 'role',
      label: '役割',
      children: roleNames[userRole()],
    },
    {
      key: 'created_at',
      label: '作成日時',
      children: formatDate(userInfo.created_at),
    },
    {
      key: 'updated_at',
      label: '更新日時',
      children: formatDate(userInfo.updated_at),
    },
  ];

  return (
    <Flex gap={8} vertical>
      <Descriptions
        title={userInfo.name}
        column={1}
        bordered
        items={userInfoData}
      />
      <Flex gap={8} wrap>
        <Button type="default" href="/manage-users/" style={{ width: '5rem' }}>
          戻る
        </Button>
        <Button
          type="primary"
          href={`/manage-users/edit?user_id=${userId}`}
          style={{ width: '12rem' }}
        >
          ユーザー情報を編集
        </Button>
      </Flex>
    </Flex>
  );
}
