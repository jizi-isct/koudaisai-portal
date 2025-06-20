"use client";

import {DocumentFormatMiscRead, DocumentFormatMiscUpdate, fetchClientAdmin} from "@/lib";
import {useCallback} from "react";
import {FileUploader} from "@/components/common/FileUploader";

type EditDocumentFormatMiscProps = {
  format: DocumentFormatMiscRead,
  updateFormat: (format: DocumentFormatMiscUpdate) => void
}

export function EditDocumentFormatMisc({updateFormat}: EditDocumentFormatMiscProps) {
  const handleFileUpload = useCallback((fileKey: string, fileName: string) => {
    updateFormat({
      file_key: fileKey,
      file_name: fileName,
    })
  }, [updateFormat])

  return (
    <>
      <label>
        ファイルをアップロード
        <FileUploader callback={handleFileUpload} client={fetchClientAdmin}/>
      </label>
    </>
  )
}