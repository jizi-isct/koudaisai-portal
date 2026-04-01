"use client";

import {ApiQueryClient} from "@koudaisai/shared-api";
import {LoadingScreen} from "@koudaisai/shared-ui";
import React from "react";
import {FormCard} from "@/components/form/view/FormCard";
import styles from "./ViewFormCards.module.css";

type ViewFormCardsProps = {
  client: ApiQueryClient
}

export function ViewFormCards({client}: ViewFormCardsProps) {
  const {data: forms} = client.useQuery("get", "/forms")

  if (!forms) {
    return <LoadingScreen/>
  }


  return (
    <div className={styles.root}>
      {forms.map((form) => <FormCard key={form.id} form={form}/>)}
    </div>
  )
}