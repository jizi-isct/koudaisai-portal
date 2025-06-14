"use client";

import {DocumentFormatPdfRead, useDownloadUrl} from "@/lib";
import {LoadingScreen} from "@/components/generic";
import {Viewer, Worker} from "@react-pdf-viewer/core";

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

  if (downloadUrl) {
    return (
      <Worker workerUrl={`https://unpkg.com/pdfjs-dist@3.11.174/build/pdf.worker.min.js`}>
        <Viewer
          fileUrl={downloadUrl}
        />
      </Worker>
    )
  } else if (error) {
    return <p color={"red"}>{error}</p>
  } else {
    return <LoadingScreen/>
  }
}