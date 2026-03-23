"use client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1, LoadingScreen} from "@koudaisai/shared-ui";
import {Button, Checkbox, Flex, message, Popconfirm, Table, TableProps, Tooltip, Upload} from "antd";
import {DeleteOutlined, DownloadOutlined, UploadOutlined} from "@ant-design/icons";
import {$plansInfoApiAdmin} from "@/lib/plansInfoApi";
import {
  BasePlanRead,
  BoothPlanCreate,
  BoothPlanUpdate,
  DaySchedule,
  GeneralPlanCreate,
  GeneralPlanUpdate,
  IndoorLocation,
  LaboPlanCreate,
  LaboPlanUpdate,
  OutdoorLocation,
  ScheduleCreate,
  ScheduleUpdate,
  StagePlanCreate,
  StagePlanUpdate,
} from "@koudaisai/shared-types";
import Papa from "papaparse";
import objectHash from "object-hash";
import {useDownload} from "@koudaisai/shared-utils";
import Image from "next/image";

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
  icon_url: string;
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
  icon_url?: string;
}

function buildDaySchedule(startTime: string, endTime: string): DaySchedule {
  return {start_time: startTime, end_time: endTime}
}

function buildLocation(building: string, locationName: string): (IndoorLocation | OutdoorLocation)[] {
  if (building !== "") {
    return [{type: "indoor", building, room: locationName}]
  }
  return [{type: "outdoor", name: locationName}]
}

function buildCreateSchedule(row: BulkCreateRow): ScheduleCreate {
  return {
    day1: row.day1_start_time !== "" && row.day1_end_time !== ""
      ? buildDaySchedule(row.day1_start_time, row.day1_end_time)
      : null,
    day2: row.day2_start_time !== "" && row.day2_end_time !== ""
      ? buildDaySchedule(row.day2_start_time, row.day2_end_time)
      : null,
  }
}

function buildPlanCreate(row: BulkCreateRow): BoothPlanCreate | GeneralPlanCreate | StagePlanCreate | LaboPlanCreate {
  const base = {
    organization_name: row.organization_name,
    plan_name: row.plan_name,
    description: row.description,
    is_child_friendly: row.is_child_friendly === "true",
    is_recommended: row.is_recommended === "true",
    schedule: buildCreateSchedule(row),
    location: buildLocation(row.building, row.location),
  }
  switch (row.id.charAt(0)) {
    case "M": return {...base, type: "booth", categories: []}
    case "I": return {...base, type: "general", categories: []}
    case "S": return {...base, type: "stage"}
    case "L": return {...base, type: "labo", is_lab_tour: row.is_lab_tour.toLowerCase() === "true"}
    default: throw new Error("企画番号の頭文字はM, I, S, Lのいずれかである必要があります。")
  }
}

function buildUpdateSchedule(row: BulkUpdateRow): ScheduleUpdate | undefined {
  let day1: DaySchedule | null | undefined
  let day2: DaySchedule | null | undefined

  if (row.day1_start_time === "null") {
    day1 = null
  } else if (row.day1_start_time && row.day1_start_time !== "" && row.day1_end_time && row.day1_end_time !== "") {
    day1 = buildDaySchedule(row.day1_start_time, row.day1_end_time)
  }

  if (row.day2_start_time === "null") {
    day2 = null
  } else if (row.day2_start_time && row.day2_start_time !== "" && row.day2_end_time && row.day2_end_time !== "") {
    day2 = buildDaySchedule(row.day2_start_time, row.day2_end_time)
  }

  if (day1 !== undefined || day2 !== undefined) {
    return {day1, day2}
  }
  return undefined
}

function buildUpdateLocation(row: BulkUpdateRow): (IndoorLocation | OutdoorLocation)[] | undefined {
  if (row.location && row.building) {
    return [{type: "indoor", building: row.building, room: row.location}]
  }
  if (row.location) {
    return [{type: "outdoor", name: row.location}]
  }
  return undefined
}

function buildPlanUpdate(row: BulkUpdateRow): BoothPlanUpdate | GeneralPlanUpdate | StagePlanUpdate | LaboPlanUpdate {
  const base = {
    organization_name: row.organization_name,
    plan_name: row.plan_name,
    description: row.description,
    is_child_friendly: row.is_child_friendly ? row.is_child_friendly === "true" : undefined,
    is_recommended: row.is_recommended ? row.is_recommended === "true" : undefined,
    schedule: buildUpdateSchedule(row),
    location: buildUpdateLocation(row),
  }

  if (row.id.charAt(0) === "L") {
    return {
      ...base,
      is_lab_tour: row.is_lab_tour ? row.is_lab_tour.toLowerCase() === "true" : undefined,
    }
  }
  return base
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
  const {mutateAsync: mutatePlanBulkCreate} = $plansInfoApiAdmin.useMutation("post", "/admin/plans:bulk")
  const {mutateAsync: mutatePlanIconImport} = $plansInfoApiAdmin.useMutation("post", "/admin/plans/{planId}/icon:import")
  const {mutateAsync: mutatePlanUpdate} = $plansInfoApiAdmin.useMutation("patch", "/admin/plans/{planId}")
  const {mutateAsync: mutatePlanDelete} = $plansInfoApiAdmin.useMutation("delete", "/admin/plans/{planId}")

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
    const iconUrls: Map<string, string> = new Map();
    let isAborted = false;

    Papa.parse<BulkCreateRow>(csv, {
        header: true,
        skipEmptyLines: true,
        worker: true,
        encoding: "UTF-8",
        step: (result, parser) => {
          try {
            const row = result.data;
            plans.set(row.id, buildPlanCreate(row))
            if (row.icon_url !== "") {
              iconUrls.set(row.id, row.icon_url)
            }
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
          if (isAborted) {
            return;
          }

          let isError = false;
          messageApi.destroy(hash)
          messageApi.loading({
            content: `企画情報を挿入中... ブラウザを閉じないでください`,
            key: hash,
            duration: 0
          });

          try {
            await mutatePlanBulkCreate({
              body: Object.fromEntries(plans)
            })
          } catch (err) {
            messageApi.destroy(hash)
            messageApi.error({
              content: `作成中にエラーが発生しました:${JSON.stringify(err)}`,
              key: hash
            });
            console.error(err)
            return;
          }

          messageApi.destroy(hash)
          messageApi.loading({
            content: `画像のインポート中...`,
            key: hash,
            duration: 0
          });
          let i = 0
          const n = iconUrls.size
          await Promise.all(iconUrls.entries().map(([id, iconUrl]) => {
            return mutatePlanIconImport({
              params: {path: {planId: id}},
              body: {url: iconUrl}
            }).then(() => {
              i++
              messageApi.destroy(hash)
              messageApi.loading({
                content: `画像のインポート中...(${i}/${n})`,
                key: hash,
                duration: 0
              });
            }).catch((err) => {
              messageApi.destroy(hash)
              messageApi.error({
                content: `画像インポート中にエラーが発生しました(${iconUrl}):${JSON.stringify(err)}`,
                key: hash
              });
              isError = true;
              console.error(err)
            })
          }))

          if (!isError) {
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
      }
    )
  }

  const handleBulkUpdate = async (csv: string) => {
    const hash = objectHash(csv)
    const plans: Map<string, BoothPlanUpdate | GeneralPlanUpdate | StagePlanUpdate | LaboPlanUpdate> = new Map();
    const iconUrls: Map<string, string> = new Map();
    let isAborted = false;

    Papa.parse<BulkUpdateRow>(csv, {
      header: true,
      skipEmptyLines: true,
      worker: true,
      encoding: "UTF-8",
      step: (result, parser) => {
        try {
          const row = result.data;
          plans.set(row.id, buildPlanUpdate(row))
          if (row.icon_url && row.icon_url !== "") {
            iconUrls.set(row.id, row.icon_url)
          }
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
                params: {path: {planId: id}},
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

        messageApi.destroy(hash)
        messageApi.loading({
          content: `画像のインポート中...`,
          key: hash,
          duration: 0
        });
        let i2 = 1
        const n2 = iconUrls.size
        await Promise.all(iconUrls.entries().map(([id, iconUrl]) => {
          return mutatePlanIconImport({
            params: {path: {planId: id}},
            body: {url: iconUrl}
          }).then(() => {
            i2++
            messageApi.destroy(hash)
            messageApi.loading({
              content: `画像のインポート中...(${i2}/${n2})`,
              key: hash,
              duration: 0
            });
          }).catch((err) => {
            messageApi.destroy(hash)
            messageApi.error({
              content: `画像インポート中にエラーが発生しました(${iconUrl}):${JSON.stringify(err)}`,
              key: hash
            });
            isError = true;
            console.error(err)
          })
        }))

        if (isError) {
          messageApi.destroy(hash)
          messageApi.error({
            content: `一部の更新に失敗しました。${isCreated.length}件更新しました。反映には最長で１分ほどかかる可能性があります。`,
            key: hash
          });
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
      let building = ""
      let location = ""
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
        day1_start_time: plan.schedule.day1?.start_time ?? "",
        day1_end_time: plan.schedule.day1?.end_time ?? "",
        day2_start_time: plan.schedule.day2?.start_time ?? "",
        day2_end_time: plan.schedule.day2?.end_time ?? "",
        building,
        location,
        is_lab_tour: "is_lab_tour" in plan ? (plan.is_lab_tour ? "true" : "false") : "false",
        icon_url: "https://api2025.jizi.jp/v1/plans/" + plan.id + "/icon"
      }
    }) ?? []

    const csv = Papa.unparse(rows, {newline: "\r\n"})
    const bom = "\uFEFF"
    const blob = new Blob([bom + csv], {type: "text/csv;charset=utf-8;"});
    download(URL.createObjectURL(blob), "plans.csv")
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
        {text: "模擬店企画", value: "booth"},
        {text: "一般企画", value: "general"},
        {text: "ステージ企画", value: "stage"},
        {text: "研究室公開企画", value: "labo"}
      ],
      onFilter: (value, record) => record.type === value,
      render: value => {
        switch (value) {
          case "booth":
            return <Tooltip title={"booth"}><span style={{color: "red"}}>模擬店企画</span></Tooltip>
          case "general":
            return <Tooltip title={"general"}><span style={{color: "blue"}}>一般企画</span></Tooltip>
          case "stage":
            return <Tooltip title={"stage"}><span style={{color: "green"}}>ステージ企画</span></Tooltip>
          case "labo":
            return <Tooltip title={"labo"}><span style={{color: "orange"}}>研究室公開企画</span></Tooltip>
          default:
            return <span>不明</span>
        }
      }
    },
    {
      key: "icon",
      title: <Tooltip title={"icon"}>アイコン</Tooltip>,
      dataIndex: "id",
      rowScope: "row",
      render: (_value, record) => (
        <Image
          src={`https://api2025.jizi.jp/cdn-cgi/image/width=128,height=128,format=webp,quality=auto/v1/plans/${record.id}/icon`}
          alt={"企画のアイコン"}
          width={128}
          height={128}
        />
      )
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
      render: (_value, record) => (
        <Tooltip title={record.is_child_friendly ? "true" : "false"}>
          <Checkbox checked={record.is_child_friendly} disabled/>
        </Tooltip>
      )
    },
    {
      key: "is_recommended",
      title: <Tooltip title={"is_recommended"}>おすすめ企画?</Tooltip>,
      dataIndex: "is_recommended",
      rowScope: "row",
      render: (_value, record) => (
        <Tooltip title={record.is_recommended ? "true" : "false"}>
          <Checkbox checked={record.is_recommended} disabled/>
        </Tooltip>
      )
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
          onCancel={() => {return}}
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
            await handleBulkUpdate(await file.text());
            return false
          }}
        >
          <Button icon={<UploadOutlined/>}>CSVから既存の企画情報を編集</Button>
        </Upload>
        <Button onClick={handleDownload} icon={<DownloadOutlined/>}>企画情報をCSVとしてダウンロード</Button>
      </Flex>

      <Flex gap={8} vertical>
        <Table<BasePlanRead>
          size={"small"}
          dataSource={data.plans.map(item => ({...item, key: item.id}))}
          columns={columns}
          bordered
          scroll={{x: 'max-content'}}
        />
      </Flex>
    </>
  );
}
