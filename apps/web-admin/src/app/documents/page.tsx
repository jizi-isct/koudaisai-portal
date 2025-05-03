"use client";

import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1} from "@koudaisai-portal/ui-generic";
import dynamic from "next/dynamic";

const EditDocumentList = dynamic(
  () =>
    import('@koudaisai-portal/ui-edit_document').then(
      (m) => m.EditDocumentList      // named export を取り出す
    ),
  {
    ssr: false,                      // ★ サーバでは一切レンダーしない
    loading: () => <p>Loading…</p>,  // 任意: 読み込み中プレースホルダ
  }
);

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  return (
    <div>
      <Heading1 emoji={"📚"}>資料管理画面</Heading1>
      <EditDocumentList/>
    </div>
  )
}