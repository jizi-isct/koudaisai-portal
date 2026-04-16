import {DocumentRead} from "@koudaisai/shared-types";
import {ViewDocumentFormatPdf} from "../ViewDocumentFormatPdf";
import {ViewDocumentFormatMarkdown} from "../ViewDocumentFormatMarkdown";
import {ViewDocumentFormatMisc} from "../ViewDocumentFormatMisc";

type ViewDocumentProps = {
  download: () => void,
  document: DocumentRead
}

export function ViewDocument({download, document}: ViewDocumentProps) {
  return (
    <>
      {document.format_pdf && <ViewDocumentFormatPdf download={download} format={document.format_pdf}/>}
      {document.format_markdown && <ViewDocumentFormatMarkdown format={document.format_markdown}/>}
      {document.format_misc && <ViewDocumentFormatMisc download={download} format={document.format_misc}/>}
    </>
  );
}