"use client";

import { DocumentRead } from "@koudaisai/shared-types";
import { useDownload,getDownloadUrl } from "@koudaisai/shared-utils";
import styles from "./ContentRowViewDocument.module.css";
import Image from "next/image";
import {Modal} from "@koudaisai/shared-ui";
import {ViewDocument} from "@/components/document/view/ViewDocument";
import {useCallback, useState} from "react";
import { fetchClientMembers } from "@/lib/api";

type ContentRowViewDocumentProps = {
  document: DocumentRead
}

/**
 * 資料 - ContentRowを表示する
 * @param document
 * @constructor
 */
export function ContentRowViewDocument({document}: ContentRowViewDocumentProps) {
  const download = useDownload()
  const [isModalOpen, setModalOpen] = useState(false);

  const handleDownloadDocument = useCallback(async () => {
    if (document.format_pdf) {
      const {data: downloadUrl} = await getDownloadUrl(fetchClientMembers,document.format_pdf.file_key, document.format_pdf.file_name)
      if (downloadUrl?.presigned_url) {
        download(downloadUrl.presigned_url, document.format_pdf.file_name)
      }
    }
    if (document.format_markdown) {
      const blob = new Blob([document.format_markdown.content], {type: "text/markdown;charset=utf-8;"})
      const url = URL.createObjectURL(blob)
      download(url, `${document.title}.md`)
    }
    if (document.format_misc) {
      const {data: downloadUrl} = await getDownloadUrl(fetchClientMembers,document.format_misc.file_key, document.format_misc.file_name)
      if (downloadUrl?.presigned_url) {
        download(downloadUrl.presigned_url, document.format_misc.file_name)
      }
    }
  }, [download, document])

  const handleOpenDocument = useCallback(async () => {
    if (document.format_misc) {
      await handleDownloadDocument()
    } else {
      setModalOpen(true)
    }
  }, [document.format_misc, setModalOpen, handleDownloadDocument])

  return (
    <>
      <div className={styles.root}>
        <div className={styles.document} onClick={handleOpenDocument}>
        <span>
          {document.title}
        </span>
        </div>
        <div className={styles.download} onClick={handleDownloadDocument}>
          <Image src={"/generic/download.svg"} width={24} height={24} alt={"ダウンロード"}/>
          <span>ダウンロード</span>
        </div>
      </div>
      <Modal isOpen={isModalOpen} setOpen={setModalOpen}>
        <ViewDocument document={document}/>
      </Modal>
    </>
  )
}