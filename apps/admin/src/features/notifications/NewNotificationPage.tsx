import { MinusCircleOutlined, PlusOutlined } from '@ant-design/icons';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Button, Flex, Form, Input, message, Radio, Space } from 'antd';
import { useState } from 'react';
import { $api } from '@/features/api/api';
import { TargetSpecifier } from '@/features/documents/TargetSpecifier';

type FormValues = {
  title: string;
  target: string[][];
  markdown: string;
};

export function NewNotificationPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <NewNotification />
    </QueryClientProvider>
  );
}

function NewNotification() {
  const [messageApi, contextHolder] = message.useMessage();
  const { mutateAsync: mutateNotificationCreate } = $api.useMutation(
    'post',
    '/notifications',
  );
  const [submitting, setSubmitting] = useState(false);
  const formType = 'markdown';

  const handleSubmit = async ({ title, target, markdown }: FormValues) => {
    setSubmitting(true);
    try {
      await mutateNotificationCreate({
        body: {
          targets: target.map((item) => item.join('/')),
          type: 'markdown',
          title,
          content: markdown,
        },
      });
    } catch (e) {
      setSubmitting(false);
      messageApi.error(`保存に失敗しました: ${String(e)}`);
      return;
    }

    setSubmitting(false);
    messageApi.success('保存しました');
    window.location.assign('/notifications');
  };

  return (
    <>
      <Form onFinish={handleSubmit} initialValues={{ target: [] }}>
        <h1>新規通知を作成</h1>
        <Form.Item name="title" label="タイトル" rules={[{ required: true }]}>
          <Input placeholder="タイトルを入力してください" />
        </Form.Item>

        <Form.Item label="通知対象" rules={[{ required: true }]}>
          <Form.List name="target">
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

        <Form.Item label="通知の種類">
          <Radio.Group defaultValue={formType}>
            <Radio.Button value="markdown">MD</Radio.Button>
          </Radio.Group>
        </Form.Item>

        <Form.Item
          name="markdown"
          label="markdownの内容"
          rules={[{ required: true }]}
        >
          <Input.TextArea />
        </Form.Item>

        <Form.Item>
          <Flex gap={8}>
            <Button type="primary" htmlType="submit" disabled={submitting}>
              送信
            </Button>
            <Button type="default" href="/notifications">
              戻る
            </Button>
          </Flex>
        </Form.Item>
      </Form>
      {contextHolder}
    </>
  );
}
