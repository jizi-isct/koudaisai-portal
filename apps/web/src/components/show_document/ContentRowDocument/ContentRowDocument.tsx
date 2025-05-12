import styles from "./ContentRowDocument.module.css"
import {Document} from "@/lib";
import Image from "next/image";

type Props = {
  document: Document,
  handleOpenDocument: () => void,
  handleDownloadDocument: () => void
}

export function ContentRowDocument({document, handleOpenDocument, handleDownloadDocument}: Props) {
  return (
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
  )
}