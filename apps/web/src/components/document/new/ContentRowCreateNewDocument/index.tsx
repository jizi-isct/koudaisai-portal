"use client";

import {ContentRow} from "@/components/generic/ContentRow";
import React from "react";
import {Modal} from "@/components/generic";
import {CreateNewDocument} from "@/components/document/new/CreateNewDocument";
import {useWriteDocumentContext} from "@/contexts/WriteDocumentContext";

type ContentRowCreateNewDocumentProps = {
  documentCategoryId: string | null,
}

export function ContentRowCreateNewDocument({documentCategoryId}: ContentRowCreateNewDocumentProps) {
  const {refetch} = useWriteDocumentContext();
  const [isModalOpen, setIsModalOpen] = React.useState(false);

  const handleCreateNewDocument = async () => {
    setIsModalOpen(false);
    await refetch()
  };
  return (
    <>
      <ContentRow key={0} content={{
        title: "➕ 資料を追加",
        onClick: () => {
          setIsModalOpen(true);
        },
      }}/>
      <Modal isOpen={isModalOpen} setOpen={setIsModalOpen}>
        <CreateNewDocument callback={handleCreateNewDocument} category={documentCategoryId}/>
      </Modal>
    </>
  );
}