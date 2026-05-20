"use client";
import {MinusCircleOutlined, PlusOutlined, UploadOutlined} from "@ant-design/icons";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Button, Flex, Form, Input, message, Radio, Result, Select, Space, Upload, type UploadFile,} from 'antd';
import TextArea from "antd/es/input/TextArea";
import {useMemo, useState} from "react";
import {TargetSpecifier} from "@/components/TargetSpecifier";
import {$apiAdmin} from "@/lib/api";
import {getSearchParam, navigateTo} from "@/lib/browserNavigation";


type FormValues = {
  title: string;
  category: string;
  targets: string[][];
  documentFormat: "pdf";
  pdfFile: UploadFile[];
} | {
  title: string;
  category: string;
  targets: string[][];
  documentFormat: "markdown";
  markdownContent: string;
} | {
  title: string;
  category: string;
  targets: string[][];
  documentFormat: "misc";
  miscFile: UploadFile[];
}

export default function Page() {
  const categoryId = getSearchParam("category_id")
  if (!categoryId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="カテゴリーIDが指定されていません。URLに?category_id=xxxxのように指定してください。"
        extra={
          <Button
            href={"/documents"}
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
      <Inner categoryId={categoryId}/>
    </QueryClientProvider>
  )
}

function Inner({categoryId}: { categoryId: string }) {
  const [form] = Form.useForm();
  const [messageApi, contextHolder] = message.useMessage();
  const {mutateAsync: mutateDocumentCreate} = $apiAdmin.useMutation(
    "post",
    "/documents"
  )
  const {mutateAsync: mutateUploadFile} = $apiAdmin.useMutation("post", "/files/upload")
  const {data: categories} = $apiAdmin.useQuery(
    "get",
    "/document-categories"
  )
  const categoryOptions = useMemo(() => {
    return categories?.map((category) => {
      return {
        value: category.id,
        label: category.title
      }
    }) ?? []
  }, [categories])
  const [submitting, setSubmitting] = useState(false)
  const documentFormat = Form.useWatch("documentFormat", form)

  const handleSubmit = async (values: FormValues) => {
    setSubmitting(true)
    try {
      switch (values.documentFormat) {
        case "pdf": {
          const {presigned_url, key} = await mutateUploadFile({
            body: {
              file_name: values.pdfFile[0].name
            }
          })

          await fetch(presigned_url, {
            method: "PUT",
            body: values.pdfFile[0].originFileObj
          })

          await mutateDocumentCreate({
            body: {
              title: values.title,
              category: values.category,
              targets: values.targets.map((t) => t.join("/")),
              format_pdf: {
                file_name: values.pdfFile[0].name,
                file_key: key
              }
            }
          })
          break;
        }
        case "markdown":
          await mutateDocumentCreate({
            body: {
              title: values.title,
              category: values.category,
              targets: values.targets.map((t) => t.join("/")),
              format_markdown: {
                content: values.markdownContent
              }
            }
          })
          break;
        case "misc": {
          const {presigned_url, key} = await mutateUploadFile({
            body: {
              file_name: values.miscFile[0].name
            }
          })

          await fetch(presigned_url, {
            method: "PUT",
            body: values.miscFile[0].originFileObj
          })

          await mutateDocumentCreate({
            body: {
              title: values.title,
              category: values.category,
              targets: values.targets.map((t) => t.join("/")),
              format_misc: {
                file_name: values.miscFile[0].name,
                file_key: key
              }
            }
          })
          break;
        }
      }
    } catch (e) {
      setSubmitting(false)
      messageApi.error("保存に失敗しました: " + e)
      return
    }
    setSubmitting(false)
    messageApi.success('保存しました')
    navigateTo("..")
  }

  return (
    <>
      <Form
        onFinish={handleSubmit}
        form={form}
        initialValues={{
          category: categoryId,
          documentFormat: "pdf",
          targets: [["group", "type", "press"]]
        }}
      >
        <h1>新規資料を作成</h1>
        <Form.Item
          name={"title"}
          label={"タイトル"}
          rules={[{required: true}]}
        >
          <Input
            placeholder={"タイトルを入力してください"}
          />
        </Form.Item>

        <Form.Item
          name="category"
          label={"カテゴリー"}
          rules={[{required: true}]}
        >
          <Select options={categoryOptions}/>
        </Form.Item>

        <Form.Item label={"対象"} rules={[{required: true}]}>
          <Form.List name={"targets"}>
            {(fields, {add, remove}) => (
              <Flex gap={16} vertical>
                {fields.map((field) => (
                  <Space key={field.key}>
                    <Form.Item name={field.name} noStyle rules={[{required: true}]}>
                      <TargetSpecifier/>
                    </Form.Item>
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

        <Form.Item label="資料のフォーマット" name={"documentFormat"} rules={[{required: true}]}>
          <Radio.Group>
            <Radio.Button value="pdf">PDF</Radio.Button>
            <Radio.Button value="markdown">Markdown</Radio.Button>
            <Radio.Button value="misc">その他</Radio.Button>
          </Radio.Group>
        </Form.Item>

        {
          documentFormat == "pdf" &&
                <Form.Item
                        label="PDFファイル"
                        rules={[{required: true}]}
                        name={"pdfFile"}
                        valuePropName={"fileList"}
                        getValueFromEvent={e => Array.isArray(e) ? e : e && e.fileList}
                >
                  <Upload
                          accept="application/pdf"
                          beforeUpload={() => {
                            return false; // Prevent automatic upload
                          }}
                          maxCount={1}
                  >
                    <Button>
                      <UploadOutlined/>
                      PDFファイルをアップロード
                    </Button>
                  </Upload>
                </Form.Item>
        }

        {
          documentFormat == "markdown" &&
                <Form.Item label="markdown" name="markdownContent" rules={[{required: true}]}>
                  <TextArea
                          rows={10}
                          placeholder={"Markdown形式で資料の内容を入力してください"}
                          disabled={documentFormat !== "markdown"}
                  />
                </Form.Item>
        }

        {
          documentFormat == "misc" &&
                <Form.Item
                        label="ファイル"
                        rules={[{required: true}]}
                        name={"miscFile"}
                        valuePropName={"fileList"}
                        getValueFromEvent={e => Array.isArray(e) ? e : e && e.fileList}
                >
                  <Upload
                          beforeUpload={() => {
                            return false; // Prevent automatic upload
                          }}
                          maxCount={1}
                  >
                    <Button>
                      <UploadOutlined/>
                      ファイルをアップロード
                    </Button>
                  </Upload>
                </Form.Item>
        }

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
