// apps/web/src/lib/util.ts から以下の関数をコピー
import type {ApiFetchClient} from "@koudaisai/shared-api";
import {useCallback, useEffect, useState} from "react";

export async function getDownloadUrl(fetchClient: ApiFetchClient, fileKey: string, fileName: string) {
  return await fetchClient.GET(
    "/files/download",
    {
      params: {
        query: {
          key: fileKey,
          file_name: fileName
        }
      }
    }
  )
}

export function useDownloadUrl(fileKey: string, fileName: string) {
  const [downloadUrl, setDownloadUrl] = useState<string | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);

  useEffect(() => {
    (async () => {
      const {data, error: error_} = await getDownloadUrl(fileKey, fileName)
      if (data) {
        setDownloadUrl(data.presigned_url);
      } else {
        setError(`Error fetching download URL: ${error_}`);
      }
    })()
  })

  return {
    downloadUrl,
    error
  }
}

export function useDownload() {
  return useCallback(
    (url: string, fileName: string) => {
      const a = document.createElement("a");
      a.href = url;
      a.download = fileName;
      a.style.display = "none";
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);

      URL.revokeObjectURL(url);
    }, []
  );
}

export function getFilesRedirectUrl(fileKey: string) {
  return `${process.env.NEXT_PUBLIC_API_BASE_URL}/files/download?key=${fileKey}&file_name=${fileKey}&redirect=true`
}