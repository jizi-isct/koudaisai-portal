"use client";

import {useReadDocumentContext} from "@/contexts/ReadDocumentContext";
import {ContentList, LoadingScreen} from "@koudaisai/shared-ui";
import {HeadingViewDocumentCategory} from "@/components/documentCategory/view/HeadingViewDocumentCategory";
import {ContentRowViewDocument} from "@/components/document/view/ContentRowViewDocument";

const headingEmojis = ["📕", "📗", "📘", "📙"];

export function ViewDocuments() {
  const {documents, isLoading, fetchError} = useReadDocumentContext()

  if (!documents) {
    if (isLoading) {
      return <LoadingScreen/>
    } else {
      return <p color={"red"}>資料の取得に失敗しました: {fetchError?.message}</p>
    }
  }

  return (
    documents.map(({category, documents}, i) => (
      <section key={`document-category-${i}`}>
        {
          category ? (
            <HeadingViewDocumentCategory documentCategory={category} emoji={headingEmojis[i % headingEmojis.length]}/>
          ) : (
            <></>
          )
        }

        <ContentList contents={
          documents.map((document, j) => (
            <ContentRowViewDocument key={`document-${i}.${j}`} document={document}/>
          ))
        }/>
      </section>
    ))
  )
}