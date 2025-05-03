"use client";

import {Modal} from "@koudaisai-portal/ui-generic";
import {Document, DocumentCategory} from "@koudaisai-portal/util";
import {EditDocument} from "../EditDocument";

type Props = {
  categories: Array<DocumentCategory>,
  isModalOpen: boolean,
  setModalOpen: (isModalOpen: boolean) => void,
  document: Document,
  setDocument: (document: Document) => void
}

export function EditDocumentModal({categories, isModalOpen, setModalOpen, document, setDocument}: Props) {
  return (
    <Modal isOpen={isModalOpen} setOpen={setModalOpen}>
      <EditDocument categories={categories} document={document} setDocument={setDocument}/>
    </Modal>
  )
}