"use client";

import {DocumentFormatMiscRead, useDownload, useDownloadUrl} from "@/lib";
import {Button, LoadingScreen} from "@/components/generic";

type ViewDocumentFormatMiscProps = {
  format: DocumentFormatMiscRead
}

/**
 * その他の資料を表示するコンポーネント
 * @param format
 * @constructor
 */
export function ViewDocumentFormatMisc({format}: ViewDocumentFormatMiscProps) {
  const {downloadUrl, error} = useDownloadUrl(format.file_key, format.file_name);
  const download = useDownload()
  if (downloadUrl) {
    return (
      <div>
        <Button text={"ダウンロード"} onClick={() => download(downloadUrl, format.file_name)}/>
      </div>
    )
  } else if (error) {
    return <p color={"red"}>{error}</p>
  } else {
    return <LoadingScreen/>
  }
}