import {
  DeleteOutlined,
  DownloadOutlined,
  EditOutlined,
  FolderOpenOutlined,
  UploadOutlined,
} from '@ant-design/icons';
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
import { api, $events26Api } from '@/features/api/api';
import {
  ensureOk,
  formatTags,
  formatTime,
  GENERAL_TAGS,
  ICON_CONTENT_TYPES,
  iconUrl,
  parseTime,
  PROJECT_TYPE_LABEL,
  putIcon,
} from './project';
import type {
  FoodStallTag,
  GeneralTag,
  Occasion,
  Place,
  Project,
  Time,
} from './project';

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

/**
 * 新規作成用 CSV の 1 行。旧 API 時代の列に揃えた暫定の形式で、
 * ダウンロード・置き換えで使う [`ProjectRow`] とは別物。
 *
 * type 列は無く、企画種別は id の接頭辞(M/S/I/L)から決める。
 * タグの列も無いので、模擬店企画と一般企画のタグは空で作られる。
 */
type CreateProjectRow = {
  id: string;
  group_name: string;
  project_name: string;
  description: string;
  is_child_friendly: string;
  is_recommended: string;
  day1_start_time: string;
  day1_end_time: string;
  day2_start_time: string;
  day2_end_time: string;
  /** 1 日目・2 日目に共通の実施場所。 */
  place: string;
  /** id が `L` で始まる場合のみ必須。 */
  is_lab_tour: string;
  icon_url: string;
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

function parseBoolean(value: string): boolean {
  return value.trim().toLowerCase() === 'true';
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

/** 企画種別は id の接頭辞で決まる(M: 模擬店, S: ステージ, I: 一般, L: 研究室公開)。 */
const PROJECT_TYPE_BY_ID_PREFIX: Record<string, Project['type']> = {
  M: 'food-stall',
  S: 'stage',
  I: 'general',
  L: 'laboratory',
};

function projectTypeFromId(id: string): Project['type'] {
  const type = PROJECT_TYPE_BY_ID_PREFIX[id.charAt(0).toUpperCase()];
  if (!type) {
    throw new Error(
      `企画番号は M / S / I / L のいずれかで始まる必要があります: ${id}`,
    );
  }
  return type;
}

function requireField(value: string, column: string, id: string): string {
  const trimmed = value?.trim() ?? '';
  if (trimmed === '') {
    throw new Error(`${column} は必須です(${id || '企画番号不明'})`);
  }
  return trimmed;
}

/** true / false のみ受け付ける。空や別の値は行ごと弾く。 */
function requireBoolean(value: string, column: string, id: string): boolean {
  const trimmed = requireField(value, column, id).toLowerCase();
  if (trimmed !== 'true' && trimmed !== 'false') {
    throw new Error(
      `${column} は true か false で指定してください(${id}): ${value}`,
    );
  }
  return trimmed === 'true';
}

/**
 * day1 / day2 の時刻列から `occasions` を組み立てる。
 * 開始・終了は必ず対で、片方だけ入っている日はエラーにする。
 * 場所は両日に共通の `place` 列を使う。
 */
function buildCreateOccasions(row: CreateProjectRow): Occasion[] {
  const place = row.place?.trim() ?? '';
  const days: {
    date: Time['date'];
    start: string;
    end: string;
    startColumn: string;
    endColumn: string;
  }[] = [
    {
      date: 1,
      start: row.day1_start_time?.trim() ?? '',
      end: row.day1_end_time?.trim() ?? '',
      startColumn: 'day1_start_time',
      endColumn: 'day1_end_time',
    },
    {
      date: 2,
      start: row.day2_start_time?.trim() ?? '',
      end: row.day2_end_time?.trim() ?? '',
      startColumn: 'day2_start_time',
      endColumn: 'day2_end_time',
    },
  ];

  return days
    .filter((day) => {
      if (day.start === '' && day.end === '') return false;
      if (day.start === '' || day.end === '') {
        throw new Error(
          `${day.startColumn} と ${day.endColumn} は同時に指定してください(${row.id})`,
        );
      }
      return true;
    })
    .map((day) => ({
      // place は events26 側の enum。CSV の値をそのまま渡し、妥当性は API に委ねる。
      // 未指定は null ではなくキーごと省く(spec 上 optional であって nullable ではない)。
      ...(place === '' ? {} : { place: place as Place }),
      timeRange: {
        start: parseTime(day.date, day.start),
        end: parseTime(day.date, day.end),
      },
    }));
}

/**
 * 新規作成用 CSV の 1 行を [`Project`] にする。
 * タグの列が無いため、模擬店企画と一般企画のタグは空になる。
 */
function buildCreateProject(row: CreateProjectRow): Project {
  const id = requireField(row.id, 'id', row.id);
  const base = {
    id,
    groupName: requireField(row.group_name, 'group_name', id),
    projectName: requireField(row.project_name, 'project_name', id),
    description: requireField(row.description, 'description', id),
    isChildFriendly: requireBoolean(
      row.is_child_friendly,
      'is_child_friendly',
      id,
    ),
    isRecommended: requireBoolean(row.is_recommended, 'is_recommended', id),
    occasions: buildCreateOccasions(row),
  };

  switch (projectTypeFromId(id)) {
    case 'food-stall':
      return { ...base, type: 'food-stall', tag: [] };
    case 'general':
      return { ...base, type: 'general', tag: [] };
    case 'stage':
      return { ...base, type: 'stage' };
    case 'laboratory':
      return {
        ...base,
        type: 'laboratory',
        isTour: requireBoolean(row.is_lab_tour, 'is_lab_tour', id),
      };
  }
}

/** 新規作成 1 件分。アイコンは企画を作った後に URL から取ってきて送る。 */
type CreateEntry = { project: Project; iconUrl: string };

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

/** 新規作成用 CSV を 1 行ずつ読む。[`parseCsv`] と同じく 1 行でも壊れていたら全体を捨てる。 */
function parseCreateCsv(csv: string): Promise<CreateEntry[]> {
  return new Promise((resolve, reject) => {
    const entries: CreateEntry[] = [];
    Papa.parse<CreateProjectRow>(csv, {
      header: true,
      skipEmptyLines: true,
      encoding: 'UTF-8',
      step: (result, parser) => {
        try {
          entries.push({
            project: buildCreateProject(result.data),
            iconUrl: result.data.icon_url?.trim() ?? '',
          });
        } catch (error) {
          parser.abort();
          reject(error instanceof Error ? error : new Error(String(error)));
        }
      },
      complete: () => resolve(entries),
      error: (error: Error) => reject(error),
    });
  });
}

/** `M-001.png` のようなファイル名から企画 ID(`M-001`)を取り出す。 */
function projectIdFromFileName(fileName: string): string {
  return fileName.replace(/\.[^.]+$/, '');
}

/**
 * URL からアイコンを取ってきて送る。
 *
 * events26 のアイコン API は画像そのものを受け取る形で URL インポートが無いため、
 * ブラウザで一度取得してから中継に流す。取得元が CORS を許可していないと失敗する。
 */
async function putIconFromUrl(projectId: string, url: string): Promise<void> {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`アイコンの取得に失敗しました(${response.status}): ${url}`);
  }
  const image = await response.blob();
  if (!ICON_CONTENT_TYPES.includes(image.type)) {
    throw new Error(`対応外のアイコン形式です(${image.type}): ${url}`);
  }
  await putIcon(projectId, image);
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
  // アイコンは URL が同じまま中身だけ変わるので、更新後はこの値を進めて再取得させる。
  const [iconVersion, setIconVersion] = useState(0);
  const iconInputRef = useRef<HTMLInputElement>(null);

  const handleDelete = (id: string) => async () => {
    ensureOk(
      await api.DELETE('/events26/projects/{project_id}', {
        params: { path: { project_id: id } },
      }),
      '企画情報の削除',
    );
    await refetch();
  };

  const handleIconDelete = (id: string) => async () => {
    ensureOk(
      await api.DELETE('/events26/projects/{project_id}/icon', {
        params: { path: { project_id: id } },
      }),
      'アイコンの削除',
    );
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
  const applyCsv = async <T,>(
    csv: string,
    label: string,
    parse: (csv: string) => Promise<T[]>,
    idOf: (item: T) => string,
    apply: (item: T) => Promise<unknown>,
  ) => {
    const hash = objectHash(csv);

    let items: T[];
    try {
      items = await parse(csv);
    } catch (error) {
      console.error(error);
      messageApi.error({
        content: `CSVの読み込み中にエラーが発生しました：${String(error)}`,
        key: hash,
      });
      return;
    }

    const total = items.length;
    let done = 0;
    for (const item of items) {
      messageApi.destroy(hash);
      messageApi.loading({
        content: `${label}(${done + 1}/${total} - ${idOf(item)})... ブラウザを閉じないでください`,
        key: hash,
        duration: 0,
      });

      try {
        await apply(item);
      } catch (err) {
        console.error(err);
        messageApi.destroy(hash);
        messageApi.error({
          content: `${label}中にエラーが発生しました(${idOf(item)}):${JSON.stringify(err)}。${done}件${label}しました。`,
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

  /**
   * 新規作成は旧 API 時代の列に揃えた CSV を受ける。
   * アイコンの取得・送信に失敗しても企画自体は作れているので、
   * そこで打ち切らず最後にまとめて報告する。
   */
  const handleBulkCreate = async (csv: string) => {
    const iconFailures: string[] = [];

    await applyCsv(
      csv,
      '新規作成',
      parseCreateCsv,
      (entry: CreateEntry) => entry.project.id,
      async (entry: CreateEntry) => {
        ensureOk(
          await api.POST('/events26/projects', { body: entry.project }),
          '企画情報の新規作成',
        );
        if (entry.iconUrl === '') return;
        try {
          await putIconFromUrl(entry.project.id, entry.iconUrl);
        } catch (error) {
          console.error(error);
          iconFailures.push(`${entry.project.id}: ${String(error)}`);
        }
      },
    );

    setIconVersion((version) => version + 1);
    if (iconFailures.length > 0) {
      messageApi.warning(
        `アイコンの登録に${iconFailures.length}件失敗しました(${iconFailures.join(' / ')})。`,
      );
    }
  };

  const handleBulkReplace = (csv: string) =>
    applyCsv(
      csv,
      '更新',
      parseCsv,
      (project: Project) => project.id,
      async (project) =>
        ensureOk(
          await api.PUT('/events26/projects/{project_id}', {
            params: { path: { project_id: project.id } },
            body: project,
          }),
          '企画情報の更新',
        ),
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
      render: (value: string, record) => (
        <a
          style={{ textDecoration: 'underline' }}
          href={`/events26/edit?project_id=${encodeURIComponent(record.id)}`}
        >
          {value}
        </a>
      ),
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
          <Tooltip title="編集">
            <Button
              href={`/events26/edit?project_id=${encodeURIComponent(value)}`}
            >
              <EditOutlined />
            </Button>
          </Tooltip>
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
      <p style={{ marginTop: '-8px', marginBottom: '8px', color: '#888' }}>
        アイコンは企画番号をファイル名にしてください（例: <code>M-001.png</code>
        ）。対応形式は png / jpeg / gif / webp / heic、 正方形で 20MB 以下です。
      </p>
      <p style={{ marginBottom: '16px', color: '#888' }}>
        新規追加のCSVの列は <code>id</code>, <code>group_name</code>,{' '}
        <code>project_name</code>, <code>description</code>,{' '}
        <code>is_child_friendly</code>, <code>is_recommended</code>,{' '}
        <code>day1_start_time</code>, <code>day1_end_time</code>,{' '}
        <code>day2_start_time</code>, <code>day2_end_time</code>,{' '}
        <code>place</code>, <code>is_lab_tour</code>, <code>icon_url</code>{' '}
        です。時刻は <code>HH:mm</code> で開始と終了を対で指定し、企画種別は
        企画番号の接頭辞（M / S / I / L）から決まります。
        <code>is_lab_tour</code> は <code>L</code>{' '}
        で始まる企画のみ必須です。置き換え・ダウンロードのCSVは従来の列のままです。
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
