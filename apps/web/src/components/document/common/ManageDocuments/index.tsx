"use client";

import {useWriteDocumentContext} from "@/contexts/WriteDocumentContext";
import {Button, ContentList, Heading2, LoadingScreen} from "@/components/generic";
import React from "react";
import {HeadingEditDocumentCategory} from "@/components/documentCategory/edit/HeadingEditDocumentCategory";
import {ContentRowEditDocument} from "@/components/document/edit/ContentRowEditDocument";
import {ContentRowCreateNewDocument} from "@/components/document/new/ContentRowCreateNewDocument";

const headingEmojis = ["📕", "📗", "📘", "📙"];

/**
 * 資料の管理画面を表示するコンポーネントです
 * @constructor
 */
export function ManageDocuments() {
  const {
    documents,
    categories,
    refetch,
    createDocumentCategory,
    isLoading
  } = useWriteDocumentContext()

  const handleCreateDocumentCategory = async () => {
    await createDocumentCategory({
      title: "新規カテゴリー",
      emoji: null
    });
    await refetch();
  }

  if (isLoading || !documents || !categories) {
    return (
      <LoadingScreen/>
    );
  }

  return (
    <div>
      <Button text={"新規カテゴリーを作成"} onClick={() => handleCreateDocumentCategory()}/>
      {
        documents.map(({category, documents}, index) => (
          <section key={category?.id ?? `no-category-${index}`}>
            {
              category ? (
                <HeadingEditDocumentCategory
                  documentCategory={category}
                  emoji={headingEmojis[index % headingEmojis.length]}
                />
              ) : (
                <Heading2 emoji={"⚠️"}>
                  カテゴリーなし
                </Heading2>
              )
            }

            <ContentList contents={
              documents.map((document) => (
                <ContentRowEditDocument
                  key={document.id}
                  document={document}
                />
              )).concat(
                category ? [
                  <ContentRowCreateNewDocument key={0} documentCategoryId={category.id}/>
                ] : []
              )
            }/>
          </section>
        ))
      }
    </div>
  )
}