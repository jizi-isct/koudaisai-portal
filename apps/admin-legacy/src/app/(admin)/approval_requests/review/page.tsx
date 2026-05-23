"use client";
import {CheckOutlined, CloseOutlined} from "@ant-design/icons";
import {LoadingScreen} from "@koudaisai/shared-ui";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Button, Card, Flex, Form, message, Result, Tag} from 'antd';
import TextArea from "antd/es/input/TextArea";
import {useEffect, useState} from "react";
import {ViewPendingEditExhibitionInfoRequest} from "@/components/ViewPendingEditExhibitionInfoRequest";
import {$apiAdmin} from "@/lib/api";
import {getSearchParam} from "@/lib/browserNavigation";
import {$plansInfoApiNoLogin} from "@/lib/plansInfoApi";

type FormValues = {
  approvalReason: string | null
}

export default function Page() {
  const approvalRequestId = getSearchParam("approval_request_id")
  if (!approvalRequestId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="承認申請IDが指定されていません。URLに?approval_request_id=xxxxのように指定してください。"
        extra={
          <Button
            href={"/approval_requests"}
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
      <Inner approvalRequestId={approvalRequestId}/>
    </QueryClientProvider>
  )
}

function Inner({approvalRequestId}: { approvalRequestId: string }) {
  const [messageApi, contextHolder] = message.useMessage();
  const {data: approvalRequest, error, refetch} = $apiAdmin.useQuery(
    "get",
    "/approval-requests/{id}",
    {
      params: {
        path: {
          id: approvalRequestId
        }
      }
    }
  )
  const {data: issuer} = $apiAdmin.useQuery(
    "get",
    "/users/{user_id}",
    {
      enabled: !!approvalRequest?.issued_by,
      params: {
        path: {
          user_id: approvalRequest?.issued_by ?? ""
        }
      }
    }
  )
  const {data: basePlan, isLoading} = $plansInfoApiNoLogin.useQuery("get", "/plans/{planId}", {
    params: {
      path: {
        planId: issuer?.group_id ?? ""
      }
    },
    enabled: !!issuer?.group_id,
  })
  const {mutateAsync: mutateApprove} = $apiAdmin.useMutation(
    "post",
    "/approval-requests/{id}/approve"
  )
  const {mutateAsync: mutateReject} = $apiAdmin.useMutation(
    "post",
    "/approval-requests/{id}/reject"
  )
  const [approvalReason, setApprovalReason] = useState<string>("")
  useEffect(() => {
    setApprovalReason(approvalRequest?.approval_reason ?? "")
  }, [approvalRequest, setApprovalReason])

  if (isLoading) {
    return <LoadingScreen/>
  }

  if (!approvalRequest || !issuer || !basePlan) {
    return <Result
      status="error"
      title="データを取得できませんでした"
      subTitle={error}
      extra={
        <Button
          href={"/approval_requests"}
          type="primary"
        >
          戻る
        </Button>
      }
    >
    </Result>
  }

  const handleApprove = async () => {
    messageApi.loading({
      content: "処理中...",
      duration: 0,
    })
    let reqApprovalReason = null
    if (approvalReason !== "") {
      reqApprovalReason = approvalReason
    }
    try {
      await mutateApprove({
        params: {
          path: {
            id: approvalRequestId
          }
        },
        body: {
          approval_reason: reqApprovalReason
        }
      })
    } catch (e) {
      messageApi.destroy()
      messageApi.error("エラーが発生しました：" + JSON.stringify(e))
    }

    await refetch()
    messageApi.destroy()
    messageApi.success("処理が成功しました")
  }
  const handleReject = async () => {
    let reqApprovalReason = null
    if (approvalReason !== "") {
      reqApprovalReason = approvalReason
    }
    try {
      await mutateReject({
        params: {
          path: {
            id: approvalRequestId
          }
        },
        body: {
          approval_reason: reqApprovalReason
        }
      })
    } catch (e) {
      messageApi.destroy()
      messageApi.error("エラーが発生しました：" + JSON.stringify(e))
    }

    await refetch()
    messageApi.destroy()
    messageApi.success("処理が成功しました")
  }
  return (
    <>
      {contextHolder}
      <Form<FormValues>>
        <h1>承認申請の審査</h1>
        <Form.Item label={"種類"}>
          <Tag color={"orange"}>企画情報訂正申請</Tag>
        </Form.Item>

        <Form.Item label={"ステータス"}>
          {
            approvalRequest.status === "pending" ? <Tag color={"blue"}>審査中</Tag> :
              approvalRequest.status === "approved" ? <Tag color={"green"}>承認済み</Tag> :
                approvalRequest.status === "rejected" ? <Tag color={"red"}>却下済み</Tag> :
                  approvalRequest.status === "closed" ? <Tag color={"purple"}>取り下げ済み</Tag> :
                    <Tag color={"gray"}>不明</Tag>
          }
        </Form.Item>

        {
          approvalRequest?.type_edit_exhibition_info &&
                <Card
                        title={"申請内容"}
                        style={{margin: "16px 0"}}
                >
                  <ViewPendingEditExhibitionInfoRequest
                          approvalRequest={approvalRequest}
                          plan={basePlan}
                  />
                </Card>
        }
        <Card
          title={"申請事由"}
          style={{margin: "16px 0"}}
        >
          {
            approvalRequest.issue_reason
          }
        </Card>
        <Form.Item label={"理由(任意)"}>
          <TextArea
            value={approvalReason}
            onChange={(e) => {
              setApprovalReason(e.target.value)
            }}
            placeholder={"承認/却下の理由を入力してください"}
            disabled={approvalRequest.status !== "pending"}
          />
        </Form.Item>

        <Form.Item>
          <Flex gap={8}>
            <Button type="primary" htmlType="submit" onClick={handleApprove}
                    disabled={approvalRequest.status !== "pending"}>
              <CheckOutlined/> 承認
            </Button>
            <Button color={"danger"} variant={"solid"} htmlType="submit" onClick={handleReject}
                    disabled={approvalRequest.status !== "pending"}>
              <CloseOutlined/> 却下
            </Button>
            <Button type="default" href={".."}>
              戻る
            </Button>
          </Flex>
        </Form.Item>
      </Form>
    </>
  )
}
