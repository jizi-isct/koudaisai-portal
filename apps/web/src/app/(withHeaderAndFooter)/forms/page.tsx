"use client";
import {ContentList, Heading1} from "@/components/generic";
import React from "react";
import {formDataNoLogin} from "@/lib/lib";

export default function Page() {
    return (
      <>
        <Heading1 emoji="📃">フォーム一覧</Heading1>
        <ContentList
          contents={
            formDataNoLogin.map((form, i) => ({
              title: form.title,
              onClick: () => {
                window.location.assign(form.url)
              }
            }))
          }
        />
      </>
    )
}