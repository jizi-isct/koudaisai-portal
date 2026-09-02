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
  Input,
} from 'antd';
import type { TableProps } from 'antd';
import objectHash from 'object-hash';
import { useRef, useState, useMemo, type ChangeEvent } from 'react';
import { api, events26Api, $events26Api } from '@/features/api/api';
import { parseCreateCsv } from './createCsv';
import { createDownloadCsv } from './downloadCsv';
import { parseEditCsv } from './editCsv';
import {
  ensureOk,
  formatTags,
  GENERAL_TAGS,
  ICON_CONTENT_TYPES,
  iconUrl,
  PROJECT_TYPE_LABEL,
  putIcon,
} from './project';
import type { Occasion, Project } from './project';
import { enrichPlaceFloors, formatTime } from './util';

/** `M-001.png` のようなファイル名から企画 ID(`M-001`)を取り出す。 */
function projectIdFromFileName(fileName: string): string {
  return fileName.replace(/\.[^.]+$/, '');
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
  const { data: places, isLoading: isPlacesLoading } = $events26Api.useQuery(
    'get',
    '/v1/places',
  );

  const [filterKey, setFilterKey] = useState<string>('');

  // アイコンは URL が同じまま中身だけ変わるので、更新後はこの値を進めて再取得させる。
  const [iconVersion, setIconVersion] = useState(0);
  const [isDownloading, setIsDownloading] = useState(false);
  const iconInputRef = useRef<HTMLInputElement>(null);
  const errorMessage = (error: unknown) =>
    error instanceof Error ? error.message : String(error);

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
        content: `CSVの読み込み中にエラーが発生しました：${errorMessage(error)}`,
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
          content: `${label}中にエラーが発生しました(${idOf(item)}): ${errorMessage(err)}。${done}件${label}しました。`,
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
    applyCsv(
      csv,
      '新規作成',
      parseCreateCsv,
      (project: Project) => project.id,
      async (project) => {
        ensureOk(
          await api.POST('/events26/projects', { body: project }),
          '企画情報の新規作成',
        );
      },
    );

  const handleBulkEdit = (csv: string) =>
    applyCsv(
      csv,
      '編集',
      (contents) => parseEditCsv(contents, data ?? []),
      (project: Project) => project.id,
      async (project) =>
        ensureOk(
          await api.PUT('/events26/projects/{project_id}', {
            params: { path: { project_id: project.id } },
            body: project,
          }),
          '企画情報の編集',
        ),
    );

  const handleDownload = async () => {
    const key = 'download-events26-csv';
    setIsDownloading(true);
    messageApi.loading({
      content: '場所の階数情報を取得しています…',
      key,
      duration: 0,
    });

    try {
      const placeInfos = await enrichPlaceFloors(
        data ?? [],
        places ?? [],
        async (placeId) => {
          const result = await events26Api.GET('/v1/places/{placeId}', {
            params: {
              path: {
                placeId: placeId as NonNullable<Occasion['place']>,
              },
            },
          });
          ensureOk(result, `場所情報(${placeId})の取得`);
          if (!result.data) {
            throw new Error(`場所情報(${placeId})の取得結果が空です`);
          }
          return 'floor' in result.data ? result.data.floor : undefined;
        },
      );
      const csv = createDownloadCsv(data ?? [], placeInfos);
      const bom = '\uFEFF';
      const blob = new Blob([bom + csv], { type: 'text/csv;charset=utf-8;' });
      download(URL.createObjectURL(blob), 'projects.csv');
      messageApi.destroy(key);
    } catch (error) {
      console.error(error);
      messageApi.error({
        content: `CSVの作成中にエラーが発生しました：${errorMessage(error)}`,
        key,
      });
    } finally {
      setIsDownloading(false);
    }
  };

  const handleFilterProjectsByName = (event: ChangeEvent<HTMLInputElement>) => {
    setFilterKey(event.target.value);
  };

  const targetedProjects: Project[] = useMemo<Project[]>(
    () => data?.filter((item) => item.groupName.includes(filterKey)) ?? [],
    [data, filterKey],
  );

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

  if (isLoading || isPlacesLoading) return <LoadingScreen />;
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
            await handleBulkEdit(await file.text());
            return false;
          }}
        >
          <Button icon={<UploadOutlined />}>CSVから既存の企画情報を編集</Button>
        </Upload>
        <Button
          onClick={handleDownload}
          icon={<DownloadOutlined />}
          loading={isDownloading}
        >
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
        <code>place</code>, <code>is_lab_tour</code>, <code>offering</code>,{' '}
        <code>category</code> です。時刻は <code>HH:mm</code>{' '}
        で開始と終了を対で指定し、企画種別は 企画番号の接頭辞（M / S / I /
        L）から決まります。
        <code>is_lab_tour</code> は <code>L</code>{' '}
        で始まる企画のみ必須です。編集CSVは <code>id</code>{' '}
        だけが必須で、それ以外は新規追加CSVと同じ列を任意に指定できます。存在する列だけを編集します。
        ダウンロードCSVは一覧確認用の別スキーマです。
      </p>

      <Input
        placeholder="団体名を検索"
        onChange={handleFilterProjectsByName}
        style={{ width: 200 }}
      />

      <Flex gap={8} vertical>
        <Table<Project>
          size="small"
          dataSource={targetedProjects.map((item) => ({
            ...item,
            key: item.id,
          }))}
          columns={columns}
          bordered
          scroll={{ x: 'max-content' }}
        />
      </Flex>
    </>
  );
}
