"use client";

import {apiQueryClientType} from "@/lib";
import {LoadingScreen} from "@/components/generic";
import React from "react";
import {FormCard} from "@/components/form/view/FormCard";
import styles from "./ViewFormCards.module.css";

type ViewFormCardsProps = {
  client: apiQueryClientType
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