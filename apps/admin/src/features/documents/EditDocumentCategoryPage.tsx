import {LoadingScreen} from "@koudaisai/shared-ui";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Button, Flex, Form, Input, message, Result} from "antd";
import {useEffect, useState} from "react";
import {$api} from "@/features/api/api";

type FormValues = {
  title: string;
  emoji: string | null;
};

export function EditDocumentCategoryPage() {
  const [queryClient] = useState(() => new QueryClient());
  const [categoryId, setCategoryId] = useState<string | null>();

  useEffect(() => {
    setCategoryId(new URLSearchParams(window.location.search).get("category_id"));
  }, []);

  if (categoryId === undefined) {
    return <LoadingScreen />;
  }

  if (!categoryId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="資料カテゴリIDが指定されていません。URLに?category_id=xxxxのように指定してください。"
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
      <EditDocumentCategoryForm categoryId={categoryId} />
    </QueryClientProvider>
  );
}

function EditDocumentCategoryForm({categoryId}: {categoryId: string}) {
  const [form] = Form.useForm();
  const [messageApi, contextHolder] = message.useMessage();
  const {data: documentCategoryRead, isLoading, error, refetch} = $api.useQuery(
    "get",
    "/document-categories/{category_id}",
    {
      params: {
        path: {
          category_id: categoryId,
        },
      },
    },
  );
  const {mutateAsync: mutateDocumentCategoryUpdate} = $api.useMutation(
    "patch",
    "/document-categories/{category_id}",
  );
  const [submitting, setSubmitting] = useState(false);

  if (isLoading) {
    return <LoadingScreen />;
  }

  if (!documentCategoryRead) {
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

  const handleSubmit = async (values: FormValues) => {
    setSubmitting(true);
    messageApi.loading("保存中...");
    try {
      await mutateDocumentCategoryUpdate({
        params: {
          path: {
            category_id: categoryId,
          },
        },
        body: {
          title: values.title,
          emoji: values.emoji ?? null,
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
    messageApi.success("保存しました");
    await refetch();
  };

  return (
    <>
      <Form
        onFinish={handleSubmit}
        initialValues={{
          title: documentCategoryRead.title,
          emoji: documentCategoryRead.emoji ?? null,
        }}
        form={form}
      >
        <h1>資料カテゴリを編集</h1>
        <Form.Item name="emoji" label="絵文字">
          <Input placeholder="アイコン絵文字を入力してください" addonBefore="絵文字" style={{width: "100%"}} />
        </Form.Item>

        <Form.Item name="title" label="タイトル" required>
          <Input placeholder="タイトルを入力してください" />
        </Form.Item>

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
