"use client";

import {$apiAdmin, Document, DocumentCategory, fetchClientAdmin} from "@/lib";
import {Button, ButtonIcon, ContentList, Heading2, Loading, Modal} from "@/components/generic";
import {EditDocumentModal} from "../EditDocumentModal";
import {CreateDocumentModal} from "../CreateDocumentModal";
import React, {useMemo, useState} from "react";
import {useQueryClient} from "@tanstack/react-query";
import {ContentRow} from "@/components/generic/ContentRow";
import {EditDocumentCategory} from "@/components/edit_document";

const headingEmojis = ["📕", "📗", "📘", "📙"];

export function EditDocumentList() {
  const queryClient = useQueryClient()
  const {mutateAsync: patchDocumentCategory} = $apiAdmin.useMutation("patch", "/document-categories/{category_id}")
  const {mutateAsync: deleteDocumentCategory} = $apiAdmin.useMutation("delete", "/document-categories/{category_id}")
  const {data: categories, refetch: refetchCategories} = $apiAdmin.useQuery("get", "/document-categories")
  const {data: documents, refetch: refetchDocuments} = $apiAdmin.useQuery("get", "/documents")
  const [selectedDocument, setSelectedDocument] = useState<Document>({})
  const [selectedDocumentCategory, setSelectedDocumentCategory] = useState<DocumentCategory>()
  const [isEditDocumentModalOpen, setIsEditDocumentModalOpen] = useState(false)
  const [isCreateDocumentModalOpen, setIsCreateDocumentModalOpen] = useState(false)
  const categoryDocumentList = useMemo(() => {
    const categoryDocumentList = new Map<DocumentCategory | undefined, Array<Document>>()

    if (!categories || !documents) return undefined

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

    return categoryDocumentList
  }, [categories, documents])

  if (!categoryDocumentList || !categories) return <Loading/>


  const handleDocumentCategoryEdit = async (category: DocumentCategory) => {
    await patchDocumentCategory(
      {
        body: {
          title: category.title
        },
        params: {
          path: {
            category_id: category.id!
          }
        }
      }
    )
    await refetchCategories()
    await refetchDocuments()
  }

  const handleDocumentCategoryDelete = (category: DocumentCategory) => async () => {
    await deleteDocumentCategory(
      {
        params: {
          path: {
            category_id: category.id!
          }
        }
      }
    )
    await refetchCategories()
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
    await refetchCategories()
  }

  const onDocumentCreate = async () => {
    await queryClient.refetchQueries()
  }


  return (
    <div>
      <Button text={"新規カテゴリーを作成"} onClick={() => handleCreateDocumentCategory()}/>
      {
        [...categoryDocumentList].map((entry, index) => (
          <React.Fragment key={`fragment-${index}`}>
            {
              entry[0] ?
                <DocumentCategoryHeading
                  documentCategory={entry[0]}
                  setDocumentCategory={handleDocumentCategoryEdit}
                  deleteDocumentCategory={handleDocumentCategoryDelete(entry[0])}
                  emoji={headingEmojis[index % 4]!}
                /> :
                <Heading2 emoji={"⚠️"}>カテゴリなし</Heading2>
            }
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

type DocumentCategoryHeadingProps = {
  documentCategory: DocumentCategory,
  setDocumentCategory: ((documentCategory: DocumentCategory) => void | Promise<void>),
  deleteDocumentCategory: (() => void | Promise<void>),
  emoji: string
}

function DocumentCategoryHeading({
                                   documentCategory,
                                   setDocumentCategory,
                                   deleteDocumentCategory,
                                   emoji
                                 }: DocumentCategoryHeadingProps) {
  const [isEditModalOpen, setIsEditModalOpen] = useState(false)

  const handleEdit = () => {
    setIsEditModalOpen(true)
  }

  const handleDelete = async () => {
    await deleteDocumentCategory()
  }

  return (
    <>
      <Heading2 emoji={emoji}>
        {documentCategory.title}
        <ButtonIcon iconType={"edit"} onClick={handleEdit}/>
        <ButtonIcon iconType={"delete"} onClick={handleDelete}/>
      </Heading2>
      <Modal isOpen={isEditModalOpen} setOpen={setIsEditModalOpen}>
        <EditDocumentCategory documentCategory={documentCategory} setDocumentCategory={setDocumentCategory}/>
      </Modal>
    </>
  )
}