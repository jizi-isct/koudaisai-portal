"use client";

import {ApiQueryClient} from "@koudaisai/shared-api";
import {ContentRow ,ContentList, LoadingScreen} from "@koudaisai/shared-ui";
import React from "react";
import {useRouter} from "next/navigation";

type ViewFormsProps = {
  client: ApiQueryClient
}

export function ViewForms({client}: ViewFormsProps) {
  const {data: forms} = client.useQuery("get", "/forms")
  const router = useRouter()

  if (!forms) {
    return <LoadingScreen/>
  }

  return (
    <ContentList
      contents={
        forms.map((form) => ({
          title: form.form_name,
          onClick: () => {
            if ('type_external' in form) {
              router.push(form.type_external.form_url)
            }
          }
        })).map((content, i) => <ContentRow content={content} key={`row-${i}`}/>)
      }
    />
  )
}