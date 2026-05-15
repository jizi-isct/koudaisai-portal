"use client";

import {useDownload, useDownloadUrl} from "@koudaisai/shared-utils";
import { DocumentFormatMiscRead } from "@koudaisai/shared-types";
import {LargeButton, LoadingScreen} from "@koudaisai/shared-ui";
import { fetchClientMembers } from "@/lib/api";

type ViewDocumentFormatMiscProps = {
  format: DocumentFormatMiscRead
}

/**
 * その他の資料を表示するコンポーネント
 * @param format
 * @constructor
 */
export function ViewDocumentFormatMisc({format}: ViewDocumentFormatMiscProps) {
  const {downloadUrl, error} = useDownloadUrl(fetchClientMembers,format.file_key, format.file_name);
  const download = useDownload()
  if (downloadUrl) {
    return (
      <div>
        <LargeButton text={"ダウンロード"} onClick={() => download(downloadUrl, format.file_name)}/>
      </div>
    )
  } else if (error) {
    return <p color={"red"}>{error}</p>
  } else {
    return <LoadingScreen/>
  }
}