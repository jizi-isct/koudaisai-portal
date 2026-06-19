import {
  MinusCircleOutlined,
  PlusOutlined,
  UploadOutlined,
} from '@ant-design/icons';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import {
  Button,
  Flex,
  Form,
  Input,
  message,
  Radio,
  Result,
  Select,
  Space,
  Upload,
} from 'antd';
import type { UploadFile } from 'antd';
import { useEffect, useMemo, useState } from 'react';
import { $api, api } from '@/features/api/api';
import { TargetSpecifier } from './TargetSpecifier';

type FormValues =
  | {
      title: string;
      category: string;
      targets: string[][];
      documentFormat: 'pdf';
      pdfFile: UploadFile[];
    }
  | {
      title: string;
      category: string;
      targets: string[][];
      documentFormat: 'markdown';
      markdownContent: string;
    }
  | {
      title: string;
      category: string;
      targets: string[][];
      documentFormat: 'misc';
      miscFile: UploadFile[];
    };

export function NewDocumentPage() {
  const [queryClient] = useState(() => new QueryClient());
  const [categoryId, setCategoryId] = useState<string | null>();

  useEffect(() => {
    setCategoryId(
      new URLSearchParams(window.location.search).get('category_id'),
    );
  }, []);

  if (categoryId === undefined) {
    return <LoadingScreen />;
  }

  if (!categoryId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="カテゴリーIDが指定されていません。URLに?category_id=xxxxのように指定してください。"
        extra={
          <Button href="/documents" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <NewDocumentForm categoryId={categoryId} />
    </QueryClientProvider>
  );
}

function NewDocumentForm({ categoryId }: { categoryId: string }) {
  const [form] = Form.useForm();
  const [messageApi, contextHolder] = message.useMessage();
  const { mutateAsync: mutateDocumentCreate } = $api.useMutation(
    'post',
    '/documents',
  );
  const { data: categories } = $api.useQuery('get', '/document-categories');
  const categoryOptions = useMemo(
    () =>
      categories?.map((category) => ({
        value: category.id,
        label: category.title,
      })) ?? [],
    [categories],
  );
  const [submitting, setSubmitting] = useState(false);
  const documentFormat = Form.useWatch('documentFormat', form);

  const uploadFile = async (file: UploadFile) => {
    // 生の fetch client を使う。openapi-react-query の mutateAsync は、エラー
    // レスポンスのボディが空(バックエンドの 403/500 は本文なし)の場合に throw
    // せず undefined を返してしまうため、HTTP ステータスを直接確認する。
    const { data, response } = await api.POST('/files/upload', {
      body: {
        file_name: file.name,
      },
    });

    if (!data) {
      throw new Error(
        `アップロードURLの取得に失敗しました (HTTP ${response.status})`,
      );
    }

    const uploadResponse = await fetch(data.presigned_url, {
      method: 'PUT',
      body: file.originFileObj,
    });
    if (!uploadResponse.ok) {
      throw new Error(
        `ファイルのアップロードに失敗しました (HTTP ${uploadResponse.status})`,
      );
    }

    return data;
  };

  const handleSubmit = async (values: FormValues) => {
    setSubmitting(true);
    try {
      switch (values.documentFormat) {
        case 'pdf': {
          const { key } = await uploadFile(values.pdfFile[0]);
          await mutateDocumentCreate({
            body: {
              title: values.title,
              category: values.category,
              targets: values.targets.map((target) => target.join('/')),
              format: 'pdf',
              file_name: values.pdfFile[0].name,
              file_key: key,
            },
          });
          break;
        }
        case 'markdown':
          await mutateDocumentCreate({
            body: {
              title: values.title,
              category: values.category,
              targets: values.targets.map((target) => target.join('/')),
              format: 'markdown',
              content: values.markdownContent,
            },
          });
          break;
        case 'misc': {
          const { key } = await uploadFile(values.miscFile[0]);
          await mutateDocumentCreate({
            body: {
              title: values.title,
              category: values.category,
              targets: values.targets.map((target) => target.join('/')),
              format: 'misc',
              file_name: values.miscFile[0].name,
              file_key: key,
            },
          });
          break;
        }
      }
    } catch (e) {
      setSubmitting(false);
      messageApi.error(`保存に失敗しました: ${String(e)}`);
      return;
    }

    setSubmitting(false);
    messageApi.success('保存しました');
    window.location.assign('/documents');
  };

  return (
    <>
      <Form
        onFinish={handleSubmit}
        form={form}
        initialValues={{
          category: categoryId,
          documentFormat: 'pdf',
          targets: [['group', 'type', 'press']],
        }}
      >
        <h1>新規資料を作成</h1>
        <Form.Item name="title" label="タイトル" rules={[{ required: true }]}>
          <Input placeholder="タイトルを入力してください" />
        </Form.Item>

        <Form.Item
          name="category"
          label="カテゴリー"
          rules={[{ required: true }]}
        >
          <Select options={categoryOptions} />
        </Form.Item>

        <Form.Item label="対象" rules={[{ required: true }]}>
          <Form.List name="targets">
            {(fields, { add, remove }) => (
              <Flex gap={16} vertical>
                {fields.map((field) => (
                  <Space key={field.key}>
                    <Form.Item
                      name={field.name}
                      noStyle
                      rules={[{ required: true }]}
                    >
                      <TargetSpecifier />
                    </Form.Item>
                    <MinusCircleOutlined onClick={() => remove(field.name)} />
                  </Space>
                ))}
                <Form.Item>
                  <Button
                    type="dashed"
                    onClick={() => add()}
                    block
                    icon={<PlusOutlined />}
                  >
                    追加
                  </Button>
                </Form.Item>
              </Flex>
            )}
          </Form.List>
        </Form.Item>

        <Form.Item
          label="資料のフォーマット"
          name="documentFormat"
          rules={[{ required: true }]}
        >
          <Radio.Group>
            <Radio.Button value="pdf">PDF</Radio.Button>
            <Radio.Button value="markdown">Markdown</Radio.Button>
            <Radio.Button value="misc">その他</Radio.Button>
          </Radio.Group>
        </Form.Item>

        {documentFormat === 'pdf' && (
          <Form.Item
            label="PDFファイル"
            rules={[{ required: true }]}
            name="pdfFile"
            valuePropName="fileList"
            getValueFromEvent={(e) => (Array.isArray(e) ? e : e?.fileList)}
          >
            <Upload
              accept="application/pdf"
              beforeUpload={() => false}
              maxCount={1}
            >
              <Button>
                <UploadOutlined />
                PDFファイルをアップロード
              </Button>
            </Upload>
          </Form.Item>
        )}

        {documentFormat === 'markdown' && (
          <Form.Item
            label="markdown"
            name="markdownContent"
            rules={[{ required: true }]}
          >
            <Input.TextArea
              rows={10}
              placeholder="Markdown形式で資料の内容を入力してください"
            />
          </Form.Item>
        )}

        {documentFormat === 'misc' && (
          <Form.Item
            label="ファイル"
            rules={[{ required: true }]}
            name="miscFile"
            valuePropName="fileList"
            getValueFromEvent={(e) => (Array.isArray(e) ? e : e?.fileList)}
          >
            <Upload beforeUpload={() => false} maxCount={1}>
              <Button>
                <UploadOutlined />
                ファイルをアップロード
              </Button>
            </Upload>
          </Form.Item>
        )}

        <Form.Item>
          <Flex gap={8}>
            <Button type="primary" htmlType="submit" disabled={submitting}>
              送信
            </Button>
            <Button type="default" href="/documents">
              戻る
            </Button>
          </Flex>
        </Form.Item>
      </Form>
      {contextHolder}
    </>
  );
}
