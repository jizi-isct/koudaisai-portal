import {ButtonIcon, Heading2, Modal} from "@/components/generic";
import React, {useState} from "react";
import {useWriteDocumentContext} from "@/contexts/WriteDocumentContext";
import {DocumentCategoryRead} from "@/lib";
import {EditDocumentCategory} from "@/components/documentCategory/edit/EditDocumentCategory";

type DocumentCategoryHeadingProps = {
  documentCategory: DocumentCategoryRead
  emoji: string
}

export function HeadingEditDocumentCategory({documentCategory, emoji}: DocumentCategoryHeadingProps) {
  const {
    refetch,
    deleteDocumentCategory
  } = useWriteDocumentContext()

  const [isEditModalOpen, setIsEditModalOpen] = useState(false)

  const handleEdit = () => {
    setIsEditModalOpen(true)
  }

  const handleDelete = async () => {
    await deleteDocumentCategory(documentCategory.id)
    await refetch()
  }

  return (
    <>
      <Heading2 emoji={documentCategory.emoji ?? emoji}>
        {documentCategory.title}
        <ButtonIcon iconType={"edit"} onClick={handleEdit}/>
        <ButtonIcon iconType={"delete"} onClick={handleDelete}/>
      </Heading2>
      <Modal isOpen={isEditModalOpen} setOpen={setIsEditModalOpen}>
        <EditDocumentCategory
          documentCategory={documentCategory}
          finish={() => setIsEditModalOpen(false)}
        />
      </Modal>
    </>
  )
}