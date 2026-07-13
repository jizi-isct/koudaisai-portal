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

type StateOfModal =
  | 'pendingSaveUserName'
  | 'sending'
  | 'successOnSaveUserName'
  | 'failedOnSaveUserName'
  | 'pendingSaveUserEmail'
  | 'successOnSaveUserEmail'
  | 'failedOnSaveUserEmail';

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
  const [isPendSaving, setIsPendSaving] = useState<boolean>(false);
  const [isSaving, setIsSaving] = useState(false);
  const [isCopying, setIsCopying] = useState<boolean>(false);

  const [openNameModal, setOpenNameModal] = useState<boolean>(false);
  const [openEmailModal, setOpenEmailModal] = useState<boolean>(false);
  const [stateOfNameModal, setStateOfNameModal] = useState<StateOfModal>(
    'pendingSaveUserName',
  );
  const [stateOfMailModal, setStateOfMailModal] = useState<StateOfModal>(
    'pendingSaveUserEmail',
  );

  const [openDeleteGroupModal, setOpenDeleteGroupModal] =
    useState<boolean>(false);
  const [isDeletingGroup, setIsDeletingGroup] = useState<boolean>(false);

  const {
    data: userInfo,
    isLoading: isLoadingUsers,
    refetch: refetchUserInfo,
  } = $api.useQuery('get', '/users/{id}', {
    params: {
      path: {
        id: userId,
      },
    },
  });

  const { mutateAsync: updateUserName } = $api.useMutation(
    'patch',
    '/users/{id}',
  );
  const { mutateAsync: updateUserEmail } = $api.useMutation(
    'post',
    '/users/{id}/m_address',
  );
  const { mutateAsync: deleteGroup } = $api.useMutation(
    'delete',
    '/groups/{id}',
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

  const stateOfModalOnSendUserName = () => {
    switch (stateOfNameModal) {
      case 'pendingSaveUserName':
        return {
          modalTitle: 'ユーザー名を変更',
          modalOnOk: () => {
            void handleSaveUserName();
          },
          modalFooter: [
            <>
              <Button
                type="default"
                onClick={handleCloseNameModal}
                disabled={isSaving}
              >
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSaveUserName();
                }}
                loading={isSaving}
              >
                ユーザー名を変更
              </Button>
            </>,
          ],
          modalContents: (
            <>
              <p>本当に変更しますか？</p>
              <br />
              <p>新しいユーザー名: {newUserValue}</p>
            </>
          ),
        };
      case 'sending':
        return {
          modalTitle: '変更中',
          modalFooter: [
            <>
              <Button type="default" disabled={isSaving}>
                キャンセル
              </Button>
              <Button type="primary" loading={isSaving}>
                ユーザー名を変更
              </Button>
            </>,
          ],
          modalContents: <p>変更中...</p>,
        };
      case 'failedOnSaveUserName':
        return {
          modalTitle: 'エラーが発生しました．',
          modalOnOk: () => {
            void handleSaveUserName();
          },
          modalFooter: [
            <>
              <Button
                type="default"
                onClick={handleCloseNameModal}
                loading={isSaving}
              >
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSaveUserName();
                }}
                loading={isSaving}
              >
                再送信
              </Button>
            </>,
          ],
          modalContents: <p>ユーザー名の変更に失敗しました．</p>,
        };
      case 'successOnSaveUserName':
        return {
          modalTitle: '変更完了',
          modalOnOk: () => {
            window.location.href = `/manage-users/view?user_id=${userId}`;
          },
          modalFooter: [
            <>
              <Button
                type="primary"
                onClick={() => {
                  window.location.href = `/manage-users/view?user_id=${userId}`;
                }}
                loading={isSaving}
              >
                OK
              </Button>
            </>,
          ],
          modalContents: <p>ユーザー名の変更が完了しました．</p>,
        };
    }
  };

  const stateOfModalOnSendUserMAddress = () => {
    switch (stateOfMailModal) {
      case 'pendingSaveUserEmail':
        return {
          modalTitle: 'メールアドレスを変更',
          modalOnOk: () => {
            void handleSaveUserEmail();
          },
          modalFooter: [
            <>
              <Button type="default" onClick={handleCloseEmailModal}>
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSaveUserEmail();
                }}
              >
                メールアドレスを変更
              </Button>
            </>,
          ],
          modalContents: (
            <>
              <p>本当に変更しますか？</p>
              <br />
              <p>新しいメールアドレス: </p>
              <p style={{ fontWeight: 'bold' }}>{newUserValue}</p>
            </>
          ),
        };
      case 'sending':
        return {
          modalTitle: '変更中',
          modalFooter: [
            <>
              <Button type="default" disabled={isSaving}>
                キャンセル
              </Button>
              <Button type="primary" loading={isSaving}>
                メールアドレスを変更
              </Button>
            </>,
          ],
          modalContents: <p>変更中...</p>,
        };
      case 'failedOnSaveUserEmail':
        return {
          modalTitle: 'エラーが発生しました．',
          modalOnOk: () => {
            void handleSaveUserEmail();
          },
          modalFooter: [
            <>
              <Button type="default" onClick={handleCloseEmailModal}>
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSaveUserEmail();
                }}
                loading={isSaving}
              >
                再送信
              </Button>
            </>,
          ],
          modalContents: <p>メールアドレスの変更に失敗しました．</p>,
        };
      case 'successOnSaveUserEmail':
        return {
          modalTitle: '変更完了',
          modalOnOk: () => {
            window.location.href = `/manage-users/view?user_id=${userId}`;
          },
          modalFooter: [
            <>
              <Button
                type="primary"
                onClick={() => {
                  window.location.href = `/manage-users/view?user_id=${userId}`;
                }}
                loading={isSaving}
              >
                OK
              </Button>
            </>,
          ],
          modalContents: (
            <>
              <p>メールアドレスを変更しました．</p>
              <br />
              {userInfo.status === 'registered' ? (
                <>
                  <p>新しい有効化URL</p>
                  <p style={{ fontWeight: 'bold' }}>{activationUrl}</p>
                  <Button
                    size="small"
                    onClick={handleCopyActivationUrl}
                    loading={isCopying}
                  >
                    有効化URLをコピー
                  </Button>
                </>
              ) : (
                <p>
                  すでに有効化しているユーザーのため，有効化URLは表示されません．
                </p>
              )}
            </>
          ),
        };
    }
  };

  const handleCloseNameModal = () => {
    setOpenNameModal(false);
    setIsPendSaving(false);
    setStateOfNameModal('pendingSaveUserName');
  };

  const handleCloseEmailModal = () => {
    setOpenEmailModal(false);
    setIsPendSaving(false);
    setActivationUrl('');
    setStateOfMailModal('pendingSaveUserEmail');
  };

  const handleCloseDeleteGroupModal = () => {
    if (isDeletingGroup) {
      return;
    }
    setOpenDeleteGroupModal(false);
  };

  const handleDeleteGroup = async () => {
    if (!userInfo.group_id) {
      return;
    }
    setIsDeletingGroup(true);
    try {
      await deleteGroup({ params: { path: { id: userInfo.group_id } } });
      messageApi.success('参加団体を削除しました');
      setOpenDeleteGroupModal(false);
      await refetchUserInfo();
    } catch {
      messageApi.error('参加団体の削除に失敗しました');
    } finally {
      setIsDeletingGroup(false);
    }
  };

  const startEditing = (field: 'name' | 'email', value: string) => {
    setEditingField(field);
    setEditingValue(value);
  };

  const handleCancelEditing = () => {
    setEditingField(null);
    setEditingValue('');
  };

  const newUserValue = editingValue.trim();

  const handleSaveEditing = () => {
    if (!editingField || !newUserValue) {
      messageApi.error('値を入力してください');
      return;
    }

    setIsPendSaving(true);

    if (editingField === 'name') {
      setOpenNameModal(true);
    } else {
      setOpenEmailModal(true);
    }
  };

  const handleSaveUserName = async () => {
    setIsSaving(true);
    setStateOfNameModal('sending');
    const result = await saveUserName();
    if (result.ok) {
      messageApi.success('ユーザー名を更新しました');
      setStateOfNameModal('successOnSaveUserName');
    } else {
      messageApi.error(
        `ユーザー名の更新に失敗しました: ${String(result.error)}`,
      );
      setStateOfNameModal('failedOnSaveUserName');
    }
  };

  const saveUserName = async () => {
    try {
      await updateUserName({
        params: { path: { id: userId } },
        body: { name: newUserValue },
      });
      setIsSaving(false);
      return { ok: true as const };
    } catch (error) {
      setIsSaving(false);
      return { ok: false as const, error };
    }
  };

  const handleSaveUserEmail = async () => {
    setIsSaving(true);
    setStateOfMailModal('sending');
    const result = await saveUserEmail();
    if (result.ok) {
      messageApi.success('メールアドレスを更新しました');
      setStateOfMailModal('successOnSaveUserEmail');
    } else {
      messageApi.error(
        `メールアドレスの更新に失敗しました: ${String(result.error)}`,
      );
      setStateOfMailModal('failedOnSaveUserEmail');
    }
  };

  const saveUserEmail = async () => {
    try {
      const response = await updateUserEmail({
        params: {
          path: {
            id: userId,
          },
        },
        body: {
          m_address: newUserValue,
        },
      });
      if (!response.activation_token) {
        setIsSaving(false);
        return {
          ok: false as const,
          error: new Error('有効化アドレスの取得に失敗しました．'),
        };
      }
      setActivationUrl(
        ACTIVATION_BASE_URL + encodeURIComponent(response.activation_token),
      );
      setIsSaving(false);
      return { ok: true as const };
    } catch (error) {
      setIsSaving(false);
      return { ok: false as const, error };
    }
  };

  const handleCopyActivationUrl = async () => {
    try {
      setIsCopying(true);
      await navigator.clipboard.writeText(activationUrl);
      setIsCopying(false);
      messageApi.success('有効化URLをコピーしました');
    } catch (error) {
      setIsCopying(false);
      messageApi.error(`有効化URLのコピーに失敗しました: ${String(error)}`)
    }
  }

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
            onPressEnter={(event) => {
              event.preventDefault();
              event.stopPropagation();
              handleSaveEditing();
            }}
            autoFocus
            style={{ width: 'min(100%, 28rem)' }}
            status={editingValue.trim() ? undefined : 'error'}
          />
          <Button
            type="primary"
            onClick={handleSaveEditing}
            loading={isPendSaving}
          >
            保存
          </Button>
          <Button onClick={handleCancelEditing} disabled={isPendSaving}>
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
      <Flex gap={8} wrap justify="space-between">
        <Button type="default" href="/manage-users/" style={{ width: '5rem' }}>
          戻る
        </Button>
        <Button
          danger
          type="primary"
          disabled={!userInfo.group_id}
          onClick={() => setOpenDeleteGroupModal(true)}
        >
          参加団体を削除
        </Button>
      </Flex>
      <Modal
        title="参加団体を削除"
        open={openDeleteGroupModal}
        onOk={() => {
          void handleDeleteGroup();
        }}
        onCancel={handleCloseDeleteGroupModal}
        centered
        closable={false}
        maskClosable={false}
        okText="削除する"
        cancelText="キャンセル"
        okButtonProps={{ danger: true, loading: isDeletingGroup }}
        cancelButtonProps={{ disabled: isDeletingGroup }}
      >
        <p>本当にこの参加団体を削除しますか？この操作は取り消せません。</p>
      </Modal>
      <Modal
        title={stateOfModalOnSendUserName()?.modalTitle}
        open={openNameModal}
        onOk={stateOfModalOnSendUserName()?.modalOnOk}
        onCancel={handleCloseNameModal}
        centered
        footer={stateOfModalOnSendUserName()?.modalFooter}
        closable={false}
        mask={{ closable: false }}
        styles={{
          body: {
            overflowY: 'auto',
            maxHeight: '80vh',
          },
        }}
      >
        {stateOfModalOnSendUserName()?.modalContents}
      </Modal>
      <Modal
        title={stateOfModalOnSendUserMAddress()?.modalTitle}
        open={openEmailModal}
        onOk={stateOfModalOnSendUserMAddress()?.modalOnOk}
        onCancel={handleCloseEmailModal}
        centered
        footer={stateOfModalOnSendUserMAddress()?.modalFooter}
        closable={false}
        mask={{ closable: false }}
        styles={{
          body: {
            overflowY: 'auto',
            maxHeight: '80vh',
          },
        }}
      >
        {stateOfModalOnSendUserMAddress()?.modalContents}
      </Modal>
      {contextHolder}
    </Flex>
  );
}
