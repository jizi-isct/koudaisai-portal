"use client";

import {DocumentRead} from "@koudaisai/shared-types";
import {downloadDocument, useDownload} from "@koudaisai/shared-utils";
import {useCallback} from "react";
import {useApiFetchClientWithNoAuth} from "@koudaisai-portal/shared-api-utils"
import {ViewDocument} from "@koudaisai-portal/shared-ui-document";

interface Props {
  document: DocumentRead
}

export default function ViewDocumentWrapper({document}: Props) {
  const download = useDownload()
  const fetchClient = useApiFetchClientWithNoAuth("https://portal.koudaisai.jp/api/v2")
  const handleDownload = useCallback(async () => {
    await downloadDocument(document, fetchClient, download)
  }, [document, fetchClient, download])
  return (
    <ViewDocument document={document} download={handleDownload}/>
  )
}
