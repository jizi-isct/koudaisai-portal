"use client";

import styles from "./ContentRowEditDocument.module.css";
import {DocumentRead} from "@/lib";
import Image from "next/image";
import {Modal} from "@/components/generic";
import {useCallback, useState} from "react";
import {EditDocument} from "@/components/document/edit/EditDocument";
import {useWriteDocumentContext} from "@/contexts/WriteDocumentContext";

type Props = {
  document: DocumentRead,
}

export function ContentRowEditDocument({document}: Props) {
  const {deleteDocument, refetch} = useWriteDocumentContext();
  const [isModalOpen, setModalOpen] = useState(false);

  const handleDelete = useCallback(async () => {
    await deleteDocument(document.id);
    await refetch()
  }, [deleteDocument, refetch, document.id]);

  return (
    <div className={styles.root}>
      <div className={styles.document} onClick={() => setModalOpen(true)}>
  <span>
    {document.title}
  </span>
      </div>
      <div
        className={styles.download}
        style={{display: 'flex'}}
        onClick={handleDelete}>
        <Image src={"/generic/delete.svg"} width={24} height={24} alt={"削除アイコン"}/>
        <span>削除</span>
      </div>
      <Modal isOpen={isModalOpen} setOpen={setModalOpen}>
        <EditDocument
          document={document}
        />
      </Modal>
    </div>
  )
}