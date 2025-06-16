"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1, LoadingScreen} from "@/components/generic";
import {$apiAdmin, NotificationRead} from "@/lib";
import {Button, Flex, Popconfirm, Table, TableProps, Tag} from "antd";
import {DeleteOutlined, PlusOutlined} from "@ant-design/icons";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const {data, isLoading, refetch} = $apiAdmin.useQuery("get", "/notifications")
  const {mutateAsync: mutateNotificationDelete} = $apiAdmin.useMutation("delete", "/notifications/{notification_id}");

  const handleDelete = (id: string) => async () => {
    await mutateNotificationDelete({
      params: {
        path: {
          notification_id: id
        }
      }
    })
    await refetch()
  }

  const columns: TableProps<NotificationRead>['columns'] = [
    {
      key: "title",
      title: "タイトル",
      dataIndex: "title",
      rowScope: "row",
      render: (value, record, _index) => <a
        style={{textDecoration: "underline"}}
        href={process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/notifications/edit?notification_id=" + record.id}
      >
        {value}
      </a>
    },
    {
      key: "type",
      title: "種類",
      render: (_value, record, _index) => {
        if (record.type_markdown) {
          return <Tag color={"green"}>MD</Tag>;
        } else {
          return <Tag color={"red"}>不明</Tag>
        }
      }
    },
    {
      key: "created_at",
      title: "作成日時",
      dataIndex: "created_at",
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
      key: "updated_at",
      title: "更新日時",
      dataIndex: "updated_at",
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
      key: "created_by",
      title: "作成者",
      dataIndex: "created_by",
      rowScope: "row"
    },
    {
      key: "updated_by",
      title: "更新者",
      dataIndex: "updated_by",
      rowScope: "row"
    },
    {
      key: "actions",
      title: '',
      dataIndex: 'id',
      fixed: 'right',
      render: (value) => <Flex gap={5}>
        <Popconfirm
          title={"通知を削除"}
          description="あなたは本当にこの通知を削除する気ですか！？"
          onConfirm={handleDelete(value)}
          onCancel={() => {
            return
          }}
          okText="はい"
          cancelText="いいえ"
        >
          <Button danger><DeleteOutlined/></Button>
        </Popconfirm>
      </Flex>,
    },
  ]

  if (isLoading) return <LoadingScreen/>;
  if (!data) return <Heading1 emoji={"⚠️"}>エラーです</Heading1>;
  return (
    <>
      <Heading1 emoji={"🔔"}>通知管理画面</Heading1>
      <Flex gap={8} vertical>
        <Button style={{width: "fit-content"}}
                href={process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/notifications/new"}><PlusOutlined/>新規作成</Button>
        <Table<NotificationRead>
          dataSource={data.map(item => ({...item, key: item.id}))}
          columns={columns}
          bordered
          scroll={{x: 'max-content'}}
        />
      </Flex>
    </>
  );
}