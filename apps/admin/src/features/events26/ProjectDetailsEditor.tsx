import {
  AdditionalInfoEditor,
  MenuInfoForm,
  type MenuInfo,
} from '@koudaisai-portal/shared-events26';
import type { apiComponents } from '@koudaisai/shared-types';
import {
  Alert,
  Button,
  Card,
  Divider,
  Flex,
  message,
  Popconfirm,
  Result,
  Spin,
  theme,
} from 'antd';
import { useEffect, useRef, useState, type CSSProperties } from 'react';
import { $api, $events26Api } from '@/features/api/api';

type Menu = apiComponents['schemas']['GetProjectDetails200ResponseMenu'];

/** 企画本体とは別 API で管理するメニューと追加情報。 */
export function ProjectDetailsEditor({ projectId }: { projectId: string }) {
  const {
    token: { colorBgContainer, colorBgLayout, colorTextTertiary },
  } = theme.useToken();
  const [messageApi, contextHolder] = message.useMessage();
  const menuDirty = useRef(false);
  const additionalInfoDirty = useRef(false);
  const [editorMenu, setEditorMenu] = useState<MenuInfo>();
  const [initialMarkdown, setInitialMarkdown] = useState('');
  const [markdown, setMarkdown] = useState('');
  const [editorError, setEditorError] = useState<string | null>(null);
  const [menuSaving, setMenuSaving] = useState(false);
  const [additionalSaving, setAdditionalSaving] = useState(false);
  const [menuDeleting, setMenuDeleting] = useState(false);
  const [additionalDeleting, setAdditionalDeleting] = useState(false);

  const {
    data: details,
    isLoading,
    error,
    refetch,
  } = $events26Api.useQuery(
    'get',
    '/v1/projects/{projectId}/details',
    { params: { path: { projectId } } },
    { retry: false },
  );
  // 仕様上のエラー応答は404のみ。通信エラーはErrorとして別に扱う。
  const isNotFound =
    error != null &&
    !(error instanceof Error) &&
    typeof error === 'object' &&
    'message' in error;

  const { mutateAsync: putMenu } = $api.useMutation(
    'put',
    '/events26/projects/{project_id}/menu',
  );
  const { mutateAsync: deleteMenuMutation } = $api.useMutation(
    'delete',
    '/events26/projects/{project_id}/menu',
  );
  const { mutateAsync: putAdditionalInfo } = $api.useMutation(
    'put',
    '/events26/projects/{project_id}/details/additional_info',
  );
  const { mutateAsync: deleteAdditionalInfoMutation } = $api.useMutation(
    'delete',
    '/events26/projects/{project_id}/details/additional_info',
  );

  const hasDetails = details !== undefined && !isNotFound;
  const menu = isNotFound ? undefined : details?.menu;
  const additionalInfo = isNotFound ? '' : (details?.additionalInfo ?? '');

  useEffect(() => {
    if ((!hasDetails && !isNotFound) || menuDirty.current) return;
    setEditorMenu(menu);
  }, [hasDetails, isNotFound, menu]);

  useEffect(() => {
    if ((!hasDetails && !isNotFound) || additionalInfoDirty.current) return;
    setMarkdown(additionalInfo);
    setInitialMarkdown(additionalInfo);
    setEditorError(null);
  }, [additionalInfo, hasDetails, isNotFound]);
  const busy =
    menuSaving || additionalSaving || menuDeleting || additionalDeleting;

  const saveMenu = async (values: MenuInfo) => {
    setMenuSaving(true);
    try {
      const body: Menu = {
        description: values.description,
        items: (values.items ?? []).map((item) => ({
          name: item.name,
          price: item.price ?? undefined,
          options: (item.options ?? []).map((option) => ({
            name: option.name,
            price: option.price ?? undefined,
          })),
        })),
      };
      await putMenu({
        params: { path: { project_id: projectId } },
        body,
      });
      menuDirty.current = false;
      await refetch();
      messageApi.success('メニューを保存しました');
    } catch (cause) {
      messageApi.error(String(cause));
    } finally {
      setMenuSaving(false);
    }
  };

  const deleteMenu = async () => {
    setMenuDeleting(true);
    try {
      await deleteMenuMutation({
        params: { path: { project_id: projectId } },
      });
      menuDirty.current = false;
      await refetch();
      messageApi.success('メニューを削除しました');
    } catch (cause) {
      messageApi.error(String(cause));
    } finally {
      setMenuDeleting(false);
    }
  };

  const saveAdditionalInfo = async () => {
    if (editorError) return;
    setAdditionalSaving(true);
    try {
      await putAdditionalInfo({
        params: { path: { project_id: projectId } },
        body: markdown,
      });
      additionalInfoDirty.current = false;
      await refetch();
      messageApi.success('企画追加情報を保存しました');
    } catch (cause) {
      messageApi.error(String(cause));
    } finally {
      setAdditionalSaving(false);
    }
  };

  const deleteAdditionalInfo = async () => {
    setAdditionalDeleting(true);
    try {
      await deleteAdditionalInfoMutation({
        params: { path: { project_id: projectId } },
      });
      additionalInfoDirty.current = false;
      await refetch();
      messageApi.success('企画追加情報を削除しました');
    } catch (cause) {
      messageApi.error(String(cause));
    } finally {
      setAdditionalDeleting(false);
    }
  };

  if (isLoading) return <Spin aria-label="企画詳細情報を読み込み中" />;
  if (error && !isNotFound && !hasDetails) {
    return (
      <Result
        status="error"
        title="企画詳細情報を取得できませんでした"
        subTitle={String(error)}
        extra={<Button onClick={() => refetch()}>再読み込み</Button>}
      />
    );
  }

  return (
    <>
      <Divider />
      <h2>メニュー・企画追加情報</h2>
      {error && !isNotFound && (
        <Alert
          type="error"
          showIcon
          message="企画詳細情報を再取得できませんでした"
          action={<Button onClick={() => refetch()}>再読み込み</Button>}
          style={{ marginBottom: 16 }}
        />
      )}
      <Card title="メニュー" size="small" style={{ marginBottom: 16 }}>
        {!menu && <p>未設定</p>}
        <MenuInfoForm
          menu={editorMenu}
          onSubmit={saveMenu}
          onValuesChange={() => {
            menuDirty.current = true;
          }}
          disabled={busy}
          style={
            {
              '--menu-card-border-color': colorTextTertiary,
              '--menu-item-background': colorBgLayout,
              '--menu-option-background': colorBgContainer,
              '--menu-remove-button-align': 'left',
            } as CSSProperties
          }
        >
          <Flex gap={8} style={{ marginTop: 16 }}>
            <Button type="primary" htmlType="submit" loading={menuSaving}>
              メニューを保存
            </Button>
            {menu && (
              <Popconfirm
                title="メニューを削除しますか？"
                onConfirm={deleteMenu}
                okText="削除"
                cancelText="キャンセル"
              >
                <Button danger disabled={busy} loading={menuDeleting}>
                  メニューを削除
                </Button>
              </Popconfirm>
            )}
          </Flex>
        </MenuInfoForm>
      </Card>
      <Card title="企画追加情報" size="small">
        {!additionalInfo && <p>未設定</p>}
        <AdditionalInfoEditor
          initialMarkdown={initialMarkdown}
          onChange={(value) => {
            setMarkdown(value);
            additionalInfoDirty.current = value !== additionalInfo;
            setEditorError(null);
          }}
          onError={setEditorError}
          readOnly={busy}
        />
        {editorError && (
          <p role="alert" style={{ color: '#cf1322' }}>
            {editorError}
          </p>
        )}
        <Flex gap={8} style={{ marginTop: 16 }}>
          <Button
            type="primary"
            onClick={saveAdditionalInfo}
            disabled={
              busy || markdown === additionalInfo || Boolean(editorError)
            }
            loading={additionalSaving}
          >
            企画追加情報を保存
          </Button>
          {additionalInfo && (
            <Popconfirm
              title="企画追加情報を削除しますか？"
              onConfirm={deleteAdditionalInfo}
              okText="削除"
              cancelText="キャンセル"
            >
              <Button danger disabled={busy} loading={additionalDeleting}>
                企画追加情報を削除
              </Button>
            </Popconfirm>
          )}
        </Flex>
      </Card>
      {contextHolder}
    </>
  );
}
