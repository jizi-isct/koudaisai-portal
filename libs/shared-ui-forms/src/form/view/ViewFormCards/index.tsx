"use client";

import {FormCard} from "../FormCard";
import styles from "./ViewFormCards.module.css";
import {FormRead} from "@koudaisai/shared-types";

type ViewFormCardsProps = {
  forms: FormRead[]
}

export function ViewFormCards({forms}: ViewFormCardsProps) {
  return (
    <div className={styles.root}>
      {forms.map((form) => <FormCard key={form.id} form={form}/>)}
    </div>
  )
}
