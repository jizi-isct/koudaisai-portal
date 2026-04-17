import {getDownloadUrl} from "./file";
import {DocumentRead} from "@koudaisai/shared-types";
import {ApiFetchClient} from "@koudaisai/shared-api";

export async function downloadDocument(document: DocumentRead, fetchClient: ApiFetchClient, download: (url: string, fileName: string) => void) {
  if (document.format_pdf) {
    const {data: downloadUrl} = await getDownloadUrl(fetchClient, document.format_pdf.file_key, document.format_pdf.file_name)
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
    const {data: downloadUrl} = await getDownloadUrl(fetchClient, document.format_misc.file_key, document.format_misc.file_name)
    if (downloadUrl?.presigned_url) {
      download(downloadUrl.presigned_url, document.format_misc.file_name)
    }
  }
}