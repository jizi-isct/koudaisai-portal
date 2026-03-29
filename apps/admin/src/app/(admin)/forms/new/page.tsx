"use client";
import {MinusCircleOutlined, PlusOutlined} from "@ant-design/icons";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Button, Flex, Form, Input, message, Radio, Space} from 'antd';
import TextArea from "antd/es/input/TextArea";
import {useRouter} from "next/navigation";
import {useState} from "react";
import {TargetSpecifier} from "@/components/TargetSpecifier";
import {$apiAdmin, fetchClientAdmin} from "@/lib/api";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const [messageApi, contextHolder] = message.useMessage();
  const {mutateAsync: mutateFormCreate} = $apiAdmin.useMutation("post", "/forms")
  const [submitting, setSubmitting] = useState(false)
  const router = useRouter()
  const [form] = Form.useForm<{
    formName: string,
    summary: string,
    url: string,
    targets: string[][],
    dueDate: string | undefined,
  }>()
  const urlValue = Form.useWatch("url", form) ?? ""

  const handleSubmit = async ({formName, summary, url, targets, dueDate}: {
    formName: string,
    summary: string,
    url: string,
    targets: string[][],
    dueDate: string | undefined,
  }) => {
    setSubmitting(true)
    try {
      await mutateFormCreate({
        body: {
          form_name: formName,
          targets: targets.map((t) => t.join("/")),
          summary: summary,
          type_external: {
            form_url: url
          },
          due_date: dueDate ? new Date(dueDate).toISOString() : null
        }
      })
    } catch (e) {
      setSubmitting(false)
      messageApi.error("保存に失敗しました: " + e)
      return
    }
    setSubmitting(false)
    messageApi.success('保存しました')
    router.push("..")
  }

  const syncFormNameAndSummary = async () => {
    messageApi.loading('外部フォームのメタデータを取得中...')
    try {
      const result = await fetchClientAdmin.GET("/util/meta", {
        params: {query: {url: urlValue}}
      })
      if (!result.data) {
        messageApi.destroy()
        messageApi.error("外部フォームのメタデータを取得できませんでした: " + result.error)
        return
      }
      if (!result.data.title || !result.data.description) {
        messageApi.destroy()
        messageApi.warning("外部フォームのメタデータにタイトルまたは要約が含まれていません")
      } else {
        messageApi.destroy()
        messageApi.success('外部フォームのメタデータを取得しました')
      }
      if (result.data.title) {
        form.setFieldsValue({formName: result.data.title})
      }
      if (result.data.description) {
        form.setFieldsValue({summary: result.data.description})
      }
    } catch (e) {
      messageApi.destroy()
      messageApi.error("外部フォームのメタデータのfetchに失敗しました: " + e)
    }
  }

  return (
    <>
      <Form
        form={form}
        onFinish={handleSubmit}
        initialValues={{targets: [], formType: "external"}}
      >
        <h1>新規フォームを作成</h1>
        <Form.Item name="formName" label={"フォーム名"} rules={[{required: true}]}>
          <Input placeholder={"フォーム名を入力してください"}/>
        </Form.Item>

        <Form.Item name="summary" label={"要約"} rules={[{required: true}]}>
          <TextArea placeholder={"フォームの要約を入力してください"}/>
        </Form.Item>

        <Form.Item label={"対象"} rules={[{required: true}]}>
          <Form.List name={"targets"}>
            {(fields, {add, remove}) => (
              <Flex gap={16} vertical>
                {fields.map((field) => (
                  <Space key={field.key}>
                    <TargetSpecifier name={field.name.toString()} onChange={() => { return }}/>
                    <MinusCircleOutlined onClick={() => { remove(field.name); }}/>
                  </Space>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined/>}>追加</Button>
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

        <Form.Item name="url" label="外部フォームurl" rules={[{required: true}]}>
          <Input/>
        </Form.Item>
        <Form.Item>
          <Button type="default" onClick={syncFormNameAndSummary}>フォーム名と要約を自動取得</Button>
        </Form.Item>

        <Form.Item name="dueDate" label="回答期限">
          <Input type="datetime-local"/>
        </Form.Item>

        <Form.Item>
          <Flex gap={8}>
            <Button type="primary" htmlType="submit" disabled={submitting}>送信</Button>
            <Button type="default" href={".."}>戻る</Button>
          </Flex>
        </Form.Item>
      </Form>
      {contextHolder}
    </>
  )
}
