"use client";

import {apiQueryClientType} from "@/lib";
import {LoadingScreen} from "@/components/generic";
import React from "react";
import {FormCard} from "@/components/form/view/FormCard";

type ViewFormCardsProps = {
  client: apiQueryClientType
}

export function ViewFormCards({client}: ViewFormCardsProps) {
  const {data: forms} = client.useQuery("get", "/forms")

  if (!forms) {
    return <LoadingScreen/>
  }


  return forms.map((form) => <FormCard key={form.id} form={form}/>)
}