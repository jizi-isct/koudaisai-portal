"use client";

import {Document, DocumentCategory, fetchClientNoAuth} from "@/lib";
import {ContentList, Heading2} from "@/components/generic";
import React, {useEffect, useState} from "react";
import {DocumentModal} from "../DocumentModal";

type Props = {
  documents: Array<Document>
}

const headingEmojis = ["📕", "📗", "📘", "📙"];

export function DocumentList({documents}: Props) {
  console.log("a")
  const category_ids = new Map<string | undefined, Array<Document>>()
  console.log("b")
  const [categories, setCategories] = useState<Array<[DocumentCategory, Array<Document>]>>([])
  console.log("c")
  const [selectedDocument, setSelectedDocument] = useState<Document>({})
  console.log("d")
  const [isModalOpen, setIsModalOpen] = useState(false)
  console.log("e")

  for (const document of documents) {
    category_ids.set(document.category, (category_ids.get(document.category) ?? []).concat(document))
  }

  useEffect(() => {
    (async () => {
      const categories = new Array<[DocumentCategory, Array<Document>]>();
      for (const entry of category_ids.entries()) {
        if (entry[0]) {
          const {data} = await fetchClientNoAuth.GET("/document-categories/{category_id}",
            {
              params: {
                path: {
                  category_id: entry[0]
                }
              }
            }
          )
          if (data) {
            categories.push([data, entry[1]])
          } else {
            categories.push(
              [{
                id: entry[0],
                title: "ERROR: CATEGORY NOT FOUND"
              },
                entry[1]]
            )
          }
        } else {
          categories.push(
            [{
              id: "",
              title: "カテゴリなし"
            },
              entry[1]]
          )
        }
      }
      setCategories(categories)
    })()
  }, [documents])

  const openDocumentModal = (document: Document) => {
    setSelectedDocument(document)
    setIsModalOpen(true)
  }

  if (!categories) return "Loading..."


  return (
    <div>
      {
        categories.map((entry, index) => (
          <React.Fragment key={`fragment-${index}`}>
            <Heading2 emoji={headingEmojis[index % 4]}>{entry[0].title}</Heading2>
            <ContentList
              contents={
                entry[1].map((document, i) => ({
                  title: document.title!,
                  onClick: () => {
                    openDocumentModal(document)
                  }
                }))
              }
            />
          </React.Fragment>
        ))
      }
      <DocumentModal
        document={selectedDocument}
        isModalOpen={isModalOpen}
        setModalOpen={setIsModalOpen}
      />
    </div>
  )
}