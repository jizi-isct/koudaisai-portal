"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1, LoadingScreen} from "@/components/generic";
import {$apiAdmin, ApprovalRequestRead} from "@/lib";
import {Button, Flex, Table, TableProps, Tag, Tooltip} from "antd";
import {MoreOutlined} from "@ant-design/icons";
import {useMemo} from "react";


export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const {data, isLoading} = $apiAdmin.useQuery("get", "/approval-requests")
  const {data: users} = $apiAdmin.useQuery("get", "/users")
  const userNameMap = useMemo(() => {
    if (!users) return {};
    return users.reduce((acc, user) => {
      acc[user.id] = user.group_id + "の" + user.name;
      return acc;
    }, {} as Record<string, string>);
  }, [users])

  const columns: TableProps<ApprovalRequestRead>['columns'] = [
    {
      key: "issued_by",
      title: "申請者",
      dataIndex: "issued_by",
      rowScope: "row",
      render: (value, record, _index) => <span>
        {userNameMap[value] || value}
      </span>
    },
    {
      key: "type",
      title: "種類",
      render: (_value, record, _index) => {
        if (record.type_edit_exhibition_info) {
          return <Tag color={"green"}>企画情報更新</Tag>;
        } else {
          return <Tag color={"red"}>不明</Tag>;
        }
      }
    },
    {
      key: "status",
      title: "ステータス",
      dataIndex: "status",
      rowScope: "row",
      render: (value) => {
        switch (value) {
          case "pending":
            return <Tag color={"blue"}>審査中</Tag>;
          case "approved":
            return <Tag color={"green"}>承認済み</Tag>;
          case "rejected":
            return <Tag color={"red"}>却下済み</Tag>;
          case "closed":
            return <Tag color={"purple"}>取り下げ済み</Tag>
          default:
            return <Tag color={"grey"}>不明</Tag>;
        }
      }
    },
    {
      key: "issued_at",
      title: "発行日時",
      dataIndex: "issued_at",
      rowScope: "row",
      render: value => new Date(value).toLocaleString("ja-JP", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit"
      })
    },
    {
      key: "actions",
      title: '操作',
      dataIndex: 'id',
      fixed: 'right',
      render: (value) => {
        return <Tooltip
          title={"詳細"}
            >
          <Button
            href={process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/approval_requests/review?approval_request_id=" + value}
          >
            <MoreOutlined/> 詳細
          </Button>
        </Tooltip>
      },
    },
  ]


  if (isLoading) return <LoadingScreen/>;
  if (!data) return <Heading1 emoji={"⚠️"}>エラーです</Heading1>;
  return (
    <>
      <Heading1 emoji={"📃"}>承認申請一覧</Heading1>
      <Flex gap={8} vertical>
        <Table<ApprovalRequestRead>
          dataSource={data.map(item => ({...item, key: item.id}))}
          columns={columns}
          bordered
          scroll={{x: 'max-content'}}
        />
      </Flex>
    </>
  );
}