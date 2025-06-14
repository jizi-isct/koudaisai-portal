"use client";

import {Heading1} from "@/components/generic"
import {$apiNoAuth} from "@/lib";

import '@react-pdf-viewer/core/lib/styles/index.css';
import '@react-pdf-viewer/default-layout/lib/styles/index.css';
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
      <ReadDocumentProvider queryClient={$apiNoAuth}>
        <Heading1 emoji="📚">資料一覧</Heading1>
        <ViewDocuments/>
      </ReadDocumentProvider>
    </>
  )
}

