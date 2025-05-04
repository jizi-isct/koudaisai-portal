"use client";

import {Modal} from "@/components/generic";
import {CreateDocument} from "../CreateDocument";
import {DocumentCategory} from "@/lib";

type Props = {
  categories: Array<DocumentCategory>,
  initialCategory: DocumentCategory,
  isModalOpen: boolean,
  setModalOpen: (isModalOpen: boolean) => void,
  onCreate: () => void
}

export function CreateDocumentModal({
                                      categories,
                                      initialCategory,
                                      isModalOpen,
                                      setModalOpen,
                                      onCreate: onCreate_
                                    }: Props) {
  const onCreate = () => {
    setModalOpen(false);
    onCreate_();
  }
  return (
    <Modal isOpen={isModalOpen} setOpen={setModalOpen}>
      <CreateDocument categories={categories} initialCategory={initialCategory} onCreate={onCreate}/>
    </Modal>
  )
}