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
  Input,
  Modal,
  message,
  type DescriptionsProps,
} from 'antd';
import { ACTIVATION_BASE_URL } from 'astro:env/client';

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
  const [messageApi, contextHolder] = message.useMessage();
  const [editingField, setEditingField] = useState<'name' | 'email' | null>(
    null,
  );
  const [editingValue, setEditingValue] = useState('');
  const [activationUrl, setActivationUrl] = useState('');
  const [isSaving, setIsSaving] = useState(false);

  const {
    data: userInfo,
    isLoading: isLoadingUsers,
    refetch: refetchUserInfo,
  } = $api.useQuery(
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

  const { mutateAsync: updateUserName } = $api.useMutation(
    'patch',
    '/users/{id}',
  );
  const { mutateAsync: updateUserEmail } = $api.useMutation(
    'post',
    '/users/{id}/m_address',
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

  const startEditing = (field: 'name' | 'email', value: string) => {
    setEditingField(field);
    setEditingValue(value);
  };

  const cancelEditing = () => {
    setEditingField(null);
    setEditingValue('');
  };

  const saveEditing = async () => {
    const value = editingValue.trim();
    if (!editingField || !value) {
      messageApi.error('値を入力してください');
      return;
    }

    setIsSaving(true);
    try {
      if (editingField === 'name') {
        await updateUserName({
          params: { path: { id: userId } },
          body: { name: value },
        });
        messageApi.success('ユーザー名を更新しました');
      } else {
        const response = await updateUserEmail({
          params: { path: { id: userId } },
          body: { m_address: value },
        });
        if (userInfo.status === 'registered' && response.activation_token) {
          setActivationUrl(
            ACTIVATION_BASE_URL +
              encodeURIComponent(response.activation_token),
          );
        }
        messageApi.success('メールアドレスを更新しました');
      }
      cancelEditing();
      await refetchUserInfo();
    } catch (error) {
      messageApi.error(`更新に失敗しました: ${String(error)}`);
    } finally {
      setIsSaving(false);
    }
  };

  const editableValue = (
    field: 'name' | 'email',
    value: string,
    inputType: 'text' | 'email' = 'text',
  ) => {
    if (editingField === field) {
      return (
        <Flex gap={8} align="start" wrap>
          <Input
            type={inputType}
            value={editingValue}
            onChange={(event) => setEditingValue(event.target.value)}
            onPressEnter={() => void saveEditing()}
            autoFocus
            style={{ width: 'min(100%, 28rem)' }}
            status={editingValue.trim() ? undefined : 'error'}
          />
          <Button
            type="primary"
            onClick={() => void saveEditing()}
            loading={isSaving}
          >
            保存
          </Button>
          <Button onClick={cancelEditing} disabled={isSaving}>
            キャンセル
          </Button>
        </Flex>
      );
    }

    return (
      <Flex gap={8} align="center" justify="space-between">
        <span>{value}</span>
        <Button size="small" onClick={() => startEditing(field, value)}>
          編集
        </Button>
      </Flex>
    );
  };

  const userInfoData: DescriptionsProps['items'] = [
    {
      key: 'id',
      label: 'ユーザーID',
      children: userInfo.id,
    },
    {
      key: 'name',
      label: 'ユーザー名',
      children: editableValue('name', userInfo.name),
    },
    {
      key: 'm_address',
      label: 'メールアドレス',
      children: editableValue('email', userInfo.m_address, 'email'),
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
        title="ユーザー情報"
        column={1}
        bordered
        items={userInfoData}
      />
      <Flex gap={8} wrap>
        <Button type="default" href="/manage-users/" style={{ width: '5rem' }}>
          戻る
        </Button>
      </Flex>
      <Modal
        title="メールアドレスを更新しました"
        open={Boolean(activationUrl)}
        onOk={() => setActivationUrl('')}
        onCancel={() => setActivationUrl('')}
        cancelButtonProps={{ style: { display: 'none' } }}
      >
        <p>新しい有効化URL</p>
        <p style={{ overflowWrap: 'anywhere', fontWeight: 'bold' }}>
          {activationUrl}
        </p>
      </Modal>
      {contextHolder}
    </Flex>
  );
}
