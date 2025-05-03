"use client";

import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {EditDocumentList} from "@koudaisai-portal/ui-edit_document";
import {Heading1} from "@koudaisai-portal/ui-generic";

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