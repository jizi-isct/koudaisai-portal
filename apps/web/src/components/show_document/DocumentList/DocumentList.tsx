"use client";

import {Document, DocumentCategory, fetchClientNoAuth, getDownloadUrl, useDownload} from "@/lib";
import {Heading2} from "@/components/generic";
import React, {useEffect, useState} from "react";
import {DocumentModal} from "../DocumentModal";
import {ContentListDocument} from "@/components/show_document";

type Props = {
  documents: Array<Document>
}

const headingEmojis = ["📕", "📗", "📘", "📙"];

export function DocumentList({documents}: Props) {
  const category_ids = new Map<string | undefined, Array<Document>>()
  const [categories, setCategories] = useState<Array<[DocumentCategory, Array<Document>]>>([])
  const [selectedDocument, setSelectedDocument] = useState<Document>({})
  const [isModalOpen, setIsModalOpen] = useState(false)
  const download = useDownload()

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

  const handleDocumentDownload = (document: Document) => async () => {
    if (document.format_pdf) {
      const key = document.format_pdf.file_key
      const fileName = document.format_pdf.file_name
      const {data, error} = await getDownloadUrl(key, fileName)
      if (data) {
        download(data.presigned_url!, fileName)
      } else {
        alert("資料ダウンロード中にエラーが発生しました。: " + error)
      }
    } else if (document.format_misc) {
      const key = document.format_misc.file_key
      const fileName = document.format_misc.file_name
      const {data, error} = await getDownloadUrl(key, fileName)
      if (data) {
        download(data.presigned_url!, fileName)
      } else {
        alert("資料ダウンロード中にエラーが発生しました。: " + error)
      }
    } else if (document.format_markdown) {
      const blob = new Blob([document.format_markdown.content], {type: "text/markdown;charset=utf-8;"})
      const url = URL.createObjectURL(blob)
      download(url, `${document.title}.md`)
    }
  }

  const handleDocumentOpen = (document: Document) => async () => {
    if (document.format_misc) {
      await handleDocumentDownload(document)()
    } else {
      openDocumentModal(document)
    }
  }

  if (!categories) return "Loading..."


  return (
    <div>
      {
        categories.map((entry, index) => (
          <React.Fragment key={`fragment-${index}`}>
            <Heading2 emoji={headingEmojis[index % 4]}>{entry[0].title}</Heading2>
            <ContentListDocument
              documents={entry[1]}
              handleDownloadDocument={handleDocumentDownload}
              handleOpenDocument={handleDocumentOpen}
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