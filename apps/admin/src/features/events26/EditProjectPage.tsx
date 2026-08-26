import {
  DeleteOutlined,
  MinusCircleOutlined,
  PlusOutlined,
  UploadOutlined,
} from '@ant-design/icons';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import {
  Button,
  Card,
  Checkbox,
  Divider,
  Flex,
  Form,
  Input,
  message,
  Popconfirm,
  Result,
  Select,
  Space,
  Upload,
} from 'antd';
import { useEffect, useMemo, useState } from 'react';
import { api, $events26Api } from '@/features/api/api';
import {
  CATEGORIES,
  CATEGORY_LABEL,
  ensureOk,
  FOOD_STALL_TAG2,
  GENERAL_TAGS,
  ICON_CONTENT_TYPES,
  iconUrl,
  parseTime,
  PROJECT_TYPE_LABEL,
  putIcon,
} from './project';
import { formatTime } from './util';
import type {
  Category,
  FoodStallTag,
  GeneralTag,
  Occasion,
  PlaceId,
  Project,
  Time,
} from './project';

/** 開催予定 1 件分。時刻は CSV と同じ `HH:mm` の文字列で持つ。 */
type OccasionValues = {
  date: Time['date'];
  place?: PlaceId;
  start: string;
  end: string;
};

/** 模擬店タグ 1 件分。`drink` は `tag2` を持たない。 */
type FoodStallTagValues = {
  tag: FoodStallTag['tag'];
  tag2?: string;
};

type FormValues = {
  type: Project['type'];
  groupName: string;
  projectName: string;
  description: string;
  isChildFriendly: boolean;
  isRecommended: boolean;
  /** 全企画種別で指定できる任意項目。 */
  category?: Category;
  /** 研究室公開企画のみ。 */
  isTour?: boolean;
  /** 模擬店企画のみ。 */
  offering?: string;
  /** 模擬店企画のみ。 */
  foodStallTags?: FoodStallTagValues[];
  /** 一般企画のみ。 */
  generalTags?: GeneralTag[];
  occasions?: OccasionValues[];
};

const TIME_RULE = {
  pattern: /^([01]?\d|2[0-3]):[0-5]\d$/,
  message: 'HH:mm 形式で入力してください',
};

const PROJECT_TYPE_OPTIONS = Object.entries(PROJECT_TYPE_LABEL).map(
  ([value, { text }]) => ({ value, label: `${text}（${value}）` }),
);

const FOOD_STALL_TAG_OPTIONS: { value: FoodStallTag['tag']; label: string }[] =
  [
    { value: 'main', label: 'main（主食）' },
    { value: 'sweet', label: 'sweet（スイーツ）' },
    { value: 'drink', label: 'drink（ドリンク）' },
  ];

function toFormValues(project: Project): FormValues {
  return {
    type: project.type,
    groupName: project.groupName,
    projectName: project.projectName,
    description: project.description,
    isChildFriendly: project.isChildFriendly,
    isRecommended: project.isRecommended,
    category: project.category,
    isTour: project.type === 'laboratory' ? project.isTour : undefined,
    offering: project.type === 'food-stall' ? project.offering : undefined,
    foodStallTags:
      project.type === 'food-stall'
        ? project.tag.map((tag) =>
            'tag2' in tag ? { tag: tag.tag, tag2: tag.tag2 } : { tag: tag.tag },
          )
        : undefined,
    generalTags: project.type === 'general' ? project.tag : undefined,
    occasions: project.occasions.map((occasion) => ({
      date: occasion.timeRange.start.date,
      place: occasion.place,
      start: formatTime(occasion.timeRange.start),
      end: formatTime(occasion.timeRange.end),
    })),
  };
}

function buildOccasions(values: OccasionValues[]): Occasion[] {
  return values.map((occasion) => ({
    // 未指定は null ではなくキーごと省く(spec 上 optional であって nullable ではない)。
    ...(occasion.place ? { place: occasion.place } : {}),
    timeRange: {
      start: parseTime(occasion.date, occasion.start),
      end: parseTime(occasion.date, occasion.end),
    },
  }));
}

function buildFoodStallTags(values: FoodStallTagValues[]): FoodStallTag[] {
  return values.map((value) => {
    if (value.tag === 'drink') return { tag: 'drink' };
    if (!value.tag2) {
      throw new Error(`模擬店タグ ${value.tag} には tag2 が必要です`);
    }
    return { tag: value.tag, tag2: value.tag2 } as FoodStallTag;
  });
}

/**
 * 入力値を [`Project`] にする。
 *
 * PUT は差分更新ではなく全置き換えなので、種別を変えたときは元の種別だけが持つ
 * 項目(タグ・ラボツアー)は送らずに落とす。
 */
function buildProject(id: string, values: FormValues): Project {
  const base = {
    id,
    groupName: values.groupName,
    projectName: values.projectName,
    description: values.description,
    isChildFriendly: values.isChildFriendly ?? false,
    isRecommended: values.isRecommended ?? false,
    occasions: buildOccasions(values.occasions ?? []),
    ...(values.category ? { category: values.category } : {}),
  };

  switch (values.type) {
    case 'food-stall':
      return {
        ...base,
        type: 'food-stall',
        tag: buildFoodStallTags(values.foodStallTags ?? []),
        ...(values.offering?.trim()
          ? { offering: values.offering.trim() }
          : {}),
      };
    case 'general':
      return { ...base, type: 'general', tag: values.generalTags ?? [] };
    case 'stage':
      return { ...base, type: 'stage' };
    case 'laboratory':
      return { ...base, type: 'laboratory', isTour: values.isTour ?? false };
  }
}

export function EditProjectPage() {
  const [queryClient] = useState(() => new QueryClient());
  const [projectId, setProjectId] = useState<string | null>();

  useEffect(() => {
    setProjectId(new URLSearchParams(window.location.search).get('project_id'));
  }, []);

  if (projectId === undefined) {
    return <LoadingScreen />;
  }

  if (!projectId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="企画番号が指定されていません。URLに?project_id=M-001のように指定してください。"
        extra={
          <Button href="/events26" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <EditProjectForm projectId={projectId} />
    </QueryClientProvider>
  );
}

function EditProjectForm({ projectId }: { projectId: string }) {
  const [form] = Form.useForm<FormValues>();
  const [messageApi, contextHolder] = message.useMessage();
  // 1 件の取得は backend に中継が無いので events26 の公開エンドポイントを直接読む。
  const {
    data: project,
    isLoading,
    error,
    refetch,
  } = $events26Api.useQuery('get', '/v1/projects/{projectId}', {
    params: { path: { projectId } },
  });
  const { data: places } = $events26Api.useQuery('get', '/v1/places');
  const [submitting, setSubmitting] = useState(false);
  // アイコンは URL が同じまま中身だけ変わるので、更新後はこの値を進めて再取得させる。
  const [iconVersion, setIconVersion] = useState(0);

  const placeOptions = useMemo(
    () =>
      places?.map((place) => ({
        value: place.id,
        label: `${place.displayName}（${place.id}）`,
      })) ?? [],
    [places],
  );

  // 種別で出し入れする項目のために監視する。初回描画では値が入らないので、
  // 取得済みの企画の種別を既定にしてちらつきを避ける。
  const watchedType = Form.useWatch('type', form);

  if (isLoading) {
    return <LoadingScreen />;
  }

  if (!project) {
    return (
      <Result
        status="error"
        title="データを取得できませんでした"
        subTitle={String(error)}
        extra={
          <Button href="/events26" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  const type = watchedType ?? project.type;

  const handleSubmit = async (values: FormValues) => {
    setSubmitting(true);
    try {
      ensureOk(
        await api.PUT('/events26/projects/{project_id}', {
          params: { path: { project_id: projectId } },
          body: buildProject(projectId, values),
        }),
        '企画情報の更新',
      );
      messageApi.success('保存しました');
      await refetch();
    } catch (e) {
      console.error(e);
      messageApi.error(`保存に失敗しました: ${String(e)}`);
    } finally {
      setSubmitting(false);
    }
  };

  const handleIconUpload = async (file: File) => {
    if (!ICON_CONTENT_TYPES.includes(file.type)) {
      messageApi.error(`対応外のアイコン形式です（${file.type}）`);
      return;
    }
    try {
      await putIcon(projectId, file);
      setIconVersion((version) => version + 1);
      messageApi.success('アイコンをアップロードしました');
    } catch (e) {
      console.error(e);
      messageApi.error(`アイコンのアップロードに失敗しました: ${String(e)}`);
    }
  };

  const handleIconDelete = async () => {
    try {
      ensureOk(
        await api.DELETE('/events26/projects/{project_id}/icon', {
          params: { path: { project_id: projectId } },
        }),
        'アイコンの削除',
      );
      setIconVersion((version) => version + 1);
      messageApi.success('アイコンを削除しました');
    } catch (e) {
      console.error(e);
      messageApi.error(`アイコンの削除に失敗しました: ${String(e)}`);
    }
  };

  return (
    <>
      <h1>企画情報を編集</h1>
      <Form
        form={form}
        layout="vertical"
        onFinish={handleSubmit}
        initialValues={toFormValues(project)}
      >
        <Form.Item label="企画番号">
          {/* id は events26 側のキーで、置き換えでは変えられない。 */}
          <Input value={projectId} disabled />
        </Form.Item>
        <Form.Item
          name="type"
          label="種類"
          rules={[{ required: true, message: '種類を選択してください' }]}
        >
          <Select options={PROJECT_TYPE_OPTIONS} />
        </Form.Item>
        <Form.Item
          name="groupName"
          label="団体名"
          rules={[{ required: true, message: '団体名を入力してください' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          name="projectName"
          label="企画名"
          rules={[{ required: true, message: '企画名を入力してください' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          name="description"
          label="概要"
          rules={[{ required: true, message: '概要を入力してください' }]}
        >
          <Input.TextArea autoSize={{ minRows: 3 }} />
        </Form.Item>
        <Form.Item name="isChildFriendly" valuePropName="checked">
          <Checkbox>子供向け企画</Checkbox>
        </Form.Item>
        <Form.Item name="isRecommended" valuePropName="checked">
          <Checkbox>おすすめ企画</Checkbox>
        </Form.Item>
        <Form.Item name="category" label="カテゴリー">
          <Select
            allowClear
            options={CATEGORIES.map((category) => ({
              value: category,
              label: CATEGORY_LABEL[category],
            }))}
            placeholder="カテゴリーを選択（任意）"
          />
        </Form.Item>

        {type === 'laboratory' && (
          <Form.Item name="isTour" valuePropName="checked">
            <Checkbox>ラボツアー</Checkbox>
          </Form.Item>
        )}

        {type === 'general' && (
          <Form.Item name="generalTags" label="タグ">
            <Select
              mode="multiple"
              allowClear
              options={GENERAL_TAGS.map((tag) => ({ value: tag, label: tag }))}
              placeholder="タグを選択"
            />
          </Form.Item>
        )}

        {type === 'food-stall' && (
          <>
            <Form.Item name="offering" label="提供品目">
              <Input placeholder="提供するメニュー・商品など（任意）" />
            </Form.Item>
            <Form.Item label="タグ">
              <Form.List name="foodStallTags">
                {(fields, { add, remove }) => (
                  <Flex gap={8} vertical>
                    {fields.map((field) => (
                      <Space key={field.key} align="baseline">
                        <Form.Item
                          name={[field.name, 'tag']}
                          noStyle
                          rules={[
                            { required: true, message: 'tag は必須です' },
                          ]}
                        >
                          <Select
                            style={{ width: 180 }}
                            options={FOOD_STALL_TAG_OPTIONS}
                            placeholder="tag"
                            onChange={() => {
                              // tag を変えると選べる tag2 も変わるので、古い値を残さない。
                              const tags =
                                form.getFieldValue('foodStallTags') ?? [];
                              tags[field.name] = {
                                ...tags[field.name],
                                tag2: undefined,
                              };
                              form.setFieldValue('foodStallTags', [...tags]);
                            }}
                          />
                        </Form.Item>
                        <Form.Item
                          noStyle
                          shouldUpdate={(prev, next) =>
                            prev.foodStallTags?.[field.name]?.tag !==
                            next.foodStallTags?.[field.name]?.tag
                          }
                        >
                          {({ getFieldValue }) => {
                            const tag: FoodStallTag['tag'] | undefined =
                              getFieldValue([
                                'foodStallTags',
                                field.name,
                                'tag',
                              ]);
                            if (!tag || tag === 'drink') return null;
                            return (
                              <Form.Item
                                name={[field.name, 'tag2']}
                                noStyle
                                rules={[
                                  {
                                    required: true,
                                    message: 'tag2 は必須です',
                                  },
                                ]}
                              >
                                <Select
                                  style={{ width: 180 }}
                                  options={FOOD_STALL_TAG2[tag].map(
                                    (value) => ({
                                      value,
                                      label: value,
                                    }),
                                  )}
                                  placeholder="tag2"
                                />
                              </Form.Item>
                            );
                          }}
                        </Form.Item>
                        <MinusCircleOutlined
                          onClick={() => remove(field.name)}
                        />
                      </Space>
                    ))}
                    <Form.Item noStyle>
                      <Button
                        type="dashed"
                        onClick={() => add({ tag: 'main' })}
                        block
                        icon={<PlusOutlined />}
                      >
                        タグを追加
                      </Button>
                    </Form.Item>
                  </Flex>
                )}
              </Form.List>
            </Form.Item>
          </>
        )}

        <Form.Item label="開催予定">
          <Form.List name="occasions">
            {(fields, { add, remove }) => (
              <Flex gap={8} vertical>
                {fields.map((field) => (
                  <Space key={field.key} align="baseline" wrap>
                    <Form.Item
                      name={[field.name, 'date']}
                      noStyle
                      rules={[{ required: true, message: '日を選択' }]}
                    >
                      <Select
                        style={{ width: 110 }}
                        options={[
                          { value: 1, label: '1日目' },
                          { value: 2, label: '2日目' },
                        ]}
                      />
                    </Form.Item>
                    <Form.Item
                      name={[field.name, 'start']}
                      noStyle
                      rules={[
                        { required: true, message: '開始時刻は必須です' },
                        TIME_RULE,
                      ]}
                    >
                      <Input style={{ width: 100 }} placeholder="10:00" />
                    </Form.Item>
                    <Form.Item
                      name={[field.name, 'end']}
                      noStyle
                      rules={[
                        { required: true, message: '終了時刻は必須です' },
                        TIME_RULE,
                      ]}
                    >
                      <Input style={{ width: 100 }} placeholder="17:00" />
                    </Form.Item>
                    <Form.Item name={[field.name, 'place']} noStyle>
                      <Select
                        style={{ width: 360 }}
                        options={placeOptions}
                        placeholder="実施場所（任意）"
                        showSearch
                        allowClear
                        optionFilterProp="label"
                      />
                    </Form.Item>
                    <MinusCircleOutlined onClick={() => remove(field.name)} />
                  </Space>
                ))}
                <Form.Item noStyle>
                  <Button
                    type="dashed"
                    onClick={() => add({ date: 1 })}
                    block
                    icon={<PlusOutlined />}
                  >
                    開催予定を追加
                  </Button>
                </Form.Item>
              </Flex>
            )}
          </Form.List>
        </Form.Item>

        <Form.Item>
          <Flex gap={8}>
            <Button type="primary" htmlType="submit" disabled={submitting}>
              保存
            </Button>
            <Button type="default" href="/events26">
              戻る
            </Button>
          </Flex>
        </Form.Item>
      </Form>

      <Divider />

      {/* アイコンは企画本体とは別の API なので、フォームの保存とは独立して更新する。 */}
      <Card title="アイコン" size="small" style={{ maxWidth: 480 }}>
        <Flex gap={16} align="center" wrap>
          <img
            src={iconUrl(projectId, iconVersion)}
            alt="企画のアイコン"
            width={96}
            height={96}
            // 未登録の企画は 404 になる。壊れた画像アイコンを出さずに隠す。
            onError={(event) => {
              event.currentTarget.style.visibility = 'hidden';
            }}
          />
          <Flex gap={8} vertical>
            <Upload
              maxCount={1}
              showUploadList={false}
              accept={ICON_CONTENT_TYPES.join(',')}
              beforeUpload={async (file) => {
                await handleIconUpload(file);
                return false;
              }}
            >
              <Button icon={<UploadOutlined />}>アイコンをアップロード</Button>
            </Upload>
            <Popconfirm
              title="アイコンを削除"
              description="この企画のアイコンを削除しますか？"
              onConfirm={handleIconDelete}
              okText="はい"
              cancelText="いいえ"
            >
              <Button danger icon={<DeleteOutlined />}>
                アイコンを削除
              </Button>
            </Popconfirm>
          </Flex>
        </Flex>
        <p style={{ marginBottom: 0, color: '#888' }}>
          対応形式は png / jpeg / gif / webp / heic、正方形で 20MB 以下です。
        </p>
      </Card>
      {contextHolder}
    </>
  );
}
