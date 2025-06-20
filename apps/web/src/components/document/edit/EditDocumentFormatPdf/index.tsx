"use client";

import {DocumentFormatPdfRead, DocumentFormatPdfUpdate, fetchClientAdmin} from "@/lib";
import {useCallback} from "react";
import {FileUploader} from "@/components/common/FileUploader";

type EditDocumentFormatPdfProps = {
  format: DocumentFormatPdfRead,
  updateFormat: (format: DocumentFormatPdfUpdate) => void
}

export function EditDocumentFormatPdf({updateFormat}: EditDocumentFormatPdfProps) {
  const handleFileUpload = useCallback((fileKey: string, fileName: string) => {
    updateFormat({
      file_key: fileKey,
      file_name: fileName,
    })
  }, [updateFormat])

  return (
    <>
      <label>
        pdfファイルをアップロード
        <FileUploader callback={handleFileUpload} fileType={"application/pdf"} client={fetchClientAdmin}/>
      </label>
    </>
  )
}