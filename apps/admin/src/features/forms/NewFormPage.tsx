import { MinusCircleOutlined, PlusOutlined } from '@ant-design/icons';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Button, Flex, Form, Input, message, Radio, Space } from 'antd';
import { useState } from 'react';
import { api, $api } from '@/features/api/api';
import { TargetSpecifier } from '@/features/documents/TargetSpecifier';

type FormValues = {
  formName: string;
  summary: string;
  url: string;
  targets: string[][];
  dueDate: string | undefined;
};

export function NewFormPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <NewForm />
    </QueryClientProvider>
  );
}

function NewForm() {
  const [messageApi, contextHolder] = message.useMessage();
  const { mutateAsync: mutateFormCreate } = $api.useMutation('post', '/forms');
  const [submitting, setSubmitting] = useState(false);
  const [form] = Form.useForm<FormValues>();
  const urlValue = Form.useWatch('url', form) ?? '';

  const handleSubmit = async ({
    formName,
    summary,
    url,
    targets,
    dueDate,
  }: FormValues) => {
    setSubmitting(true);
    // api_v3 では due_date は必須。
    if (!dueDate) {
      setSubmitting(false);
      messageApi.error('回答期限を入力してください');
      return;
    }
    try {
      await mutateFormCreate({
        body: {
          name: formName,
          targets: targets.map((target) => target.join('/')),
          summary,
          type: 'external',
          form_url: url,
          due_date: new Date(dueDate).toISOString(),
        },
      });
    } catch (e) {
      setSubmitting(false);
      messageApi.error(`保存に失敗しました: ${String(e)}`);
      return;
    }

    setSubmitting(false);
    messageApi.success('保存しました');
    window.location.assign('/forms');
  };

  const syncFormNameAndSummary = async () => {
    messageApi.loading('外部フォームのメタデータを取得中...');
    try {
      const result = await api.GET('/util/meta', {
        params: { query: { url: urlValue } },
      });

      if (!result.data) {
        messageApi.destroy();
        messageApi.error(
          `外部フォームのメタデータを取得できませんでした: ${String(result.error)}`,
        );
        return;
      }

      if (!result.data.title || !result.data.description) {
        messageApi.destroy();
        messageApi.warning(
          '外部フォームのメタデータにタイトルまたは要約が含まれていません',
        );
      } else {
        messageApi.destroy();
        messageApi.success('外部フォームのメタデータを取得しました');
      }

      if (result.data.title) {
        form.setFieldsValue({ formName: result.data.title });
      }
      if (result.data.description) {
        form.setFieldsValue({ summary: result.data.description });
      }
    } catch (e) {
      messageApi.destroy();
      messageApi.error(
        `外部フォームのメタデータのfetchに失敗しました: ${String(e)}`,
      );
    }
  };

  return (
    <>
      <Form
        form={form}
        onFinish={handleSubmit}
        initialValues={{ targets: [], formType: 'external' }}
      >
        <h1>新規フォームを作成</h1>
        <Form.Item
          name="formName"
          label="フォーム名"
          rules={[{ required: true }]}
        >
          <Input placeholder="フォーム名を入力してください" />
        </Form.Item>

        <Form.Item name="summary" label="要約" rules={[{ required: true }]}>
          <Input.TextArea placeholder="フォームの要約を入力してください" />
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

        <Form.Item label="フォームの種類">
          <Radio.Group defaultValue="external">
            <Radio.Button value="external">外部</Radio.Button>
          </Radio.Group>
        </Form.Item>

        <Form.Item
          name="url"
          label="外部フォームurl"
          rules={[{ required: true }]}
        >
          <Input />
        </Form.Item>
        <Form.Item>
          <Button type="default" onClick={syncFormNameAndSummary}>
            フォーム名と要約を自動取得
          </Button>
        </Form.Item>

        <Form.Item name="dueDate" label="回答期限" rules={[{ required: true }]}>
          <Input type="datetime-local" />
        </Form.Item>

        <Form.Item>
          <Flex gap={8}>
            <Button type="primary" htmlType="submit" disabled={submitting}>
              送信
            </Button>
            <Button type="default" href="/forms">
              戻る
            </Button>
          </Flex>
        </Form.Item>
      </Form>
      {contextHolder}
    </>
  );
}
