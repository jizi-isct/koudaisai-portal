"use client";

import {DocumentFormatPdfRead, useDownload, useDownloadUrl} from "@/lib";
import {Button, LoadingScreen} from "@/components/generic";

type ViewDocumentFormatPdfProps = {
  format: DocumentFormatPdfRead
}

/**
 * PDF資料を表示するコンポーネント
 * @param formatPdf
 * @constructor
 */
export function ViewDocumentFormatPdf({format}: ViewDocumentFormatPdfProps) {
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