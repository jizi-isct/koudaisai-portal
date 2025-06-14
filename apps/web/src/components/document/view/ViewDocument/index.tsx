"use client";

import {DocumentRead} from "@/lib";
import {ViewDocumentFormatPdf} from "@/components/document/view/ViewDocumentFormatPdf";
import {ViewDocumentFormatMarkdown} from "@/components/document/view/ViewDocumentFormatMarkdown";
import {ViewDocumentFormatMisc} from "@/components/document/view/ViewDocumentFormatMisc";

type ViewDocumentProps = {
  document: DocumentRead
}

export function ViewDocument({document}: ViewDocumentProps) {
  return (
    <>
      {document.format_pdf && <ViewDocumentFormatPdf format={document.format_pdf}/>}
      {document.format_markdown && <ViewDocumentFormatMarkdown format={document.format_markdown}/>}
      {document.format_misc && <ViewDocumentFormatMisc format={document.format_misc}/>}
    </>
  );
}