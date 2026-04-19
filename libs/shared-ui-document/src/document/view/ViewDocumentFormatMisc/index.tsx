import {DocumentFormatMiscRead} from "@koudaisai/shared-types";
import {LargeButton} from "@koudaisai/shared-ui";

type ViewDocumentFormatMiscProps = {
  download: () => void,
  format: DocumentFormatMiscRead
}

/**
 * その他の資料を表示するコンポーネント
 * @param fetchClient
 * @param format
 * @constructor
 */
export function ViewDocumentFormatMisc({download, format}: ViewDocumentFormatMiscProps) {
  return (
    <div>
      <LargeButton text={"ダウンロード"} onClick={() => download()}/>
    </div>
  )
}