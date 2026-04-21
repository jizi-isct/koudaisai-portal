import {DocumentFormatPdfRead} from "@koudaisai/shared-types";
import {Button} from "@koudaisai/shared-ui";

type ViewDocumentFormatPdfProps = {
  download: () => void,
  format: DocumentFormatPdfRead
}

/**
 * PDF資料を表示するコンポーネント
 * @param formatPdf
 * @constructor
 */
export function ViewDocumentFormatPdf({download}: ViewDocumentFormatPdfProps) {
  return (
    <div>
      <Button text={"ダウンロード"} onClick={() => download()}/>
    </div>
  )
}