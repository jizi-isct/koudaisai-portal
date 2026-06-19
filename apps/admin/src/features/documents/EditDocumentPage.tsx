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
  Result,
  Select,
  Space,
  Tag,
  Upload,
} from 'antd';
import type { UploadFile } from 'antd';
import { useEffect, useMemo, useState } from 'react';
import { $api } from '@/features/api/api';
import { TargetSpecifier } from './TargetSpecifier';

type FormValues = {
  title: string;
  category: string;
  targets: string[][];
  pdfFile?: UploadFile[];
  markdownContent?: string;
  miscFile?: UploadFile[];
};

export function EditDocumentPage() {
  const [queryClient] = useState(() => new QueryClient());
  const [documentId, setDocumentId] = useState<string | null>();

  useEffect(() => {
    setDocumentId(
      new URLSearchParams(window.location.search).get('document_id'),
    );
  }, []);

  if (documentId === undefined) {
    return <LoadingScreen />;
  }

  if (!documentId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="資料IDが指定されていません。URLに?document_id=xxxxのように指定してください。"
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
      <EditDocumentForm documentId={documentId} />
    </QueryClientProvider>
  );
}

function EditDocumentForm({ documentId }: { documentId: string }) {
  const [form] = Form.useForm();
  const [messageApi, contextHolder] = message.useMessage();
  const {
    data: documentRead,
    isLoading: isLoadingDocuments,
    error,
    refetch,
  } = $api.useQuery('get', '/documents/{id}', {
    params: {
      path: {
        id: documentId,
      },
    },
  });
  const { mutateAsync: mutateUploadFile } = $api.useMutation(
    'post',
    '/files/upload',
  );
  const { data: categories, isLoading: isLoadingCategories } = $api.useQuery(
    'get',
    '/document-categories',
  );
  const { mutateAsync: mutateDocumentUpdate } = $api.useMutation(
    'patch',
    '/documents/{id}',
  );
  const [submitting, setSubmitting] = useState(false);

  const categoryOptions = useMemo(
    () =>
      categories?.map((category) => ({
        value: category.id,
        label: category.title,
      })) ?? [],
    [categories],
  );

  if (isLoadingDocuments || isLoadingCategories) {
    return <LoadingScreen />;
  }

  if (!documentRead || !categories) {
    return (
      <Result
        status="error"
        title="データを取得できませんでした"
        subTitle={String(error)}
        extra={
          <Button href="/documents" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  // format は平坦化された判別子(pdf/markdown/misc)。編集では変更不可。
  const documentFormat = documentRead.format;

  const uploadFile = async (file: UploadFile) => {
    const uploaded = await mutateUploadFile({
      body: {
        file_name: file.name,
      },
    });

    await fetch(uploaded.presigned_url, {
      method: 'PUT',
      body: file.originFileObj,
    });

    return uploaded;
  };

  const handleSubmit = async (values: FormValues) => {
    setSubmitting(true);
    messageApi.loading('保存中...');
    try {
      // DocumentUpdate は format 判別 union を単一フィールド format で受ける。
      // ファイル未差し替え(uid==='default')や markdown 未入力のときは format を
      // 省略して他項目のみ更新する。
      let format = undefined;
      if (documentRead.format === 'pdf') {
        if (values.pdfFile && values.pdfFile[0].uid !== 'default') {
          const { key } = await uploadFile(values.pdfFile[0]);
          format = {
            format: 'pdf' as const,
            file_name: values.pdfFile[0].name,
            file_key: key,
          };
        }
      } else if (documentRead.format === 'markdown') {
        format = values.markdownContent
          ? { format: 'markdown' as const, content: values.markdownContent }
          : undefined;
      } else if (documentRead.format === 'misc') {
        if (values.miscFile && values.miscFile[0].uid !== 'default') {
          const { key } = await uploadFile(values.miscFile[0]);
          format = {
            format: 'misc' as const,
            file_name: values.miscFile[0].name,
            file_key: key,
          };
        }
      }

      await mutateDocumentUpdate({
        params: { path: { id: documentId } },
        body: {
          title: values.title,
          category: values.category,
          targets: values.targets.map((target) => target.join('/')),
          format,
        },
      });
    } catch (e) {
      setSubmitting(false);
      messageApi.destroy();
      messageApi.error(`保存に失敗しました: ${String(e)}`);
      return;
    }

    setSubmitting(false);
    messageApi.destroy();
    messageApi.success('保存しました');
    await refetch();
  };

  return (
    <>
      <Form
        onFinish={handleSubmit}
        initialValues={{
          title: documentRead.title,
          category: documentRead.category,
          targets: documentRead.targets.map((target) => target.split('/')),
          pdfFile:
            documentRead.format === 'pdf'
              ? [
                  {
                    uid: 'default',
                    name: documentRead.file_name,
                    status: 'done',
                  },
                ]
              : [],
          miscFile:
            documentRead.format === 'misc'
              ? [
                  {
                    uid: 'default',
                    name: documentRead.file_name,
                    status: 'done',
                  },
                ]
              : [],
        }}
        form={form}
      >
        <h1>資料を編集</h1>
        <Form.Item name="title" label="タイトル" required>
          <Input placeholder="タイトルを入力してください" />
        </Form.Item>

        <Form.Item name="category" label="カテゴリー" required>
          <Select options={categoryOptions} />
        </Form.Item>

        <Form.Item label="対象" required>
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

        <Form.Item label="資料のフォーマット">
          {documentRead.format === 'pdf' && <Tag color="orange">PDF</Tag>}
          {documentRead.format === 'markdown' && (
            <Tag color="green">Markdown</Tag>
          )}
          {documentRead.format === 'misc' && <Tag color="blue">その他</Tag>}
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
            initialValue={
              documentRead.format === 'markdown' ? documentRead.content : ''
            }
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
