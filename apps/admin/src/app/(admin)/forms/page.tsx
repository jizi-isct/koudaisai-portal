"use client";
import {DeleteOutlined, PlusOutlined} from "@ant-design/icons";
import {FormRead} from "@koudaisai/shared-types";
import {Heading1, LoadingScreen} from "@koudaisai/shared-ui";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Button, Flex, message, Popconfirm, Table, TableProps, Tag} from "antd";
import {$apiAdmin} from "@/lib/api";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const [messageApi, contextHolder] = message.useMessage();
  const {data, isLoading, refetch} = $apiAdmin.useQuery("get", "/forms")
  const {mutateAsync: mutateFormDelete} = $apiAdmin.useMutation("delete", "/forms/{form_id}");

  const handleDelete = (id: string) => async () => {
    messageApi.loading("削除中...")
    try {
      await mutateFormDelete({
        params: {
          path: {
            form_id: id
          }
        }
      })
      messageApi.destroy()
      messageApi.success("削除しました");
    } catch (e) {
      messageApi.destroy()
      messageApi.error("削除に失敗しました: " + e);
      return;
    } finally {
      await refetch()
    }
  }

  const columns: TableProps<FormRead>['columns'] = [
    {
      key: "title",
      title: "タイトル",
      dataIndex: "form_name",
      rowScope: "row",
      render: (value, record) => <a
        style={{textDecoration: "underline"}}
        href={"/forms/edit?form_id=" + record.id}
      >
        {value}
      </a>
    },
    {
      key: "type",
      title: "種類",
      render: (_value, record) => {
        if ('type_external' in record) {
          return <Tag color={"green"}>外部</Tag>;
        } else if ('type_builtin' in record) {
          return <Tag color={"blue"}>ビルトイン</Tag>
        } else {
          return <Tag color={"blue"}>不明</Tag>;
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
          title={"フォームを削除"}
          description="あなたは本当にこのフォームを削除する気ですか！？"
          onConfirm={handleDelete(value)}
          onCancel={() => { return }}
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
      <Heading1 emoji={"📃"}>フォーム管理画面</Heading1>
      <Flex gap={8} vertical>
        <Button style={{width: "fit-content"}} href={"/forms/new"}><PlusOutlined/>新規作成</Button>
        <Table<FormRead>
          dataSource={data.map(item => ({...item, key: item.id}))}
          columns={columns}
          bordered
          scroll={{x: 'max-content'}}
        />
      </Flex>
      {contextHolder}
    </>
  );
}
