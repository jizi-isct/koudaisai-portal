"use client";

import {Heading1, LoadingScreen} from "@/components/generic"
import {$apiMembers, $apiNoAuth, fetchClientMembers, fetchClientNoAuth, useIsLoggedInMembers} from "@/lib";

import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {ViewDocuments} from "@koudaisai-portal/shared-ui-document";
import {ReadDocumentProvider, useReadDocumentContext} from "@/contexts/ReadDocumentContext";
import {ApiFetchClient} from "@koudaisai/shared-api";


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
  const fetchClient = isLoggedIn ? fetchClientMembers : fetchClientNoAuth
  return (
    <>
      <ReadDocumentProvider queryClient={isLoggedIn ? $apiMembers : $apiNoAuth}>
        <Heading1 emoji="📚">資料一覧</Heading1>
        <ViewDocumentsWithContext fetchClient={fetchClient}/>
      </ReadDocumentProvider>
    </>
  )
}

function ViewDocumentsWithContext({fetchClient}: {fetchClient: ApiFetchClient}) {
  const {documents, isLoading, fetchError} = useReadDocumentContext()
  return <ViewDocuments documents={documents} isLoading={isLoading} fetchError={fetchError} fetchClient={fetchClient}/>
}