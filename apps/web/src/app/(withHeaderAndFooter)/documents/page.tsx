"use client";

import {Heading1} from "@/components/generic"
import {DocumentList} from "@/components/show_document";
import {$apiNoAuth} from "@/lib";

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

