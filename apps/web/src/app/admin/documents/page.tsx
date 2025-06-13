"use client";

import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Heading1} from "@/components/generic";
import {WriteDocumentProvider} from "@/contexts/WriteDocumentContext";
import {$apiAdmin} from "@/lib";
import {ManageDocuments} from "@/components/document/common/ManageDocuments";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  return (
    <WriteDocumentProvider queryClient={$apiAdmin}>
      <Heading1 emoji={"📚"}>資料管理画面</Heading1>
      <ManageDocuments/>
    </WriteDocumentProvider>
  )
}