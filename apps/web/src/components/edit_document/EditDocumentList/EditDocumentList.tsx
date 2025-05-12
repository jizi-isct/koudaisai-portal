"use client";

import {$apiAdmin, Document, DocumentCategory, fetchClientAdmin} from "@/lib";
import {Button, ContentList, Heading2, Loading, TextInput} from "@/components/generic";
import {EditDocumentModal} from "../EditDocumentModal";
import {CreateDocumentModal} from "../CreateDocumentModal";
import React, {useState} from "react";
import {useQueryClient} from "@tanstack/react-query";
import {ContentRow} from "@/components/generic/ContentRow";

const headingEmojis = ["📕", "📗", "📘", "📙"];

export function EditDocumentList() {
  const queryClient = useQueryClient()
  const {data: categories} = $apiAdmin.useQuery("get", "/document-categories")
  const {data: documents} = $apiAdmin.useQuery("get", "/documents")
  const [selectedDocument, setSelectedDocument] = useState<Document>({})
  const [selectedDocumentCategory, setSelectedDocumentCategory] = useState<DocumentCategory>()
  const [isEditDocumentModalOpen, setIsEditDocumentModalOpen] = useState(false)
  const [isCreateDocumentModalOpen, setIsCreateDocumentModalOpen] = useState(false)

  if (!categories || !documents) return <Loading/>

  const categoryDocumentList = new Map<DocumentCategory | undefined, Array<Document>>()

  for (const category of categories) {
    categoryDocumentList.set(category, [])
  }

  for (const document of documents) {
    const category = categories
      .filter(
        (category) => category.id === document.category
      )[0]
    categoryDocumentList.set(
      category,
      (categoryDocumentList.get(category) ?? []).concat(document)
    )
  }

  const handleDocumentCategoryEdit = async (category: DocumentCategory, title: string) => {
    await fetchClientAdmin.PATCH("/document-categories/{category_id}", {
      body: {
        title: title
      },
      params: {
        path: {
          category_id: category.id!
        }
      }
    })
    await queryClient.refetchQueries()
  }

  const openEditDocumentModal = async (document: Document) => {
    setSelectedDocument(document)
    setIsEditDocumentModalOpen(true)
  }

  const openCreateDocumentModal = async (category: DocumentCategory) => {
    setSelectedDocumentCategory(category)
    setIsCreateDocumentModalOpen(true)
  }

  const handleEditDocument = async (document: Document) => {
    setSelectedDocument(document)
    await fetchClientAdmin.PATCH("/documents/{document_id}", {
      body: document,
      params: {
        path: {
          document_id: document.id!
        }
      }
    })
    await queryClient.refetchQueries()
  }

  const handleCreateDocumentCategory = async () => {
    await fetchClientAdmin.POST("/document-categories", {
      body: {
        title: "新規カテゴリー"
      }
    })
    await queryClient.refetchQueries()
  }

  const onDocumentCreate = async () => {
    await queryClient.refetchQueries()
  }

  if (!categories) return "Loading..."


  return (
    <div>
      <Button text={"新規カテゴリーを作成"} onClick={() => handleCreateDocumentCategory()}/>
      {
        [...categoryDocumentList].map((entry, index) => (
          <React.Fragment key={`fragment-${index}`}>
            <Heading2 emoji={headingEmojis[index % 4]}>
              {
                entry[0]
                  ? <TextInput
                    value={entry[0]?.title}
                    setValue={str => {
                      handleDocumentCategoryEdit(entry[0]!, str)
                    }}
                    paragraph={false}
                  />
                  : "カテゴリなし"
              }
            </Heading2>
            <ContentList
              contents={
                entry[1].map((document, i) => ({
                  title: document.title!,
                  onClick: () => {
                    openEditDocumentModal(document)
                  }
                })).concat({
                  title: "➕ 資料を追加",
                  onClick: () => {
                    openCreateDocumentModal(entry[0]!)
                  }
                }).map((content, i) =>
                  <ContentRow key={`row-${i}`} content={content}/>
                )
              }
            />
          </React.Fragment>
        ))
      }
      <EditDocumentModal
        categories={categories}
        document={selectedDocument}
        setDocument={(document) => handleEditDocument(document)}
        isModalOpen={isEditDocumentModalOpen}
        setModalOpen={setIsEditDocumentModalOpen}
      />
      <CreateDocumentModal
        categories={categories}
        initialCategory={selectedDocumentCategory!}
        isModalOpen={isCreateDocumentModalOpen}
        setModalOpen={setIsCreateDocumentModalOpen}
        onCreate={onDocumentCreate}
      />
    </div>
  )
}