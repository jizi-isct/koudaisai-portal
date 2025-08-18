"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1, LoadingScreen} from "@/components/generic";
import {Button, Checkbox, Flex, Popconfirm, Table, TableProps, Tag, Tooltip} from "antd";
import {DeleteOutlined} from "@ant-design/icons";
import {$plansInfoApi} from "@/lib/plansInfoApi";
import {BasePlanRead} from "@/lib/plansInfoTypes";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const {data, isLoading, refetch} = $plansInfoApi.useQuery("get", "/plans")
  const {mutateAsync: mutatePlanDelete} = $plansInfoApi.useMutation("delete", "/plans/{planId}")

  const handleDelete = (id: string) => async () => {
    await mutatePlanDelete({
      params: {
        path: {
          planId: id
        }
      }
    })
    await refetch()
  }

  const columns: TableProps<BasePlanRead>['columns'] = [
    {
      key: "id",
      title: <Tooltip title={"id"}>企画番号</Tooltip>,
      dataIndex: "id",
      rowScope: "row",
    },
    {
      key: "type",
      title: <Tooltip title={"type"}>種類</Tooltip>,
      dataIndex: "type",
      filters: [
        {
          text: "模擬店企画",
          value: "booth"
        },
        {
          text: "一般企画",
          value: "general"
        },
        {
          text: "ステージ企画",
          value: "stage"
        },
        {
          text: "研究室公開企画",
          value: "labo"
        }
      ],
      onFilter: (value, record) => record.type === value,
      render: value => {
        switch (value) {
          case "booth":
            return <Tooltip title={"booth"}><Tag color={"red"}>模擬店企画</Tag></Tooltip>
          case "general":
            return <Tooltip title={"general"}><Tag color={"blue"}>一般企画</Tag></Tooltip>
          case "stage":
            return <Tooltip title={"stage"}><Tag color={"green"}>ステージ企画</Tag></Tooltip>
          case "labo":
            return <Tooltip title={"labo"}><Tag color={"orange"}>研究室公開企画</Tag></Tooltip>
          default:
            return <Tag color={"warning"}>不明</Tag>
        }
      }
    },
    {
      key: "organization_name",
      title: <Tooltip title={"organization_name"}>団体名</Tooltip>,
      dataIndex: "organization_name",
      rowScope: "row",
    },
    {
      key: "plan_name",
      title: <Tooltip title={"plan_name"}>企画名</Tooltip>,
      dataIndex: "plan_name",
      rowScope: "row"
    },
    {
      key: "description",
      title: <Tooltip title={"description"}>概要</Tooltip>,
      dataIndex: "description",
      rowScope: "row"
    },
    {
      key: "is_child_friendly",
      title: <Tooltip title={"is_child_friendly"}>子供向け企画?</Tooltip>,
      dataIndex: "is_child_friendly",
      rowScope: "row",
      render: (_value, record, _index) => {
        if (record.is_child_friendly) {
          return <Tooltip title={"true"}><Checkbox value={true} disabled={true}/></Tooltip>
        } else {
          return <Tooltip title={"false"}><Checkbox value={false} disabled={true}/></Tooltip>
        }
      }
    },
    {
      key: "is_recommended",
      title: <Tooltip title={"is_recommended"}>おすすめ企画?</Tooltip>,
      dataIndex: "is_recommended",
      rowScope: "row",
      render: (_value, record, _index) => {
        if (record.is_recommended) {
          return <Tooltip title={"true"}><Checkbox value={true} disabled={true}/></Tooltip>
        } else {
          return <Tooltip title={"false"}><Checkbox value={false} disabled={true}/></Tooltip>
        }
      }
    },
    {
      key: "actions",
      title: '操作',
      dataIndex: 'id',
      fixed: 'right',
      render: (value) => <Flex gap={5}>
        <Popconfirm
          title={"企画情報を削除"}
          description="あなたは本当にこの企画情報を削除する気ですか！？"
          onConfirm={handleDelete(value)}
          onCancel={() => {
            return
          }}
          okText="はい"
          cancelText="いいえ"
        >
          <Tooltip title={"削除"}>
            <Button danger><DeleteOutlined/></Button>
          </Tooltip>
        </Popconfirm>
      </Flex>,
    },
  ]

  if (isLoading) return <LoadingScreen/>;
  if (!data?.plans) return <Heading1 emoji={"⚠️"}>エラーです</Heading1>;
  return (
    <>
      <Heading1 emoji={"💁"}>企画情報</Heading1>
      <Flex gap={8} vertical>
        <Table<BasePlanRead> size={"small"}
                             dataSource={data.plans.map(item => ({...item, key: item.id}))}
                             columns={columns}
                             bordered
                             scroll={{x: 'max-content'}}
        />
      </Flex>
    </>
  );
}