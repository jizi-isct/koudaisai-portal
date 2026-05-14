"use client";

import {DocumentRead} from "@koudaisai/shared-types"
import styles from "./ContentRowViewDocument.module.css";
import {Modal} from "@koudaisai/shared-ui";
import {ViewDocument} from "../ViewDocument";
import {useCallback, useState} from "react";
import downloadImg from "./download.svg";

type ContentRowViewDocumentProps = {
  download: () => void
  document: DocumentRead
}

/**
 * 資料 - ContentRowを表示する
 * @param fetchClient
 * @param document
 * @constructor
 */
export function ContentRowViewDocument({download, document}: ContentRowViewDocumentProps) {
  const [isModalOpen, setModalOpen] = useState(false);

  const handleOpenDocument = useCallback(async () => {
    if (document.format_misc) {
      download()
    } else {
      setModalOpen(true)
    }
  }, [document.format_misc, setModalOpen, download])

  return (
    <>
      <div className={styles.root}>
        <div className={styles.document} onClick={handleOpenDocument}>
        <span>
          {document.title}
        </span>
        </div>
        <div className={styles.download} onClick={download}>
          <img src={downloadImg} width={24} height={24} alt={"ダウンロード"}/>
          <span>ダウンロード</span>
        </div>
      </div>
      <Modal isOpen={isModalOpen} setOpen={setModalOpen}>
        <ViewDocument download={download} document={document}/>
      </Modal>
    </>
  )
}
