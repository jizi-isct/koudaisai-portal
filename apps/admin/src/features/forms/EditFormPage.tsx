import { MinusCircleOutlined, PlusOutlined } from '@ant-design/icons';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Button, Flex, Form, Input, message, Radio, Result, Space } from 'antd';
import { useEffect, useState } from 'react';
import { $api } from '@/features/api/api';
import { TargetSpecifier } from '@/features/documents/TargetSpecifier';
import { timeZoneOffset } from './TimeZoneOffset/timeZoneOffset.ts';

type FormValues = {
  formName: string | undefined;
  targets: string[][] | undefined;
  summary: string | undefined;
  dueDate: string | undefined;
  url: string | undefined;
};

export function EditFormPage() {
  const [queryClient] = useState(() => new QueryClient());
  const [formId, setFormId] = useState<string | null>();

  useEffect(() => {
    setFormId(new URLSearchParams(window.location.search).get('form_id'));
  }, []);

  if (formId === undefined) {
    return <LoadingScreen />;
  }

  if (!formId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="フォームIDが指定されていません。URLに?form_id=xxxxのように指定してください。"
        extra={
          <Button href="/forms" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <EditForm formId={formId} />
    </QueryClientProvider>
  );
}

function EditForm({ formId }: { formId: string }) {
  const [messageApi, contextHolder] = message.useMessage();
  const {
    data: form,
    isLoading,
    error,
  } = $api.useQuery('get', '/forms/{id}', {
    params: { path: { id: formId } },
  });
  const { mutateAsync: mutateFormUpdate } = $api.useMutation(
    'patch',
    '/forms/{id}',
  );
  const [submitting, setSubmitting] = useState(false);

  // api_v3 のフォームは外部フォームのみ。
  const formType = form?.type;

  if (isLoading) return <LoadingScreen />;

  if (!form) {
    return (
      <Result
        status="error"
        title="データを取得できませんでした"
        subTitle={String(error)}
        extra={
          <Button href="/forms" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  const handleSubmit = async ({
    targets,
    formName,
    summary,
    dueDate,
  }: FormValues) => {
    setSubmitting(true);
    try {
      // FormUpdate は name/summary/targets/due_date のみ。フォーム URL/種別は変更不可。
      await mutateFormUpdate({
        params: { path: { id: formId } },
        body: {
          name: formName,
          targets: targets?.map((target) => target.join('/')),
          summary,
          due_date: dueDate ? new Date(dueDate).toISOString() : undefined,
        },
      });
    } catch (e) {
      setSubmitting(false);
      messageApi.error(`保存に失敗しました: ${String(e)}`);
      return;
    }

    setSubmitting(false);
    messageApi.success('保存しました');
  };

  return (
    <>
      <Form
        onFinish={handleSubmit}
        initialValues={{
          formName: form.name,
          summary: form.summary,
          targets: form.targets.map((target) => target.split('/')),
          url: form.type === 'external' ? form.form_url : '',
          dueDate: form.due_date
            ? timeZoneOffset({ serverDate: new Date(form.due_date) })
            : undefined,
        }}
      >
        <h1>フォームを編集</h1>
        <Form.Item name="formName" label="フォーム名">
          <Input placeholder="フォーム名を入力してください" />
        </Form.Item>

        <Form.Item name="summary" label="要約" required>
          <Input.TextArea placeholder="フォームの要約を入力してください" />
        </Form.Item>

        <Form.Item label="対象">
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
          <Radio.Group defaultValue={formType}>
            <Radio.Button value="external">外部</Radio.Button>
          </Radio.Group>
        </Form.Item>

        <Form.Item name="url" label="外部フォームurl">
          <Input.TextArea />
        </Form.Item>

        <Form.Item name="dueDate" label="回答期限">
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
