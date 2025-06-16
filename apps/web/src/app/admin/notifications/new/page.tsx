"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Button, Flex, Form, Input, message, Radio, Space} from 'antd';
import {TargetSpecifier} from "@/components/common/TargetSpecifier";
import {useState} from "react";
import {MinusCircleOutlined, PlusOutlined} from "@ant-design/icons";
import TextArea from "antd/es/input/TextArea";
import {$apiAdmin} from "@/lib";
import {useRouter} from "next/navigation";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const [messageApi, contextHolder] = message.useMessage();
  const {mutateAsync: mutateNotificationCreate} = $apiAdmin.useMutation(
    "post",
    "/notifications"
  )
  const [submitting, setSubmitting] = useState(false)
  const formType = "markdown"
  const router = useRouter()

  const handleSubmit = async ({title, target, markdown}: { title: string, target: string[][], markdown: string }) => {
    setSubmitting(true)
    try {
      await mutateNotificationCreate({
          body: {
            title: title,
            target: target.map((t) => t.join("/")),
            type_markdown: {
              content: markdown
            }
          }
        }
      )
    } catch (e) {
      setSubmitting(false)
      messageApi.error("保存に失敗しました: " + e)
      return
    }
    setSubmitting(false)
    messageApi.success('保存しました')
    router.push("..")
  }

  return (
    <>
      <Form
        onFinish={handleSubmit}
        initialValues={
          {
            target: []
          }
        }
      >
        <h1>新規通知を作成</h1>
        <Form.Item name={"title"} label={"タイトル"} rules={[{required: true}]}>
          <Input
            placeholder={"タイトルを入力してください"}
          />
        </Form.Item>

        <Form.Item label={"通知対象"} rules={[{required: true}]}>
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

        <Form.Item name="markdown" label="markdownの内容" rules={[{required: true}]}>
          <TextArea/>
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