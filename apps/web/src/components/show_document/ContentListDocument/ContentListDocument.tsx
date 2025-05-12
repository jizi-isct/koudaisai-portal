import {ContentList} from "@/components/generic";
import {Document} from "@/lib";
import {ContentRowDocument} from "@/components/show_document";

type Props = {
  documents: Array<Document>,
  handleOpenDocument: ((document: Document) => () => void) | ((document: Document) => () => Promise<void>),
  handleDownloadDocument: ((document: Document) => () => void) | ((document: Document) => () => Promise<void>)
}

export function ContentListDocument({documents, handleOpenDocument, handleDownloadDocument}: Props) {
  return (
    <ContentList contents={
      documents.map(
        (document, i) =>
          <ContentRowDocument
            key={`document-row-${i}`}
            document={document}
            handleOpenDocument={handleOpenDocument(document)}
            handleDownloadDocument={handleDownloadDocument(document)}
          />
      )
    }/>
  )
}