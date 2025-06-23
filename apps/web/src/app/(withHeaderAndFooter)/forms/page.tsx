"use client";
import {Heading1} from "@/components/generic";
import React from "react";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {$apiMembers, $apiNoAuth, useIsLoggedInMembers} from "@/lib";
import {ViewForms} from "@/components/form/view/ViewForms";
import {ViewFormCards} from "@/components/form/view/ViewFormCards";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const isLoggedIn = useIsLoggedInMembers();

  if (isLoggedIn) {
    return (
      <>
        <Heading1 emoji={"📃"}>フォーム一覧</Heading1>
        <ViewFormCards client={$apiMembers}/>
      </>
    )
  } else {
    return (
      <>
        <Heading1 emoji={"📃"}>フォーム一覧</Heading1>
        <ViewForms client={$apiNoAuth}/>
      </>
    )
  }
}