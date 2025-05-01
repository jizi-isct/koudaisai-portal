"use client";

import {Heading1} from "@koudaisai-portal/ui-generic"
import {DocumentList} from "@koudaisai-portal/ui-show_document"
import {$apiNoAuth} from "@koudaisai-portal/util";
import "@koudaisai-portal/ui-generic/css"

import '@react-pdf-viewer/core/lib/styles/index.css';
import '@react-pdf-viewer/default-layout/lib/styles/index.css';
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";


export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const {data: documents} = $apiNoAuth.useQuery(
    "get",
    "/documents"
  )

  return (
    <>
      <Heading1 emoji="📚">資料一覧</Heading1>
      {documents ? <DocumentList documents={documents}/> : "Loading..."}
    </>
  )
}

