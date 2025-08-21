"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1, LoadingScreen} from "@/components/generic";
import {Button, Checkbox, Flex, message, Popconfirm, Table, TableProps, Tag, Tooltip, Upload} from "antd";
import {DeleteOutlined, DownloadOutlined, UploadOutlined} from "@ant-design/icons";
import {$plansInfoApiAdmin} from "@/lib/plansInfoApi";
import {
  BasePlanRead,
  BoothPlanCreate,
  BoothPlanUpdate,
  GeneralPlanCreate,
  GeneralPlanUpdate,
  LaboPlanCreate,
  LaboPlanUpdate,
  StagePlanCreate,
  StagePlanUpdate
} from "@/lib/plansInfoTypes";
import Papa from "papaparse";
import objectHash from "object-hash";
import {useDownload} from "@/lib";

type BulkCreateRow = {
  id: string;
  organization_name: string;
  plan_name: string;
  description: string;
  is_child_friendly: string;
  is_recommended: string;
  day1_start_time: string;
  day1_end_time: string;
  day2_start_time: string;
  day2_end_time: string;
  building: string;
  location: string;
  is_lab_tour: string;
}

type BulkUpdateRow = {
  id: string;
  organization_name?: string;
  plan_name?: string;
  description?: string;
  is_child_friendly?: string;
  is_recommended?: string;
  day1_start_time?: string;
  day1_end_time?: string;
  day2_start_time?: string;
  day2_end_time?: string;
  building?: string;
  location?: string;
  is_lab_tour?: string;
}

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const download = useDownload()
  const [messageApi, contextHolder] = message.useMessage();
  const {data, isLoading, refetch} = $plansInfoApiAdmin.useQuery("get", "/plans")
  const {mutateAsync: mutatePlanCreate} = $plansInfoApiAdmin.useMutation("put", "/plans/{planId}")
  const {mutateAsync: mutatePlanUpdate} = $plansInfoApiAdmin.useMutation("patch", "/plans/{planId}")
  const {mutateAsync: mutatePlanDelete} = $plansInfoApiAdmin.useMutation("delete", "/plans/{planId}")

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

  const handleBulkCreate = async (csv: string) => {
    const hash = objectHash(csv)
    const plans: Map<string, BoothPlanCreate | GeneralPlanCreate | StagePlanCreate | LaboPlanCreate> = new Map();
    let isAborted = false;

    Papa.parse<BulkCreateRow>(csv, {
      header: true,
      skipEmptyLines: true,
      worker: true,           // Web Workerでパース（UIをブロックしない）
      encoding: "UTF-8",      // Shift_JISなら "Shift_JIS" に変更（不明なら自前検知→二度読みが安全）
      step: (result, parser) => {
        try {
          const data = result.data;

          // type
          let type;
          switch (data.id.charAt(0)) {
            case "M":
              type = "booth";
              break;
            case "I":
              type = "general";
              break;
            case "S":
              type = "stage";
              break;
            case "L":
              type = "labo";
              break;
            default:
              throw "企画番号の頭文字はM, I, S, Lのいずれかである必要があります。"
          }

          // organization_name
          const organization_name = data.organization_name;

          // plan_name
          const plan_name = data.plan_name;

          // description
          const description = data.description;

          // is_child_friendly
          const is_child_friendly = data.is_child_friendly === "true";

          // is_recommended
          const is_recommended = data.is_recommended === "true";

          // schedule
          let day1 = null
          let day2 = null
          if (data.day1_start_time !== "" && data.day1_end_time !== "") {
            day1 = {
              start_time: data.day1_start_time,
              end_time: data.day1_end_time
            }
          }
          if (data.day2_start_time !== "" && data.day2_end_time !== "") {
            day2 = {
              start_time: data.day2_start_time,
              end_time: data.day2_end_time
            }
          }

          // location
          let location
          if (data.building !== "") {
            location = [{
              type: "indoor",
              building: data.building,
              room: data.location
            }]
          } else {
            location = [{
              type: "outdoor",
              name: data.location
            }]
          }

          // is_lab_tour
          let is_lab_tour = undefined;
          if (type === "labo") {
            is_lab_tour = data.is_lab_tour.toLowerCase() === "true";
          }

          plans.set(data.id, {
            type,
            organization_name,
            plan_name,
            description,
            is_child_friendly,
            is_recommended,
            schedule: {
              day1,
              day2
            },
            location,
            is_lab_tour
          })
        } catch (error) {
          console.error('Error in step processing:', error);
          messageApi.error({
            content: `行の処理中にエラーが発生しました：${error}`,
            key: hash
          });
          isAborted = true;
          parser.abort();
        }
      },
      complete: async () => {
        // If parsing was aborted due to an error, don't proceed with complete callback
        if (isAborted) {
          return;
        }

        const n = plans.size
        let i = 1;
        let isError = false;
        const isCreated = [];
        try {
          for (const [id, plan] of plans.entries()) {
            messageApi.destroy(hash)
            messageApi.loading({
              content: `作成中(${i}/${n} - ${id})... ブラウザを閉じないでください`,
              key: hash,
              duration: 0
            });

            try {
              await mutatePlanCreate({
                params: {
                  path: {
                    planId: id
                  }
                },
                body: plan
              })
            } catch (err) {
              messageApi.destroy(hash)
              messageApi.error({
                content: `作成中にエラーが発生しました:${JSON.stringify(err)}(${i}/${n} - ${id})`,
                key: hash
              });
              isError = true;
              break;
            }
            isCreated.push(id)

            i++
          }
        } catch (e) {
          messageApi.destroy(hash)
          messageApi.error({
            content: `エラーが発生しました：${e}(${i}/${n})`,
            key: hash
          });
        }

        if (isError) {
          messageApi.loading({
            content: "ロールバック中... ブラウザを閉じないでください．",
            key: hash + "_rollback",
            duration: 0
          })

          for (const id of isCreated) {
            try {
              await mutatePlanDelete({
                params: {
                  path: {
                    planId: id
                  }
                }
              })
            } catch (e) {
              messageApi.error(`${id} の削除に失敗しました: ${JSON.stringify(e)}`)
            }
          }
          messageApi.destroy(hash + "_rollback")
          messageApi.info({
            content: "ロールバック処理が終了しました．",
          })
        } else {
          messageApi.destroy(hash)
          messageApi.success({
            content: "新規作成が完了しました。反映には最長で１分ほどかかる可能性があります。",
            key: hash
          });
        }
      },
      error: (error: never) => {
        console.error(error);
        messageApi.error({
          content: `CSVの読み込み中にエラーが発生しました：${error}`,
          key: hash
        });
      },
    })
  }

  const handleBulkUpdate = async (csv: string) => {
    const hash = objectHash(csv)
    const plans: Map<string, BoothPlanUpdate | GeneralPlanUpdate | StagePlanUpdate | LaboPlanUpdate> = new Map();
    let isAborted = false;

    Papa.parse<BulkUpdateRow>(csv, {
      header: true,
      skipEmptyLines: true,
      worker: true,           // Web Workerでパース（UIをブロックしない）
      encoding: "UTF-8",      // Shift_JISなら "Shift_JIS" に変更（不明なら自前検知→二度読みが安全）
      step: (result, parser) => {
        try {
          const data = result.data;

          // type
          let type;
          switch (data.id.charAt(0)) {
            case "M":
              type = "booth";
              break;
            case "I":
              type = "general";
              break;
            case "S":
              type = "stage";
              break;
            case "L":
              type = "labo";
              break;
            default:
              throw "企画番号の頭文字はM, I, S, Lのいずれかである必要があります。"
          }

          // organization_name
          const organization_name = data.organization_name;

          // plan_name
          const plan_name = data.plan_name;

          // description
          const description = data.description;

          // is_child_friendly
          const is_child_friendly = data.is_child_friendly ? data.is_child_friendly === "true" : undefined;

          // is_recommended
          const is_recommended = data.is_recommended ? data.is_recommended === "true" : undefined;

          // schedule
          let schedule = undefined
          let day1 = undefined;
          let day2 = undefined;
          if (data.day1_start_time === "null") {
            day1 = null
          } else if (data.day1_start_time && data.day1_start_time !== "" && data.day1_end_time !== "") {
            day1 = {
              start_time: data.day1_start_time,
              end_time: data.day1_end_time
            }
          }
          if (data.day2_start_time === "null") {
            day2 = null
          } else if (data.day2_start_time && data.day2_start_time !== "" && data.day2_end_time !== "") {
            day2 = {
              start_time: data.day2_start_time,
              end_time: data.day2_end_time
            }
          }

          if (day1 || day2) {
            schedule = {
              day1, day2
            }
          }

          // location
          let location = undefined
          if (data.location && data.building !== "") {
            location = [{
              type: "indoor",
              building: data.building,
              room: data.location
            }]
          } else if (data.location) {
            location = [{
              type: "outdoor",
              name: data.location
            }]
          }

          // is_lab_tour
          let is_lab_tour = undefined;
          if (type === "labo") {
            const lab_tour = data.is_lab_tour
            is_lab_tour = lab_tour ? lab_tour.toLowerCase() === "true" : undefined;
          } else {
            type = undefined;
          }

          plans.set(data.id, {
            type,
            organization_name,
            plan_name,
            description,
            is_child_friendly,
            is_recommended,
            schedule,
            location,
            is_lab_tour
          })
        } catch (error) {
          console.error('Error in step processing:', error);
          messageApi.error({
            content: `行の処理中にエラーが発生しました：${error}`,
            key: hash
          });
          isAborted = true;
          parser.abort();
        }
      },
      complete: async () => {
        // If parsing was aborted due to an error, don't proceed with complete callback
        if (isAborted) {
          return;
        }

        const n = plans.size
        let i = 1;
        let isError = false;
        const isCreated = [];
        try {
          for (const [id, plan] of plans.entries()) {
            messageApi.destroy(hash)
            messageApi.loading({
              content: `更新中(${i}/${n} - ${id})... ブラウザを閉じないでください`,
              key: hash,
              duration: 0
            });

            try {
              await mutatePlanUpdate({
                params: {
                  path: {
                    planId: id
                  }
                },
                body: plan
              })
            } catch (err) {
              messageApi.destroy(hash)
              messageApi.error({
                content: `更新中にエラーが発生しました:${JSON.stringify(err)}(${i}/${n} - ${id})`,
                key: hash
              });
              isError = true;
              break;
            }
            isCreated.push(id)

            i++
          }
        } catch (e) {
          messageApi.destroy(hash)
          messageApi.error({
            content: `エラーが発生しました：${e}(${i}/${n})`,
            key: hash
          });
        }

        if (isError) {
          messageApi.loading({
            content: "ロールバック中... ブラウザを閉じないでください．",
            key: hash + "_rollback",
            duration: 0
          })

          for (const id of isCreated) {
            try {
              await mutatePlanDelete({
                params: {
                  path: {
                    planId: id
                  }
                },
              })
            } catch (e) {
              messageApi.destroy(`${id} の削除に失敗しました${JSON.stringify(e)}`)
            }
          }
          messageApi.destroy(hash + "_rollback")
          messageApi.info({
            content: "ロールバック処理が終了しました．",
          })
        } else {
          messageApi.destroy(hash)
          messageApi.success({
            content: "更新が完了しました。反映には最長で１分ほどかかる可能性があります。",
            key: hash
          });
        }
      },
      error: (error: never) => {
        console.error(error);
        messageApi.error({
          content: `CSVの読み込み中にエラーが発生しました：${error}`,
          key: hash
        });
      },
    })
  }

  const handleDownload = async () => {
    const rows = data?.plans?.map(plan => {
      let building = "なし"
      let location = "なし"
      switch (plan.location[0].type) {
        case "indoor":
          building = plan.location[0].building
          location = plan.location[0].room
          break;
        case "outdoor":
          location = plan.location[0].name
      }

      return {
        id: plan.id,
        organization_name: plan.organization_name,
        plan_name: plan.plan_name,
        description: plan.description,
        is_child_friendly: plan.is_child_friendly ? "true" : "false",
        is_recommended: plan.is_recommended ? "true" : "false",
        day1_start_time: plan.schedule?.day1?.start_time ?? "なし",
        day1_end_time: plan.schedule?.day1?.end_time ?? "なし",
        day2_start_time: plan.schedule?.day2?.start_time ?? "なし",
        day2_end_time: plan.schedule?.day2?.end_time ?? "なし",
        building,
        location,
        is_lab_tour: "is_lab_tour" in plan ? (plan.is_lab_tour ? "true" : "false") : "false",
      }
    }) ?? []

    const csv = Papa.unparse(rows)
    const blob = new Blob([csv], {type: "text/csv;charset=utf-8;"});
    const url = URL.createObjectURL(blob);

    download(url, "plans.csv")

    return
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
          return <Tooltip title={"true"}><Checkbox checked={true} disabled={true}/></Tooltip>
        } else {
          return <Tooltip title={"false"}><Checkbox checked={false} disabled={true}/></Tooltip>
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
          return <Tooltip title={"true"}><Checkbox checked={true} disabled={true}/></Tooltip>
        } else {
          return <Tooltip title={"false"}><Checkbox checked={false} disabled={true}/></Tooltip>
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
      {contextHolder}
      <Heading1 emoji={"💁"}>企画情報</Heading1>
      <Flex gap={8} align={"center"} wrap={"wrap"} style={{marginBottom: "16px"}}>
        <Upload
          maxCount={1}
          accept=".csv"
          beforeUpload={async (file) => {
            await handleBulkCreate(await file.text());
            return false
          }}
        >
          <Button icon={<UploadOutlined/>}>CSVから企画情報を新規追加</Button>
        </Upload>
        <Upload
          maxCount={1}
          accept=".csv"
          beforeUpload={async (file) => {
            handleBulkUpdate(await file.text());
            return false
          }}
        >
          <Button icon={<UploadOutlined/>}>CSVから既存の企画情報を編集</Button>
        </Upload>
        <Button onClick={handleDownload} icon={<DownloadOutlined/>}>企画情報をCSVとしてダウンロード</Button>
      </Flex>

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