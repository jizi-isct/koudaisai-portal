"use client";

import {Heading1} from "@koudaisai/shared-ui"
import {$apiMembers} from "@/lib/api";

import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {ViewDocuments} from "@/components/document/view/ViewDocuments";
import {ReadDocumentProvider} from "@/contexts/ReadDocumentContext";


export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
 
  return (
    <>
      <ReadDocumentProvider queryClient={$apiMembers}>
        <Heading1 emoji="📚">資料一覧</Heading1>
        <ViewDocuments/>
      </ReadDocumentProvider>
    </>
  )
}

