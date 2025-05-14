import styles from "./EditDocumentRaw.module.css";
import {Document} from "@/lib";
import Image from "next/image";

type Props = {
    document: Document,
    handleOpenDocument: () => void,
  handleDeleteDocument?: () => void
}

export function EditDocumentRaw({document, handleOpenDocument, handleDeleteDocument}: Props) {
  return (
    <div className={styles.root}>
        <div className={styles.document} onClick={handleOpenDocument}>
        <span>
          {document.title}
        </span>
      </div>
      <div 
        className={styles.download}
        style={{ display: handleDeleteDocument ? 'flex' : 'none' }}
        onClick={handleDeleteDocument} >
        <Image src={"/generic/delete.svg"} width={24} height={24} alt={"ダウンロード"}/>
        <span>削除</span>
      </div>
    </div>
  )
}