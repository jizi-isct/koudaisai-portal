import type {
  DocumentCategoryRead,
  DocumentRead,
} from '@koudaisai/shared-types';
import { download, getDownloadUrl } from '@koudaisai/shared-utils';
import { ViewDocuments as SharedViewDocuments } from '@koudaisai-portal/shared-ui-document';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { useEffect, useState } from 'react';
import { api } from '@/features/api/api';

type DocumentsByCategory = {
  category: DocumentCategoryRead | null;
  documents: DocumentRead[];
};

export function ViewDocumentsWrapper() {
  const [documents, setDocuments] = useState<DocumentsByCategory[] | null>(
    null,
  );
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      const { data, error } = await api.GET('/documents/by-category');

      if (error) {
        setError(`${error}`);
        setDocuments([]);
        return;
      }

      setDocuments(
        (data ?? []).map((entry) => ({
          category: entry.category ?? null,
          documents: entry.documents,
        })),
      );
    })().catch((caughtError) => {
      setError(`${caughtError}`);
      setDocuments([]);
    });
  }, []);

  const handleDownloadDocument = async (documentId: string) => {
    const document = documents
      ?.flatMap(({ documents }) => documents)
      .find((document) => document.id === documentId);

    if (!document) return;

    if (document.format === 'pdf') {
      const { data: downloadUrl } = await getDownloadUrl(
        api,
        document.file_key,
        document.file_name,
      );
      if (downloadUrl?.presigned_url) {
        download(downloadUrl.presigned_url, document.file_name);
      }
    }

    if (document.format === 'markdown') {
      const blob = new Blob([document.content], {
        type: 'text/markdown;charset=utf-8;',
      });
      const url = URL.createObjectURL(blob);
      download(url, `${document.title}.md`);
    }

    if (document.format === 'misc') {
      const { data: downloadUrl } = await getDownloadUrl(
        api,
        document.file_key,
        document.file_name,
      );
      if (downloadUrl?.presigned_url) {
        download(downloadUrl.presigned_url, document.file_name);
      }
    }
  };

  if (!documents) {
    return <LoadingScreen />;
  }

  if (error) {
    return <p>資料の取得に失敗しました: {error}</p>;
  }

  return (
    <SharedViewDocuments
      documents={documents}
      download={handleDownloadDocument}
    />
  );
}
