"use client";
import {Heading1} from "@koudaisai/shared-ui";
import React from "react";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {$apiMembers} from "@/lib/api";
import {ViewFormCards} from "@/components/form/view/ViewFormCards";

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
        <Heading1 emoji={"📃"}>フォーム一覧</Heading1>
        <ViewFormCards client={$apiMembers}/>
      </>
    )
}