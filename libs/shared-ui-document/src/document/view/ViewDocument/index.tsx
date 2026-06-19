import { DocumentRead } from '@koudaisai/shared-types';
import { ViewDocumentFormatPdf } from '../ViewDocumentFormatPdf';
import { ViewDocumentFormatMarkdown } from '../ViewDocumentFormatMarkdown';
import { ViewDocumentFormatMisc } from '../ViewDocumentFormatMisc';

type ViewDocumentProps = {
  download: () => void;
  document: DocumentRead;
};

export function ViewDocument({ download, document }: ViewDocumentProps) {
  return (
    <>
      {document.format === 'pdf' && (
        <ViewDocumentFormatPdf download={download} format={document} />
      )}
      {document.format === 'markdown' && (
        <ViewDocumentFormatMarkdown format={document} />
      )}
      {document.format === 'misc' && (
        <ViewDocumentFormatMisc download={download} format={document} />
      )}
    </>
  );
}
