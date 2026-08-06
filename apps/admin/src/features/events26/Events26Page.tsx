import {
  DeleteOutlined,
  DownloadOutlined,
  FolderOpenOutlined,
  UploadOutlined,
} from '@ant-design/icons';
import type {
  apiComponents,
  events26Components,
} from '@koudaisai/shared-types';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { useDownload } from '@koudaisai/shared-utils';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import {
  Button,
  Checkbox,
  Flex,
  message,
  Popconfirm,
  Table,
  Tag,
  Tooltip,
  Upload,
} from 'antd';
import type { TableProps } from 'antd';
import objectHash from 'object-hash';
import Papa from 'papaparse';
import { useRef, useState } from 'react';
import { EVENTS26_API_URL } from 'astro:env/client';
import { api, $api, $events26Api } from '@/features/api/api';

type Project = events26Components['schemas']['Project'];
type Occasion = events26Components['schemas']['Occasion'];
type Place = NonNullable<Occasion['place']>;
type Time = events26Components['schemas']['Time'];
type FoodStallTag = events26Components['schemas']['FoodStallTag'];
type GeneralTag = events26Components['schemas']['GeneralTag'];

/**
 * backend 中継(`/api/v3/events26`)が公開している `Project` スキーマ。
 *
 * 中身は events26 の `Project` と同じものだが、backend 側の生成クライアントで
 * `Time.date` が events26 の `1 | 2` ではなく `string` として公開されており、
 * 型としては一致しない。正本は events26 の spec なのでこちらで組み立て、
 * 送信時だけこの型へ寄せる。backend 側のスキーマが直ったらキャストは外せる。
 */
type ApiProject = apiComponents['schemas']['Project'];

/** CSV の 1 行。すべての列を文字列として読み書きする。 */
type ProjectRow = {
  id: string;
  type: string;
  group_name: string;
  project_name: string;
  description: string;
  is_child_friendly: string;
  is_recommended: string;
  /** 研究室公開企画のみ。ラボツアーかどうか。 */
  is_tour: string;
  /**
   * 模擬店企画は `main:rice;sweet:western`、一般企画は `experience;display`。
   * ステージ企画と研究室公開企画にタグは無いので空。
   */
  tags: string;
  day1_place: string;
  day1_start: string;
  day1_end: string;
  day2_place: string;
  day2_start: string;
  day2_end: string;
};

const CSV_COLUMNS: (keyof ProjectRow)[] = [
  'id',
  'type',
  'group_name',
  'project_name',
  'description',
  'is_child_friendly',
  'is_recommended',
  'is_tour',
  'tags',
  'day1_place',
  'day1_start',
  'day1_end',
  'day2_place',
  'day2_start',
  'day2_end',
];

const PROJECT_TYPE_LABEL: Record<
  Project['type'],
  { text: string; color: string }
> = {
  'food-stall': { text: '模擬店企画', color: 'red' },
  general: { text: '一般企画', color: 'blue' },
  stage: { text: 'ステージ企画', color: 'green' },
  laboratory: { text: '研究室公開企画', color: 'orange' },
};

const GENERAL_TAGS: GeneralTag[] = [
  'experience',
  'display',
  'performance',
  'food',
  'lecture',
];

function parseBoolean(value: string): boolean {
  return value.trim().toLowerCase() === 'true';
}

/** `HH:MM` を `date` 日目の [`Time`] に変換する。 */
function parseTime(date: Time['date'], value: string): Time {
  const match = /^(\d{1,2}):(\d{2})$/.exec(value.trim());
  if (!match) {
    throw new Error(`時刻は HH:MM 形式で指定してください: ${value}`);
  }
  const hour = Number(match[1]);
  const minute = Number(match[2]);
  if (hour > 23 || minute > 59) {
    throw new Error(`時刻の範囲が不正です: ${value}`);
  }
  return { date, hour, minute };
}

function formatTime(time: Time): string {
  return `${String(time.hour).padStart(2, '0')}:${String(time.minute).padStart(2, '0')}`;
}

/**
 * day1 / day2 の列から `occasions` を組み立てる。
 * 開始・終了の両方が入っている日だけを要素にするので、片方だけの日は落ちる。
 */
function buildOccasions(row: ProjectRow): Occasion[] {
  const days: {
    date: Time['date'];
    place: string;
    start: string;
    end: string;
  }[] = [
    {
      date: 1,
      place: row.day1_place,
      start: row.day1_start,
      end: row.day1_end,
    },
    {
      date: 2,
      place: row.day2_place,
      start: row.day2_start,
      end: row.day2_end,
    },
  ];

  return days
    .filter((day) => day.start.trim() !== '' && day.end.trim() !== '')
    .map((day) => ({
      // place は events26 側の enum。CSV の値をそのまま渡し、妥当性は API に委ねる。
      // 未指定は null ではなくキーごと省く(spec 上 optional であって nullable ではない)。
      ...(day.place.trim() === '' ? {} : { place: day.place.trim() as Place }),
      timeRange: {
        start: parseTime(day.date, day.start),
        end: parseTime(day.date, day.end),
      },
    }));
}

function buildFoodStallTags(tags: string): FoodStallTag[] {
  return tags
    .split(';')
    .map((tag) => tag.trim())
    .filter((tag) => tag !== '')
    .map((tag) => {
      const [head, tail] = tag.split(':').map((part) => part.trim());
      switch (head) {
        case 'main':
        case 'sweet':
          if (!tail) {
            throw new Error(`模擬店タグ ${head} には tag2 が必要です: ${tag}`);
          }
          return { tag: head, tag2: tail } as FoodStallTag;
        case 'drink':
          return { tag: 'drink' } as FoodStallTag;
        default:
          throw new Error(`不明な模擬店タグです: ${tag}`);
      }
    });
}

function buildGeneralTags(tags: string): GeneralTag[] {
  return tags
    .split(';')
    .map((tag) => tag.trim())
    .filter((tag) => tag !== '')
    .map((tag) => {
      if (!GENERAL_TAGS.includes(tag as GeneralTag)) {
        throw new Error(`不明な一般企画タグです: ${tag}`);
      }
      return tag as GeneralTag;
    });
}

function buildProject(row: ProjectRow): Project {
  const base = {
    id: row.id.trim(),
    groupName: row.group_name,
    projectName: row.project_name,
    description: row.description,
    isChildFriendly: parseBoolean(row.is_child_friendly),
    isRecommended: parseBoolean(row.is_recommended),
    occasions: buildOccasions(row),
  };

  switch (row.type.trim()) {
    case 'food-stall':
      return { ...base, type: 'food-stall', tag: buildFoodStallTags(row.tags) };
    case 'general':
      return { ...base, type: 'general', tag: buildGeneralTags(row.tags) };
    case 'stage':
      return { ...base, type: 'stage' };
    case 'laboratory':
      return { ...base, type: 'laboratory', isTour: parseBoolean(row.is_tour) };
    default:
      throw new Error(
        `type は food-stall, general, stage, laboratory のいずれかである必要があります: ${row.type}`,
      );
  }
}

function formatTags(project: Project): string {
  if (project.type === 'food-stall') {
    return project.tag
      .map((tag) => ('tag2' in tag ? `${tag.tag}:${tag.tag2}` : tag.tag))
      .join(';');
  }
  if (project.type === 'general') {
    return project.tag.join(';');
  }
  return '';
}

function toRow(project: Project): ProjectRow {
  const day1 = project.occasions.find(
    (occasion) => occasion.timeRange.start.date === 1,
  );
  const day2 = project.occasions.find(
    (occasion) => occasion.timeRange.start.date === 2,
  );

  return {
    id: project.id,
    type: project.type,
    group_name: project.groupName,
    project_name: project.projectName,
    description: project.description,
    is_child_friendly: project.isChildFriendly ? 'true' : 'false',
    is_recommended: project.isRecommended ? 'true' : 'false',
    is_tour:
      project.type === 'laboratory' ? (project.isTour ? 'true' : 'false') : '',
    tags: formatTags(project),
    day1_place: day1?.place ?? '',
    day1_start: day1 ? formatTime(day1.timeRange.start) : '',
    day1_end: day1 ? formatTime(day1.timeRange.end) : '',
    day2_place: day2?.place ?? '',
    day2_start: day2 ? formatTime(day2.timeRange.start) : '',
    day2_end: day2 ? formatTime(day2.timeRange.end) : '',
  };
}

/**
 * CSV を 1 行ずつ [`Project`] にする。
 * 1 行でも壊れていたら全体を捨てる(部分適用されると原因を追いにくいため)。
 */
function parseCsv(csv: string): Promise<Project[]> {
  return new Promise((resolve, reject) => {
    const projects: Project[] = [];
    Papa.parse<ProjectRow>(csv, {
      header: true,
      skipEmptyLines: true,
      worker: true,
      encoding: 'UTF-8',
      step: (result, parser) => {
        try {
          projects.push(buildProject(result.data));
        } catch (error) {
          parser.abort();
          reject(error instanceof Error ? error : new Error(String(error)));
        }
      },
      complete: () => resolve(projects),
      error: (error: Error) => reject(error),
    });
  });
}

/** events26 がアイコンとして受け付ける形式。ここに無いファイルは送らずに飛ばす。 */
const ICON_CONTENT_TYPES = [
  'image/png',
  'image/jpeg',
  'image/gif',
  'image/webp',
  'image/heic',
];

/** アイコン原本の公開 URL。events26 が認証なしで配信している。 */
function iconUrl(projectId: string, cacheBuster: number): string {
  return `${EVENTS26_API_URL}/v1/projects/${encodeURIComponent(projectId)}/icon?v=${cacheBuster}`;
}

/** `M-001.png` のようなファイル名から企画 ID(`M-001`)を取り出す。 */
function projectIdFromFileName(fileName: string): string {
  return fileName.replace(/\.[^.]+$/, '');
}

/**
 * アイコンを 1 件 PUT する。
 *
 * events26 の `/admin/v1` は Cloudflare Access 配下なので backend 中継を通す。
 * ボディは画像そのもので JSON ではないため、openapi-fetch の既定シリアライザを
 * 素通しに差し替え、形式判定に使う `Content-Type` を明示する。
 */
async function putIcon(projectId: string, file: File): Promise<void> {
  const { error } = await api.PUT('/events26/projects/{project_id}/icon', {
    params: { path: { project_id: projectId } },
    body: file as unknown as string,
    bodySerializer: (body: unknown) => body as BodyInit,
    headers: { 'Content-Type': file.type },
  });
  if (error) {
    throw error;
  }
}

export function Events26Page() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <Events26Table />
    </QueryClientProvider>
  );
}

function Events26Table() {
  const download = useDownload();
  const [messageApi, contextHolder] = message.useMessage();
  // 一覧は backend に中継が無いので events26 の公開エンドポイントを直接読む。
  const { data, isLoading, refetch } = $events26Api.useQuery(
    'get',
    '/v1/projects',
  );
  const { mutateAsync: mutateProjectCreate } = $api.useMutation(
    'post',
    '/events26/projects',
  );
  const { mutateAsync: mutateProjectReplace } = $api.useMutation(
    'put',
    '/events26/projects/{project_id}',
  );
  const { mutateAsync: mutateProjectDelete } = $api.useMutation(
    'delete',
    '/events26/projects/{project_id}',
  );
  const { mutateAsync: mutateIconDelete } = $api.useMutation(
    'delete',
    '/events26/projects/{project_id}/icon',
  );
  // アイコンは URL が同じまま中身だけ変わるので、更新後はこの値を進めて再取得させる。
  const [iconVersion, setIconVersion] = useState(0);
  const iconInputRef = useRef<HTMLInputElement>(null);

  const handleDelete = (id: string) => async () => {
    await mutateProjectDelete({ params: { path: { project_id: id } } });
    await refetch();
  };

  const handleIconDelete = (id: string) => async () => {
    await mutateIconDelete({ params: { path: { project_id: id } } });
    setIconVersion((version) => version + 1);
  };

  /**
   * フォルダごと選ばれた画像を、ファイル名(拡張子を除いた部分)の企画 ID へ順に送る。
   * 対応外の形式は送らずに飛ばし、最後にまとめて件数を報告する。
   */
  const handleIconFolder = async (files: File[]) => {
    const key = 'icon-folder-upload';
    const targets = files.filter((file) =>
      ICON_CONTENT_TYPES.includes(file.type),
    );
    const skipped = files.length - targets.length;

    if (targets.length === 0) {
      messageApi.error({
        content: '対応形式(png/jpeg/gif/webp/heic)の画像がありませんでした。',
        key,
      });
      return;
    }

    const failed: string[] = [];
    let done = 0;
    for (const file of targets) {
      const projectId = projectIdFromFileName(file.name);
      messageApi.destroy(key);
      messageApi.loading({
        content: `アイコンをアップロード中(${done + 1}/${targets.length} - ${projectId})... ブラウザを閉じないでください`,
        key,
        duration: 0,
      });

      try {
        await putIcon(projectId, file);
        done++;
      } catch (error) {
        console.error(error);
        failed.push(`${file.name}: ${JSON.stringify(error)}`);
      }
    }

    setIconVersion((version) => version + 1);
    messageApi.destroy(key);

    const skippedNote =
      skipped > 0 ? `対応外の形式を${skipped}件飛ばしました。` : '';
    if (failed.length > 0) {
      messageApi.error({
        content: `${done}件アップロードしました。${failed.length}件失敗しています(${failed.join(' / ')})。${skippedNote}`,
        key,
      });
      return;
    }
    messageApi.success({
      content: `${done}件のアイコンをアップロードしました。${skippedNote}`,
      key,
    });
  };

  /**
   * CSV の各行を 1 件ずつ送る。events26 には一括投入が無いため。
   * 途中で失敗したらそこで打ち切り、成功した件数を伝える。
   */
  const applyCsv = async (
    csv: string,
    label: string,
    apply: (project: Project) => Promise<unknown>,
  ) => {
    const hash = objectHash(csv);

    let projects: Project[];
    try {
      projects = await parseCsv(csv);
    } catch (error) {
      console.error(error);
      messageApi.error({
        content: `CSVの読み込み中にエラーが発生しました：${String(error)}`,
        key: hash,
      });
      return;
    }

    const total = projects.length;
    let done = 0;
    for (const project of projects) {
      messageApi.destroy(hash);
      messageApi.loading({
        content: `${label}(${done + 1}/${total} - ${project.id})... ブラウザを閉じないでください`,
        key: hash,
        duration: 0,
      });

      try {
        await apply(project);
      } catch (err) {
        console.error(err);
        messageApi.destroy(hash);
        messageApi.error({
          content: `${label}中にエラーが発生しました(${project.id}):${JSON.stringify(err)}。${done}件${label}しました。`,
          key: hash,
        });
        await refetch();
        return;
      }
      done++;
    }

    messageApi.destroy(hash);
    messageApi.success({
      content: `${total}件の${label}が完了しました。`,
      key: hash,
    });
    await refetch();
  };

  const handleBulkCreate = (csv: string) =>
    applyCsv(csv, '新規作成', (project) =>
      mutateProjectCreate({ body: project as unknown as ApiProject }),
    );

  const handleBulkReplace = (csv: string) =>
    applyCsv(csv, '更新', (project) =>
      mutateProjectReplace({
        params: { path: { project_id: project.id } },
        body: project as unknown as ApiProject,
      }),
    );

  const handleDownload = () => {
    const csv = Papa.unparse(data?.map(toRow) ?? [], {
      columns: CSV_COLUMNS,
      newline: '\r\n',
    });
    const bom = '\uFEFF';
    const blob = new Blob([bom + csv], { type: 'text/csv;charset=utf-8;' });
    download(URL.createObjectURL(blob), 'projects.csv');
  };

  const columns: TableProps<Project>['columns'] = [
    {
      key: 'id',
      title: <Tooltip title="id">企画番号</Tooltip>,
      dataIndex: 'id',
      rowScope: 'row',
    },
    {
      key: 'type',
      title: <Tooltip title="type">種類</Tooltip>,
      dataIndex: 'type',
      filters: Object.entries(PROJECT_TYPE_LABEL).map(([value, { text }]) => ({
        text,
        value,
      })),
      onFilter: (value, record) => record.type === value,
      render: (value: Project['type']) => {
        const label = PROJECT_TYPE_LABEL[value];
        if (!label) return <Tag color="warning">不明</Tag>;
        return (
          <Tooltip title={value}>
            <Tag color={label.color}>{label.text}</Tag>
          </Tooltip>
        );
      },
    },
    {
      key: 'icon',
      title: 'アイコン',
      render: (_value, record) => (
        <Flex vertical gap={4} align="center">
          <img
            src={iconUrl(record.id, iconVersion)}
            alt="企画のアイコン"
            width={96}
            height={96}
            // 未登録の企画は 404 になる。壊れた画像アイコンを出さずに隠す。
            onError={(event) => {
              event.currentTarget.style.visibility = 'hidden';
            }}
          />
          <Popconfirm
            title="アイコンを削除"
            description="この企画のアイコンを削除しますか？"
            onConfirm={handleIconDelete(record.id)}
            okText="はい"
            cancelText="いいえ"
          >
            <Button size="small" danger icon={<DeleteOutlined />} />
          </Popconfirm>
        </Flex>
      ),
    },
    {
      key: 'groupName',
      title: <Tooltip title="groupName">団体名</Tooltip>,
      dataIndex: 'groupName',
      rowScope: 'row',
    },
    {
      key: 'projectName',
      title: <Tooltip title="projectName">企画名</Tooltip>,
      dataIndex: 'projectName',
      rowScope: 'row',
    },
    {
      key: 'description',
      title: <Tooltip title="description">概要</Tooltip>,
      dataIndex: 'description',
      rowScope: 'row',
    },
    {
      key: 'tags',
      title: <Tooltip title="tag">タグ</Tooltip>,
      render: (_value, record) => formatTags(record) || '-',
    },
    {
      key: 'occasions',
      title: <Tooltip title="occasions">開催予定</Tooltip>,
      render: (_value, record) =>
        record.occasions.length === 0
          ? '-'
          : record.occasions.map((occasion, index) => (
              <div key={index}>
                {`${occasion.timeRange.start.date}日目 ${formatTime(occasion.timeRange.start)}〜${formatTime(occasion.timeRange.end)}`}
                {occasion.place ? ` @ ${occasion.place}` : ''}
              </div>
            )),
    },
    {
      key: 'isChildFriendly',
      title: <Tooltip title="isChildFriendly">子供向け企画?</Tooltip>,
      dataIndex: 'isChildFriendly',
      rowScope: 'row',
      render: (_value, record) => (
        <Tooltip title={record.isChildFriendly ? 'true' : 'false'}>
          <Checkbox checked={record.isChildFriendly} disabled />
        </Tooltip>
      ),
    },
    {
      key: 'isRecommended',
      title: <Tooltip title="isRecommended">おすすめ企画?</Tooltip>,
      dataIndex: 'isRecommended',
      rowScope: 'row',
      render: (_value, record) => (
        <Tooltip title={record.isRecommended ? 'true' : 'false'}>
          <Checkbox checked={record.isRecommended} disabled />
        </Tooltip>
      ),
    },
    {
      key: 'actions',
      title: '操作',
      dataIndex: 'id',
      fixed: 'right',
      render: (value: string) => (
        <Flex gap={5}>
          <Popconfirm
            title="企画情報を削除"
            description="あなたは本当にこの企画情報を削除する気ですか！？"
            onConfirm={handleDelete(value)}
            okText="はい"
            cancelText="いいえ"
          >
            <Tooltip title="削除">
              <Button danger>
                <DeleteOutlined />
              </Button>
            </Tooltip>
          </Popconfirm>
        </Flex>
      ),
    },
  ];

  if (isLoading) return <LoadingScreen />;
  if (!data) return <Heading1 emoji="⚠️">エラーです</Heading1>;

  return (
    <>
      {contextHolder}
      <Heading1 emoji="💁">企画情報</Heading1>
      <Flex gap={8} align="center" wrap="wrap" style={{ marginBottom: '16px' }}>
        <Upload
          maxCount={1}
          accept=".csv"
          beforeUpload={async (file) => {
            await handleBulkCreate(await file.text());
            return false;
          }}
        >
          <Button icon={<UploadOutlined />}>CSVから企画情報を新規追加</Button>
        </Upload>
        <Upload
          maxCount={1}
          accept=".csv"
          beforeUpload={async (file) => {
            await handleBulkReplace(await file.text());
            return false;
          }}
        >
          <Button icon={<UploadOutlined />}>
            CSVから既存の企画情報を置き換え
          </Button>
        </Upload>
        <Button onClick={handleDownload} icon={<DownloadOutlined />}>
          企画情報をCSVとしてダウンロード
        </Button>
        {/*
          antd の Upload はファイル 1 件ごとに beforeUpload が走るため、
          フォルダ全体を 1 回のイベントで受け取れる素の input を使う。
          webkitdirectory は React の型に無いので属性を直接指定する。
        */}
        <input
          ref={iconInputRef}
          type="file"
          multiple
          accept={ICON_CONTENT_TYPES.join(',')}
          style={{ display: 'none' }}
          {...{ webkitdirectory: '' }}
          onChange={async (event) => {
            const files = Array.from(event.target.files ?? []);
            // 同じフォルダを選び直しても change が起きるようにクリアしておく。
            event.target.value = '';
            if (files.length > 0) {
              await handleIconFolder(files);
            }
          }}
        />
        <Button
          icon={<FolderOpenOutlined />}
          onClick={() => iconInputRef.current?.click()}
        >
          フォルダからアイコンを一括アップロード
        </Button>
      </Flex>
      <p style={{ marginTop: '-8px', marginBottom: '16px', color: '#888' }}>
        アイコンは企画番号をファイル名にしてください（例: <code>M-001.png</code>
        ）。対応形式は png / jpeg / gif / webp / heic、 正方形で 20MB 以下です。
      </p>

      <Flex gap={8} vertical>
        <Table<Project>
          size="small"
          dataSource={data.map((item) => ({ ...item, key: item.id }))}
          columns={columns}
          bordered
          scroll={{ x: 'max-content' }}
        />
      </Flex>
    </>
  );
}
