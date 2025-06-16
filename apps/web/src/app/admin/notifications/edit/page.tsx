"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {useSearchParams} from "next/navigation";
import {Button, Flex, Form, Input, message, Radio, Result, Space} from 'antd';
import {TargetSpecifier} from "@/components/common/TargetSpecifier";
import {useMemo, useState} from "react";
import {MinusCircleOutlined, PlusOutlined} from "@ant-design/icons";
import TextArea from "antd/es/input/TextArea";
import {$apiAdmin} from "@/lib";
import {LoadingScreen} from "@/components/generic";

export default function Page() {
  const searchParams = useSearchParams()
  const notificationId = searchParams.get("notification_id")
  if (!notificationId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="通知IDが指定されていません。URLに?notification_id=xxxxのように指定してください。"
        extra={
          <Button
            href={process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/notifications"}
            type="primary"
          >
            戻る
          </Button>
        }
      >
      </Result>
    )
  }
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner notificationId={notificationId}/>
    </QueryClientProvider>
  )
}

function Inner({notificationId}: { notificationId: string }) {
  const [messageApi, contextHolder] = message.useMessage();
  const {data, isLoading, error} = $apiAdmin.useQuery(
    "get",
    "/notifications/{notification_id}",
    {
      params: {
        path: {
          notification_id: notificationId
        }
      }
    }
  )
  const {mutateAsync: mutateNotificationUpdate} = $apiAdmin.useMutation(
    "patch",
    "/notifications/{notification_id}"
  )
  const [submitting, setSubmitting] = useState(false)

  const formType = useMemo(() => {
    if (data?.type_markdown) {
      return "markdown"
    }
    return "markdown"
  }, [data])

  if (isLoading) {
    return <LoadingScreen/>
  }

  if (!data) {
    return <Result
      status="error"
      title="データを取得できませんでした"
      subTitle={error}
      extra={
        <Button
          href={process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/notifications"}
          type="primary"
        >
          戻る
        </Button>
      }
    >
    </Result>
  }

  const handleSubmit = async ({title, target}: { title: string | undefined, target: string[][] | undefined }) => {
    setSubmitting(true)
    await mutateNotificationUpdate({
        params: {
          path: {
            notification_id: notificationId
          }
        },
        body: {
          title: title,
          target: target?.map((t) => t.join("/"))
        }
      }
    )
    setSubmitting(false)
    messageApi.success('保存しました')
  }

  return (
    <>
      <Form
        onFinish={handleSubmit}
        initialValues={{
          title: data.title,
          target: data.target.map((t) => t.split("/")),
          markdown: data.type_markdown.content
        }}
      >
        <h1>通知を編集</h1>
        <Form.Item name={"title"} label={"タイトル"}>
          <Input
            placeholder={"タイトルを入力してください"}
          />
        </Form.Item>
        <Form.Item label={"通知対象"}>
          <Form.List name={"target"}>
            {(fields, {add, remove}) => (
              <Flex gap={16} vertical>
                {fields.map((field) => (
                  <Space key={field.key}>
                    <TargetSpecifier name={field.name.toString()} onChange={() => {
                      return
                    }}/>
                    <MinusCircleOutlined
                      onClick={() => {
                        remove(field.name);
                      }}
                    />
                  </Space>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined/>}>
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

        <Form.Item name="markdown" label="markdownの内容">
          <TextArea defaultValue={data.type_markdown.content}/>
        </Form.Item>

        <Form.Item>
          <Flex gap={8}>
            <Button type="primary" htmlType="submit" disabled={submitting}>
              送信
            </Button>
            <Button type="default" href={".."}>
              戻る
            </Button>
          </Flex>
        </Form.Item>
      </Form>
      {contextHolder}
    </>
  )
}