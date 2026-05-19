import {DeleteOutlined, PlusOutlined} from "@ant-design/icons";
import type {DocumentCategoryRead, DocumentRead} from "@koudaisai/shared-types";
import {Heading1, LoadingScreen} from "@koudaisai/shared-ui";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Button, Flex, message, Popconfirm, Table, Tag} from "antd";
import type {TableProps} from "antd";
import {useMemo, useState} from "react";
import {$api} from "@/features/api/api";

type RowType = {
  category: DocumentCategoryRead | null;
  documents: DocumentRead[];
};

export function DocumentsPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <DocumentsTable />
    </QueryClientProvider>
  );
}

function formatDate(value: string) {
  return new Date(value).toLocaleString("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function DocumentsTable() {
  const [messageApi, contextHolder] = message.useMessage();
  const {data, isLoading, refetch} = $api.useQuery("get", "/documents/by-category", {
    params: {query: {include_empty_categories: true}},
  });
  const {mutateAsync: mutateDocumentCategoryDelete} = $api.useMutation(
    "delete",
    "/document-categories/{category_id}",
  );
  const {mutateAsync: mutateDocumentCategoryCreate} = $api.useMutation(
    "post",
    "/document-categories",
  );
  const {mutateAsync: mutateDocumentDelete} = $api.useMutation(
    "delete",
    "/documents/{document_id}",
  );

  const dataSource = useMemo(() => {
    return data?.map((item) => ({
      ...item,
      key: item.category?.id ?? "なし",
    })) ?? [];
  }, [data]);

  const handleCreateCategory = async () => {
    messageApi.loading("新規資料カテゴリを作成中...");
    try {
      await mutateDocumentCategoryCreate({
        body: {
          title: "新規資料カテゴリ",
          emoji: null,
        },
      });
    } catch (e) {
      messageApi.destroy();
      messageApi.error(`資料カテゴリの作成に失敗しました: ${String(e)}`);
      return;
    }

    await refetch();
    messageApi.destroy();
    messageApi.success("資料カテゴリを作成しました");
  };

  const columns: TableProps<RowType>["columns"] = [
    {
      key: "title",
      title: "資料カテゴリ",
      render: (_value, record) =>
        record.category ? (
          <a
            style={{textDecoration: "underline"}}
            href={`/documents/edit_category?category_id=${record.category.id}`}
          >
            {record.category.emoji} {record.category.title}
          </a>
        ) : (
          <span>⚠️ カテゴリーなし</span>
        ),
    },
    {
      key: "created_at",
      title: "作成日時",
      rowScope: "row",
      render: (_value, record) => (record.category ? formatDate(record.category.created_at) : ""),
    },
    {
      key: "updated_at",
      title: "更新日時",
      rowScope: "row",
      render: (_value, record) => (record.category ? formatDate(record.category.updated_at) : ""),
    },
    {
      key: "actions",
      title: "操作",
      fixed: "right",
      render: (_value, record) => (
        <Flex gap={5}>
          {record.category ? (
            <Popconfirm
              title="資料カテゴリを削除"
              description="あなたは本当にこの資料カテゴリを削除する気ですか！？"
              onConfirm={async () => {
                await mutateDocumentCategoryDelete({
                  params: {
                    path: {
                      category_id: record.category?.id ?? "",
                    },
                  },
                });
                await refetch();
              }}
              okText="はい"
              cancelText="いいえ"
            >
              <Button danger>
                <DeleteOutlined />
              </Button>
            </Popconfirm>
          ) : (
            <Button danger disabled>
              <DeleteOutlined />
            </Button>
          )}
        </Flex>
      ),
    },
  ];

  const expandedColumns: TableProps<DocumentRead>["columns"] = [
    {
      key: "title",
      title: "タイトル",
      dataIndex: "title",
      rowScope: "row",
      render: (value, record) => (
        <a style={{textDecoration: "underline"}} href={`/documents/edit?document_id=${record.id}`}>
          {value}
        </a>
      ),
    },
    {
      key: "format",
      title: "種類",
      render: (_value, record) => {
        if ("format_pdf" in record) return <Tag color="orange">PDF</Tag>;
        if ("format_markdown" in record) return <Tag color="green">Markdown</Tag>;
        if ("format_misc" in record) return <Tag color="blue">その他</Tag>;
        return <Tag color="red">不明</Tag>;
      },
    },
    {
      key: "targets",
      title: "対象",
      dataIndex: "targets",
      render: (value: string[]) => (
        <Flex wrap gap={4} style={{maxWidth: "200px"}}>
          {value.map((target) => (
            <Tag key={target}>{target}</Tag>
          ))}
        </Flex>
      ),
    },
    {
      key: "created_at",
      title: "作成日時",
      dataIndex: "created_at",
      rowScope: "row",
      render: formatDate,
    },
    {
      key: "updated_at",
      title: "更新日時",
      dataIndex: "updated_at",
      rowScope: "row",
      render: formatDate,
    },
    {
      key: "created_by",
      title: "作成者",
      dataIndex: "created_by",
      rowScope: "row",
      render: (value: string) => `${value.substring(0, 6)}...`,
    },
    {
      key: "updated_by",
      title: "更新者",
      dataIndex: "updated_by",
      rowScope: "row",
      render: (value: string) => `${value.substring(0, 6)}...`,
    },
    {
      key: "actions",
      title: "",
      dataIndex: "id",
      fixed: "right",
      render: (value: string) => (
        <Flex gap={5}>
          <Popconfirm
            title="資料を削除"
            description="あなたは本当にこの資料を削除する気ですか！？"
            onConfirm={async () => {
              await mutateDocumentDelete({
                params: {
                  path: {
                    document_id: value,
                  },
                },
              });
              await refetch();
            }}
            okText="はい"
            cancelText="いいえ"
          >
            <Button danger>
              <DeleteOutlined />
            </Button>
          </Popconfirm>
        </Flex>
      ),
    },
  ];

  const expandedRowRender = (record: RowType) => (
    <Table<DocumentRead>
      columns={expandedColumns}
      dataSource={record.documents.map((document) => ({...document, key: document.id}))}
      scroll={{x: "max-content"}}
      pagination={false}
      footer={() => (
        <Button disabled={record.category === null} href={`/documents/new?category_id=${record.category?.id}`}>
          <PlusOutlined />
          新規資料を作成
        </Button>
      )}
    />
  );

  if (isLoading) return <LoadingScreen />;

  return (
    <>
      {contextHolder}
      <Heading1 emoji="📚">資料管理画面</Heading1>
      <Table<RowType>
        columns={columns}
        dataSource={dataSource}
        scroll={{x: "max-content"}}
        expandable={{
          expandedRowRender,
        }}
        pagination={false}
        footer={() => (
          <Button onClick={handleCreateCategory}>
            <PlusOutlined />
            新規資料カテゴリを作成
          </Button>
        )}
      />
    </>
  );
}
