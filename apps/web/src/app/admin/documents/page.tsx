"use client";

import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1} from "@/components/generic";
import {EditDocumentList} from "@/components/edit_document";

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