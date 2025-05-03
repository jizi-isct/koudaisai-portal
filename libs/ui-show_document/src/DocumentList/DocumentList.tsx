import {Document, DocumentCategory, fetchClientNoAuth} from "@koudaisai-portal/util";
import {ContentList, Heading2} from "@koudaisai-portal/ui-generic";
import {DocumentModal} from "../DocumentModal/DocumentModal";
import React, {useState} from "react";

type Props = {
  documents: Array<Document>
}

const headingEmojis = ["📕", "📗", "📘", "📙"];

export function DocumentList({documents}: Props) {
  const category_ids = new Map<string | undefined, Array<Document>>()
  const [categories, setCategories] = useState<Array<[DocumentCategory, Array<Document>]>>()
  const [selectedDocument, setSelectedDocument] = useState<Document>({})
  const [isModalOpen, setIsModalOpen] = useState(false)

  for (const document of documents) {
    category_ids.set(document.category, (category_ids.get(document.category) ?? []).concat(document))
  }

  useState(async () => {
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
  })

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