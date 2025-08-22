"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1, LoadingScreen} from "@/components/generic";
import {$apiAdmin, ApprovalRequestRead, getFilesRedirectUrl} from "@/lib";
import {Button, Flex, Popconfirm, Table, TableProps, Tag} from "antd";
import {CheckOutlined, CloseOutlined} from "@ant-design/icons";
import {ReactNode, useMemo} from "react";
import Image from "next/image";

type ExpandedRowDataType = {
  key: string,
  value: ReactNode;
}

const expandedColumns: TableProps<ExpandedRowDataType>['columns'] = [
  {
    key: "key",
    title: "項目名",
    dataIndex: "key",
    rowScope: "row",
    render: (value, record, _index) => <span>
        {value}
      </span>
  }, {
    key: "value",
    title: "項目",
    dataIndex: "value",
    rowScope: "row",
    render: (value, record, _index) => <span>
        {value}
      </span>
  },
]

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const {data, isLoading, refetch} = $apiAdmin.useQuery("get", "/approval-requests")
  const {data: users} = $apiAdmin.useQuery("get", "/users")
  const userNameMap = useMemo(() => {
    if (!users) return {};
    return users.reduce((acc, user) => {
      acc[user.id] = user.group_id + "の" + user.name;
      return acc;
    }, {} as Record<string, string>);
  }, [users])
  const {mutateAsync: mutateApprove} = $apiAdmin.useMutation("post", "/approval-requests/{id}/approve");
  const {mutateAsync: mutateReject} = $apiAdmin.useMutation("post", "/approval-requests/{id}/reject");

  const handleApprove = (id: string) => async () => {
    await mutateApprove({
      params: {
        path: {
          id: id
        }
      }
    })
    await refetch()
  }

  const handleReject = (id: string) => async () => {
    await mutateReject({
      params: {
        path: {
          id: id
        }
      }
    })
    await refetch()
  }

  const expandedRowRender = (record: ApprovalRequestRead) => {
    const dataSource = [];
    dataSource.push({
      key: "企画",
      value: record.type_edit_exhibition_info.plan_name === undefined ? "変更なし" : record.type_edit_exhibition_info.plan_name
    })
    if (record.type_edit_exhibition_info.icon_key) {
      dataSource.push({
        key: "アイコン",
        value: <Image
          src={getFilesRedirectUrl(record.type_edit_exhibition_info.icon_key)}
          alt={""}
          width={128}
          height={128}
        />
      })
    } else if (record.type_edit_exhibition_info.icon_key === undefined) {
      dataSource.push({
        key: "アイコン",
        value: "変更なし"
      })
    }
    if (record.type_edit_exhibition_info.description) {
      dataSource.push({
        key: "説明",
        value: record.type_edit_exhibition_info.description
      })
    } else {
      dataSource.push({
        key: "説明",
        value: "変更なし"
      })
    }
    if (record.type_edit_exhibition_info.is_child_friendly) {
      dataSource.push({
        key: "子供向け企画か否か",
        value: record.type_edit_exhibition_info.is_child_friendly
      })
    } else {
      dataSource.push({
        key: "子供向け企画か否か",
        value: "変更なし"
      })
    }
    return (
      <Table<ExpandedRowDataType>
        columns={expandedColumns}
        dataSource={dataSource}
        pagination={false}
      />
    )
  }

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
      render: (value, record) => {
        if (record.status == "pending") {
          return (<Flex gap={5}>
            <Popconfirm
              title={"承認"}
              description="この承認申請を本当に承認しますか？この操作は元に戻せませんよ！！"
              onConfirm={handleApprove(value)}
              onCancel={() => {
                return
              }}
              okText="はい"
              cancelText="いいえ"
            >
              <Button><CheckOutlined/></Button>
            </Popconfirm>
            <Popconfirm
              title={"却下"}
              description="この承認申請を本当に却下しますか？この操作は元に戻せません！！"
              onConfirm={handleReject(value)}
              onCancel={() => {
                return
              }}
              okText="はい"
              cancelText="いいえ"
            >
              <Button danger><CloseOutlined/></Button>
            </Popconfirm>
          </Flex>)
        } else {
          return <span>操作不可</span>;
        }
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
          expandable={{expandedRowRender}}
          scroll={{x: 'max-content'}}
        />
      </Flex>
    </>
  );
}