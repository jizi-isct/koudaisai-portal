import {MinusCircleOutlined, PlusOutlined} from "@ant-design/icons";
import {LoadingScreen} from "@koudaisai/shared-ui";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Button, Flex, Form, Input, message, Radio, Result, Space} from "antd";
import {useEffect, useState} from "react";
import {$api} from "@/features/api/api";
import {TargetSpecifier} from "@/features/documents/TargetSpecifier";

type FormValues = {
  title: string | undefined;
  target: string[][] | undefined;
  markdown: string | undefined;
};

export function EditNotificationPage() {
  const [queryClient] = useState(() => new QueryClient());
  const [notificationId, setNotificationId] = useState<string | null>();

  useEffect(() => {
    setNotificationId(new URLSearchParams(window.location.search).get("notification_id"));
  }, []);

  if (notificationId === undefined) {
    return <LoadingScreen />;
  }

  if (!notificationId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="通知IDが指定されていません。URLに?notification_id=xxxxのように指定してください。"
        extra={
          <Button href="/notifications" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <EditNotification notificationId={notificationId} />
    </QueryClientProvider>
  );
}

function EditNotification({notificationId}: {notificationId: string}) {
  const [messageApi, contextHolder] = message.useMessage();
  const {data, isLoading, error} = $api.useQuery("get", "/notifications/{notification_id}", {
    params: {
      path: {
        notification_id: notificationId,
      },
    },
  });
  const {mutateAsync: mutateNotificationUpdate} = $api.useMutation(
    "patch",
    "/notifications/{notification_id}",
  );
  const [submitting, setSubmitting] = useState(false);

  if (isLoading) {
    return <LoadingScreen />;
  }

  if (!data) {
    return (
      <Result
        status="error"
        title="データを取得できませんでした"
        subTitle={String(error)}
        extra={
          <Button href="/notifications" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  if (!("type_markdown" in data)) {
    return (
      <Result
        status="error"
        title="この種類の通知は編集できません"
        subTitle={String(error)}
        extra={
          <Button href="/notifications" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  const handleSubmit = async ({title, target, markdown}: FormValues) => {
    setSubmitting(true);
    try {
      await mutateNotificationUpdate({
        params: {
          path: {
            notification_id: notificationId,
          },
        },
        body: {
          target: target?.map((item) => item.join("/")),
          type_markdown: {
            title,
            content: markdown,
          },
        },
      });
      messageApi.success("保存しました");
    } catch (e) {
      messageApi.error(`保存に失敗しました: ${String(e)}`);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <>
      <Form
        onFinish={handleSubmit}
        initialValues={{
          title: data.type_markdown.title,
          target: data.target.map((item) => item.split("/")),
          markdown: data.type_markdown.content,
        }}
      >
        <h1>通知を編集</h1>
        <Form.Item name="title" label="タイトル">
          <Input placeholder="タイトルを入力してください" />
        </Form.Item>
        <Form.Item label="通知対象">
          <Form.List name="target">
            {(fields, {add, remove}) => (
              <Flex gap={16} vertical>
                {fields.map((field) => (
                  <Space key={field.key}>
                    <Form.Item name={field.name} noStyle rules={[{required: true}]}>
                      <TargetSpecifier />
                    </Form.Item>
                    <MinusCircleOutlined onClick={() => remove(field.name)} />
                  </Space>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    追加
                  </Button>
                </Form.Item>
              </Flex>
            )}
          </Form.List>
        </Form.Item>
        <Form.Item label="通知の種類">
          <Radio.Group defaultValue="markdown">
            <Radio.Button value="markdown">MD</Radio.Button>
          </Radio.Group>
        </Form.Item>

        <Form.Item name="markdown" label="markdownの内容">
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
