import '@mdxeditor/editor/style.css';

import {
  AdmonitionDirectiveDescriptor,
  BlockTypeSelect,
  BoldItalicUnderlineToggles,
  CodeToggle,
  CreateLink,
  DiffSourceToggleWrapper,
  InsertTable,
  InsertThematicBreak,
  ListsToggle,
  MDXEditor,
  type MDXEditorMethods,
  Separator,
  UndoRedo,
  diffSourcePlugin,
  directivesPlugin,
  headingsPlugin,
  imagePlugin,
  linkDialogPlugin,
  linkPlugin,
  listsPlugin,
  markdownShortcutPlugin,
  quotePlugin,
  tablePlugin,
  thematicBreakPlugin,
  toolbarPlugin,
} from '@mdxeditor/editor';
import type { apiComponents } from '@koudaisai/shared-types';
import {
  Button,
  Card,
  Divider,
  Flex,
  Form,
  Input,
  message,
  Popconfirm,
  Result,
  Spin,
} from 'antd';
import { useEffect, useRef, useState } from 'react';
import { $api, $events26Api } from '@/features/api/api';

type Menu = apiComponents['schemas']['GetProjectDetails200ResponseMenu'];
type MenuFormValues = {
  description: string;
  items: {
    name: string;
    price?: string;
    options: { name: string; price?: string }[];
  }[];
};

const emptyMenu: MenuFormValues = { description: '', items: [] };

function toFormValues(menu?: Menu): MenuFormValues {
  return {
    description: menu?.description ?? '',
    items:
      menu?.items.map((item) => ({
        name: item.name,
        price: item.price === undefined ? undefined : String(item.price),
        options: item.options.map((option) => ({
          name: option.name,
          price: option.price === undefined ? undefined : String(option.price),
        })),
      })) ?? [],
  };
}

function validatePrice(_: unknown, value?: string) {
  if (
    value === undefined ||
    value === '' ||
    (/^\d+(?:\.\d+)?$/.test(value) && Number.isFinite(Number(value)))
  ) {
    return Promise.resolve();
  }
  return Promise.reject(new Error('値段は0以上の半角数字で入力してください'));
}

function price(value?: string): number | undefined {
  return value === undefined || value === '' ? undefined : Number(value);
}

/** 企画本体とは別 API で管理するメニューと追加情報。 */
export function ProjectDetailsEditor({ projectId }: { projectId: string }) {
  const [form] = Form.useForm<MenuFormValues>();
  const [messageApi, contextHolder] = message.useMessage();
  const editorRef = useRef<MDXEditorMethods>(null);
  const [overlayContainer, setOverlayContainer] =
    useState<HTMLDivElement | null>(null);
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

  useEffect(() => {
    if (details === undefined && !isNotFound) return;
    form.setFieldsValue(toFormValues(details?.menu));
    const nextMarkdown = details?.additionalInfo ?? '';
    setMarkdown(nextMarkdown);
    editorRef.current?.setMarkdown(nextMarkdown);
    setEditorError(null);
  }, [details, form, isNotFound]);

  const menu = details?.menu;
  const additionalInfo = details?.additionalInfo ?? '';
  const busy =
    menuSaving || additionalSaving || menuDeleting || additionalDeleting;

  const saveMenu = async (values: MenuFormValues) => {
    setMenuSaving(true);
    try {
      const body: Menu = {
        description: values.description,
        items: (values.items ?? []).map((item) => ({
          name: item.name,
          price: price(item.price),
          options: (item.options ?? []).map((option) => ({
            name: option.name,
            price: price(option.price),
          })),
        })),
      };
      await putMenu({
        params: { path: { project_id: projectId } },
        body,
      });
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
      await refetch();
      messageApi.success('企画追加情報を削除しました');
    } catch (cause) {
      messageApi.error(String(cause));
    } finally {
      setAdditionalDeleting(false);
    }
  };

  if (isLoading) return <Spin aria-label="企画詳細情報を読み込み中" />;
  if (error && !isNotFound) {
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
      <Card title="メニュー" size="small" style={{ marginBottom: 16 }}>
        {!menu && <p>未設定</p>}
        <Form<MenuFormValues>
          form={form}
          layout="vertical"
          initialValues={emptyMenu}
          onFinish={saveMenu}
          disabled={busy}
        >
          <Form.Item
            label="メニュー全体の説明"
            name="description"
            rules={[
              {
                required: true,
                whitespace: true,
                message: '説明を入力してください',
              },
            ]}
          >
            <Input.TextArea rows={3} />
          </Form.Item>
          <Divider titlePlacement="start">商品一覧</Divider>
          <Form.List name="items">
            {(fields, { add, remove }) => (
              <Flex vertical gap={16}>
                {fields.map((field, index) => (
                  <Card
                    key={field.key}
                    title={'商品 ' + (index + 1)}
                    size="small"
                  >
                    <Form.Item
                      label="商品名"
                      name={[field.name, 'name']}
                      rules={[
                        {
                          required: true,
                          whitespace: true,
                          message: '商品名を入力してください',
                        },
                      ]}
                    >
                      <Input />
                    </Form.Item>
                    <Form.Item
                      label="値段（円・任意）"
                      name={[field.name, 'price']}
                      rules={[{ validator: validatePrice }]}
                    >
                      <Input inputMode="decimal" />
                    </Form.Item>
                    <Form.List name={[field.name, 'options']}>
                      {(
                        optionFields,
                        { add: addOption, remove: removeOption },
                      ) => (
                        <Flex vertical gap={8}>
                          {optionFields.map((optionField, optionIndex) => (
                            <Card
                              key={optionField.key}
                              title={'オプション ' + (optionIndex + 1)}
                              size="small"
                            >
                              <Form.Item
                                label="オプション名"
                                name={[optionField.name, 'name']}
                                rules={[
                                  {
                                    required: true,
                                    whitespace: true,
                                    message: 'オプション名を入力してください',
                                  },
                                ]}
                              >
                                <Input placeholder="名前" />
                              </Form.Item>
                              <Form.Item
                                label="値段（円・任意）"
                                name={[optionField.name, 'price']}
                                rules={[{ validator: validatePrice }]}
                              >
                                <Input inputMode="decimal" />
                              </Form.Item>
                              <Button
                                danger
                                onClick={() => removeOption(optionField.name)}
                              >
                                オプションを削除
                              </Button>
                            </Card>
                          ))}
                          <Button type="dashed" onClick={() => addOption()}>
                            オプションを追加
                          </Button>
                        </Flex>
                      )}
                    </Form.List>
                    <Button
                      danger
                      onClick={() => remove(field.name)}
                      style={{ marginTop: 12 }}
                    >
                      商品を削除
                    </Button>
                  </Card>
                ))}
                <Button type="dashed" onClick={() => add({ options: [] })}>
                  商品を追加
                </Button>
              </Flex>
            )}
          </Form.List>
          <Form.Item style={{ marginTop: 16, marginBottom: 0 }}>
            <Flex gap={8}>
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
                  <Button danger loading={menuDeleting}>
                    メニューを削除
                  </Button>
                </Popconfirm>
              )}
            </Flex>
          </Form.Item>
        </Form>
      </Card>
      <Card title="企画追加情報" size="small">
        {!details?.additionalInfo && <p>未設定</p>}
        <div
          ref={setOverlayContainer}
          style={{
            border: '1px solid #d9d9d9',
            borderRadius: 6,
            minHeight: 360,
          }}
        >
          <MDXEditor
            ref={editorRef}
            markdown={additionalInfo}
            onChange={(value) => {
              setMarkdown(value);
              setEditorError(null);
            }}
            onError={({ error: cause }) =>
              setEditorError('Markdownを解析できませんでした: ' + cause)
            }
            overlayContainer={overlayContainer}
            placeholder="企画の追加情報を入力してください"
            readOnly={busy}
            plugins={[
              toolbarPlugin({
                toolbarContents: () => (
                  <DiffSourceToggleWrapper>
                    <UndoRedo />
                    <BlockTypeSelect />
                    <Separator />
                    <BoldItalicUnderlineToggles />
                    <Separator />
                    <ListsToggle />
                    <Separator />
                    <CreateLink />
                    <CodeToggle />
                    <Separator />
                    <InsertTable />
                    <InsertThematicBreak />
                  </DiffSourceToggleWrapper>
                ),
              }),
              imagePlugin(),
              diffSourcePlugin({
                diffMarkdown: additionalInfo,
                viewMode: 'rich-text',
              }),
              headingsPlugin({ allowedHeadingLevels: [2, 3, 4] }),
              listsPlugin(),
              tablePlugin(),
              quotePlugin(),
              thematicBreakPlugin(),
              linkPlugin(),
              linkDialogPlugin(),
              directivesPlugin({
                directiveDescriptors: [AdmonitionDirectiveDescriptor],
              }),
              markdownShortcutPlugin(),
            ]}
          />
        </div>
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
          {details?.additionalInfo && (
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
