'use client';

import type { ApiFetchClient } from '@koudaisai/shared-api';
import { useCallback, useEffect, useState } from 'react';
import { getDownloadUrl } from './file';

export function useDownloadUrl(
  fetchClient: ApiFetchClient,
  fileKey: string,
  fileName: string,
) {
  const [downloadUrl, setDownloadUrl] = useState<string | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);

  useEffect(() => {
    (async () => {
      const { data, error: error_ } = await getDownloadUrl(
        fetchClient,
        fileKey,
        fileName,
      );
      if (data) {
        setDownloadUrl(data.presigned_url);
        setError(undefined);
      } else {
        setDownloadUrl(undefined);
        setError(`Error fetching download URL: ${error_}`);
      }
    })();
  }, [fetchClient, fileKey, fileName]);

  return {
    downloadUrl,
    error,
  };
}

export function useDownload() {
  return useCallback((url: string, fileName: string) => {
    download(url, fileName);
  }, []);
}

export function download(url: string, fileName: string) {
  const a = document.createElement('a');
  a.href = url;
  a.download = fileName;
  a.style.display = 'none';
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);

  if (url.startsWith('blob:')) {
    URL.revokeObjectURL(url);
  }
}
