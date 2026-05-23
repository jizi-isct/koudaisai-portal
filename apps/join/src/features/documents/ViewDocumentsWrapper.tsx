'use client';

import type {
  DocumentCategoryRead,
  DocumentRead,
} from '@koudaisai/shared-types';
import { downloadDocument, useDownload } from '@koudaisai/shared-utils';
import { useCallback } from 'react';
import { useApiFetchClientWithNoAuth } from '@koudaisai-portal/shared-api-utils';
import { ViewDocuments } from '@koudaisai-portal/shared-ui-document';

interface Props {
  documents: Array<{
    category: DocumentCategoryRead | null;
    documents: DocumentRead[];
  }>;
  apiUrl: string;
}

export default function ViewDocumentsWrapper({ documents, apiUrl }: Props) {
  const download = useDownload();
  const fetchClient = useApiFetchClientWithNoAuth(apiUrl);
  const handleDownload = useCallback(
    async (documentId: string) => {
      const matchedEntries = documents
        .flatMap((documentCategory) => documentCategory.documents)
        .filter((document) => document.id === documentId);

      if (matchedEntries.length === 0) {
        throw new Error('document not found');
      }

      await downloadDocument(matchedEntries[0], fetchClient, download);
    },
    [documents, fetchClient, download],
  );

  return <ViewDocuments documents={documents} download={handleDownload} />;
}
