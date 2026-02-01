"use client";

import {Heading1, LoadingScreen} from "@/components/generic"
import {$apiMembers, $apiNoAuth, useIsLoggedInMembers} from "@/lib";

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
  const isLoggedIn = useIsLoggedInMembers()
  if (isLoggedIn === undefined) {
    return <LoadingScreen/>
  }
  return (
    <>
      <ReadDocumentProvider queryClient={isLoggedIn ? $apiMembers : $apiNoAuth}>
        <Heading1 emoji="📚">資料一覧</Heading1>
        <ViewDocuments/>
      </ReadDocumentProvider>
    </>
  )
}

