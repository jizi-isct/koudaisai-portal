"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {useSearchParams} from "next/navigation";
import {Button, Flex, Form, Input, message, Result, Select, Space, Tag, Upload, UploadFile} from 'antd';
import {TargetSpecifier} from "@/components/common/TargetSpecifier";
import {useMemo, useState} from "react";
import {MinusCircleOutlined, PlusOutlined, UploadOutlined} from "@ant-design/icons";
import TextArea from "antd/es/input/TextArea";
import {$apiAdmin} from "@/lib";
import {LoadingScreen} from "@/components/generic";

type FormValues = {
  title: string;
  category: string;
  targets: string[][];
  documentFormat: "pdf";
  pdfFile?: UploadFile[];
  markdownContent?: string;
  miscFile?: UploadFile[];
}

export default function Page() {
  const searchParams = useSearchParams()
  const documentId = searchParams.get("document_id")
  if (!documentId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="資料IDが指定されていません。URLに?document_id=xxxxのように指定してください。"
        extra={
          <Button
            href={process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/documents"}
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
      <Inner documentId={documentId}/>
    </QueryClientProvider>
  )
}

function Inner({documentId}: { documentId: string }) {
  const [form] = Form.useForm()
  const [messageApi, contextHolder] = message.useMessage();
  const {data: documentRead, isLoading: isLoadingDocuments, error, refetch: refetchDocument} = $apiAdmin.useQuery(
    "get",
    "/documents/{document_id}",
    {
      params: {
        path: {
          document_id: documentId
        }
      }
    }
  )

  const {mutateAsync: mutateUploadFile} = $apiAdmin.useMutation("post", "/files/upload")
  const {data: categories, isLoading: isLoadingCategories} = $apiAdmin.useQuery(
    "get",
    "/document-categories"
  )
  const {mutateAsync: mutateDocumentUpdate} = $apiAdmin.useMutation(
    "patch",
    "/documents/{document_id}"
  )
  const [submitting, setSubmitting] = useState(false)


  let documentFormat = "misc"
  if (documentRead?.format_pdf) {
    documentFormat = "pdf"
  } else if (documentRead?.format_misc) {
    documentFormat = "misc"
  } else if (documentRead?.format_markdown) {
    documentFormat = "markdown"
  }

  const categoryOptions = useMemo(() => {
    return categories?.map((category) => {
      return {
        value: category.id,
        label: category.title
      }
    }) ?? []
  }, [categories])

  if (isLoadingDocuments || isLoadingCategories) {
    return <LoadingScreen/>
  }


  if (!documentRead || !categories) {
    return <Result
      status="error"
      title="データを取得できませんでした"
      subTitle={error}
      extra={
        <Button
          href={process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/documents"}
          type="primary"
        >
          戻る
        </Button>
      }
    >
    </Result>
  }

  const handleSubmit = async (values: FormValues) => {
    setSubmitting(true)
    messageApi.loading("保存中...")
    try {
      if (documentRead.format_pdf) {
        // PDFファイルをアップロード。変更されていない場合は何もしない
        let formatPdf = undefined
        if (values.pdfFile && values.pdfFile[0].uid !== "default") {
          const {presigned_url, key} = await mutateUploadFile({
            body: {
              file_name: values.pdfFile[0].name
            }
          })

          await fetch(presigned_url, {
            method: "PUT",
            body: values.pdfFile[0].originFileObj
          })

          formatPdf = {
            file_name: values.pdfFile[0].name,
            file_key: key
          }
        }

        // 資料を更新
        await mutateDocumentUpdate({
          params: {
            path: {
              document_id: documentId
            }
          },
          body: {
            title: values.title,
            category: values.category,
            targets: values.targets.map((t) => t.join("/")),
            format_pdf: formatPdf
          }
        })
      } else if (documentRead.format_markdown) {
        // Markdown形式の資料を更新
        await mutateDocumentUpdate({
          params: {
            path: {
              document_id: documentId
            }
          },
          body: {
            title: values.title,
            category: values.category,
            targets: values.targets.map((t) => t.join("/")),
            format_markdown: values.markdownContent ? {
              content: values.markdownContent
            } : undefined,
          }
        })
      } else if (documentRead.format_misc) {
        // その他のファイルをアップロード。変更されていない場合は何もしない
        let formatMisc = undefined
        if (values.miscFile && values.miscFile[0].uid !== "default") {
          const {presigned_url, key} = await mutateUploadFile({
            body: {
              file_name: values.miscFile[0].name
            }
          })

          await fetch(presigned_url, {
            method: "PUT",
            body: values.miscFile[0].originFileObj
          })

          formatMisc = {
            file_name: values.miscFile[0].name,
            file_key: key
          }
        }

        // 資料を更新
        await mutateDocumentUpdate({
          params: {
            path: {
              document_id: documentId
            }
          },
          body: {
            title: values.title,
            category: values.category,
            targets: values.targets.map((t) => t.join("/")),
            format_misc: formatMisc,
          }
        })
      }
    } catch (e) {
      setSubmitting(false)
      messageApi.destroy()
      messageApi.error("保存に失敗しました: " + e)
      return
    }

    setSubmitting(false)
    messageApi.destroy()
    messageApi.success('保存しました')
    await refetchDocument()
  }

  return (
    <>
      <Form
        onFinish={handleSubmit}
        initialValues={{
          title: documentRead.title,
          category: documentRead.category,
          targets: documentRead.targets.map((t) => t.split("/")),
          pdfFile: documentRead.format_pdf ? [{
            uid: "default",
            name: documentRead.format_pdf?.file_name,
            status: "done"
          }] : [],
          miscFile: documentRead.format_misc ? [{
            uid: "default",
            name: documentRead.format_misc?.file_name,
            status: "done"
          }] : [],
        }}
        form={form}
      >
        <h1>資料を編集</h1>
        <Form.Item name={"title"} label={"タイトル"} required={true}>
          <Input
            placeholder={"タイトルを入力してください"}
          />
        </Form.Item>

        <Form.Item name={"category"} label={"カテゴリー"} required={true}>
          <Select options={categoryOptions}/>
        </Form.Item>

        <Form.Item label={"対象"} required={true}>
          <Form.List name={"targets"}>
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

        <Form.Item label="資料のフォーマット">
          {documentRead.format_pdf && <Tag color={"orange"}>PDF</Tag>}
          {documentRead.format_markdown && <Tag color={"green"}>Markdown</Tag>}
          {documentRead.format_misc && <Tag color={"blue"}>その他</Tag>}
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
                <Form.Item label="markdown" rules={[{required: true}]}>
                  <TextArea
                          rows={10}
                          placeholder={"Markdown形式で資料の内容を入力してください"}
                          name="markdownContent"
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