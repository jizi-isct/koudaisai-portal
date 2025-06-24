"use client";

import {apiQueryClientType} from "@/lib";
import {ContentList, LoadingScreen} from "@/components/generic";
import {ContentRow} from "@/components/generic/ContentRow";
import React from "react";
import {useRouter} from "next/navigation";

type ViewFormsProps = {
  client: apiQueryClientType
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