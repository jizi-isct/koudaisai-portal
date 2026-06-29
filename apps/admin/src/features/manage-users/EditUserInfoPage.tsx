import { $api } from '@/features/api/api';
import { LoadingScreen, Heading1 } from '@koudaisai/shared-ui';
import { useState, useEffect } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Form, Flex, Button, Result, Input, Modal, message } from 'antd';

type FormValues = {
  username: string;
  usermaddress: string;
};

type stateOfModal =
  | 'pendingSendUserName'
  | 'sending'
  | 'successOnSendUserName'
  | 'failedOnSendUserName'
  | 'pendingSendUserMAddress'
  | 'successOnSendUserMAddress'
  | 'failedOnSendUserMAddress';

export function EditUserInfoPage() {
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
      <EditUserInfo userId={userId} />
    </QueryClientProvider>
  );
}

function EditUserInfo({ userId }: { userId: string }) {
  const [messageApi, contextHolder] = message.useMessage();
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

  const [openNameModal, setOpenNameModal] = useState<boolean>(false);
  const [openMAddressModal, setOpenMAddressModal] = useState<boolean>(false);
  const [stateOfNameModal, setStateOfNameModal] = useState<stateOfModal>(
    'pendingSendUserName',
  );
  const [stateOfMailModal, setStateOfMailModal] = useState<stateOfModal>(
    'pendingSendUserMAddress',
  );
  const [activationUrl, setActivationUrl] = useState<string>('');

  const [isSending, setIsSending] = useState<boolean>(false);

  const { mutateAsync: mutateUpdateUserName } = $api.useMutation(
    'patch',
    '/users/{id}',
  );

  const { mutateAsync: mutateUpdateUserMAddress } = $api.useMutation(
    'post',
    '/users/{id}/m_address',
  );

  const [nameForm] = Form.useForm<FormValues>();
  const [mailForm] = Form.useForm<FormValues>();

  if (isLoadingUsers) {
    return <LoadingScreen />;
  }

  if (!userInfo) {
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

  const handleUpdateUserName = () => {
    setOpenNameModal(true);
  };

  const handleUpdateUserNameCancel = () => {
    setOpenNameModal(false);
  };

  const handleUpdateUserMAddress = () => {
    setOpenMAddressModal(true);
  };

  const handleUpdateUserMAddressCancel = () => {
    setOpenMAddressModal(false);
  };

  const newName = nameForm.getFieldValue('username');
  const newAddress = mailForm.getFieldValue('usermaddress');

  const handleSendUserName = async () => {
    setIsSending(true);
    setStateOfNameModal('sending');
    const result = await sendUserName(newName);
    if (result.ok) {
      messageApi.success('ユーザー名を更新しました');
      setStateOfNameModal('successOnSendUserName');
    } else {
      messageApi.error(
        `ユーザー名の更新に失敗しました: ${String(result.error)}`,
      );
      setStateOfNameModal('failedOnSendUserName');
    }
  };

  const handleSendUserMAddress = async () => {
    setIsSending(true);
    setStateOfMailModal('sending');
    const result = await sendUserMAddress(newAddress);
    if (result.ok) {
      messageApi.success('メールアドレスを更新しました');
      setStateOfMailModal('successOnSendUserMAddress');
    } else {
      messageApi.error(
        `メールアドレスの更新に失敗しました: ${String(result.error)}`,
      setStateOfMailModal('failedOnSendUserMAddress');
    }
  };

  const sendUserName = async (newName: string) => {
    try {
      await mutateUpdateUserName({
        params: {
          path: {
            id: userId,
          },
        },
        body: {
          name: newName,
        },
      });
      setIsSending(false);
      return { ok: true as const };
    } catch (error) {
      setIsSending(false);
      return { ok: false as const, error };
    }
  };

  const sendUserMAddress = async (newAddress: string) => {
    try {
      const response = await mutateUpdateUserMAddress({
        params: {
          path: {
            id: userId,
          },
        },
        body: {
          m_address: newAddress,
        },
      });
      if (!response.activation_token) {
      setIsSending(false);
      return {
        ok: false as const,
        error: new Error('Activation Tokenの取得に失敗しました．'),
      };
    }
      setActivationUrl(
        'https://portal.koudaisai.jp/activate?token=' +
          response.activation_token,
      );
      setIsSending(false);
      return { ok: true as const };
    } catch (error) {
      setIsSending(false);
      return { ok: false as const, error };
    }
  };

  const stateOfModalOnSendUserName = () => {
    switch (stateOfNameModal) {
      case 'pendingSendUserName':
        return {
          modalTitle: 'ユーザー名を変更',
          modalOnOk: () => {
            void handleSendUserName();
          },
          modalFooter: [
            <>
              <Button
                type="default"
                onClick={handleUpdateUserNameCancel}
                loading={isSending}
              >
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSendUserName();
                }}
                loading={isSending}
              >
                ユーザー名を変更
              </Button>
            </>,
          ],
          modalContents: (
            <>
              <p>本当に送信しますか？</p>
              <br />
              <p>新しいユーザー名: {newName}</p>
            </>
          ),
        };
      case 'sending':
        return {
          modalTitle: '送信中',
          modalOnOk: () => {
            void handleSendUserName();
          },
          modalFooter: [
            <>
              <Button
                type="default"
                onClick={handleUpdateUserNameCancel}
                loading={isSending}
              >
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSendUserName();
                }}
                loading={isSending}
              >
                ユーザー名を変更
              </Button>
            </>,
          ],
          modalContents: <p>送信中...</p>,
        };
      case 'failedOnSendUserName':
        return {
          modalTitle: 'エラーが発生しました．',
          modalOnOk: () => {
            void handleSendUserName();
          },
          modalFooter: [
            <>
              <Button
                type="default"
                onClick={handleUpdateUserNameCancel}
                loading={isSending}
              >
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSendUserName();
                }}
                loading={isSending}
              >
                再送信
              </Button>
            </>,
          ],
          modalContents: <p>ユーザー名の送信に失敗しました．</p>,
        };
      case 'successOnSendUserName':
        return {
          modalTitle: '送信完了',
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
                loading={isSending}
              >
                終了する
              </Button>
            </>,
          ],
          modalContents: <p>ユーザー名の更新が完了しました．</p>,
        };
    }
  };

  const stateOfModalOnSendUserMAddress = () => {
    switch (stateOfMailModal) {
      case 'pendingSendUserMAddress':
        return {
          modalTitle: 'メールアドレスを変更',
          modalOnOk: () => {
            void handleSendUserMAddress();
          },
          modalFooter: [
            <>
              <Button
                type="default"
                onClick={handleUpdateUserMAddressCancel}
                loading={isSending}
              >
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSendUserMAddress();
                }}
                loading={isSending}
              >
                メールアドレスを変更
              </Button>
            </>,
          ],
          modalContents: (
            <>
              <p>本当に送信しますか？</p>
              <br />
              <p>新しいメールアドレス: </p>
              <p style={{ fontWeight: 'bold' }}>{newAddress}</p>
            </>
          ),
        };
      case 'sending':
        return {
          modalTitle: '送信中',
          modalOnOk: () => {
            void handleSendUserMAddress();
          },
          modalFooter: [
            <>
              <Button
                type="default"
                onClick={handleUpdateUserMAddressCancel}
                loading={isSending}
              >
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSendUserMAddress();
                }}
                loading={isSending}
              >
                メールアドレスを変更
              </Button>
            </>,
          ],
          modalContents: <p>送信中...</p>,
        };
      case 'failedOnSendUserMAddress':
        return {
          modalTitle: 'エラーが発生しました．',
          modalOnOk: () => {
            void handleSendUserMAddress();
          },
          modalOnCancel: { handleUpdateUserMAddressCancel },
          modalFooter: [
            <>
              <Button
                type="default"
                onClick={handleUpdateUserMAddressCancel}
                loading={isSending}
              >
                キャンセル
              </Button>
              <Button
                type="primary"
                onClick={() => {
                  void handleSendUserName();
                }}
                loading={isSending}
              >
                再送信
              </Button>
            </>,
          ],
          modalContents: <p>ユーザー名の送信に失敗しました．</p>,
        };
      case 'successOnSendUserMAddress':
        return {
          modalTitle: '送信完了',
          modalOnOk: () => {
            window.location.href = `/manage-users/view?user_id=${userId}`;
          },
          modalOnCancel: {},
          modalFooter: [
            <>
              <Button
                type="primary"
                onClick={() => {
                  window.location.href = `/manage-users/view?user_id=${userId}`;
                }}
                loading={isSending}
              >
                終了する
              </Button>
            </>,
          ],
          modalContents: (
            <>
              <p>メールアドレスを更新しました．</p>
              <br />
              {userInfo.status === 'registered' ? (
                <>
                  <p>新しい有効化URL</p>
                  <p style={{ fontWeight: 'bold' }}>{activationUrl}</p>
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

  return (
    <>
      <Flex gap={16} vertical>
        <Heading1 emoji="">{userInfo.name} さんの情報を編集</Heading1>
        <Form
          form={nameForm}
          name="edit-user-name"
          labelCol={{ span: 8 }}
          wrapperCol={{ span: 16 }}
          style={{ maxWidth: 600 }}
          initialValues={{ username: userInfo.name }}
          onFinish={handleUpdateUserName}
          autoComplete="off"
        >
          <Form.Item
            label="ユーザー名"
            name="username"
            rules={[
              { required: true, message: 'ユーザー名を入力してください' },
            ]}
          >
            <Input placeholder="ユーザー名" />
          </Form.Item>
          <Form.Item label={null} style={{ marginTop: '-1rem' }}>
            <Button
              type="primary"
              htmlType="submit"
              style={{ maxWidth: '200' }}
            >
              ユーザー名を変更
            </Button>
          </Form.Item>
        </Form>
        <Form
          form={mailForm}
          name="edit-user-maddress"
          labelCol={{ span: 8 }}
          wrapperCol={{ span: 16 }}
          style={{ maxWidth: 600 }}
          initialValues={{ usermaddress: userInfo.m_address }}
          onFinish={handleUpdateUserMAddress}
          autoComplete="off"
        >
          <Form.Item
            label="メールアドレス"
            name="usermaddress"
            rules={[
              { required: true, message: 'メールアドレスを入力してください' },
            ]}
          >
            <Input placeholder="メールアドレス" />
          </Form.Item>
          <Form.Item label={null} style={{ marginTop: '-1rem' }}>
            <Button
              type="primary"
              htmlType="submit"
              style={{ maxWidth: '200' }}
            >
              メールアドレスを変更
            </Button>
          </Form.Item>
        </Form>
        <Modal
          title={stateOfModalOnSendUserName()?.modalTitle}
          open={openNameModal}
          onOk={stateOfModalOnSendUserName()?.modalOnOk}
          onCancel={handleUpdateUserNameCancel}
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
          open={openMAddressModal}
          onOk={stateOfModalOnSendUserMAddress()?.modalOnOk}
          onCancel={handleUpdateUserMAddressCancel}
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
    </>
  );
}
