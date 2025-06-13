"use client";

import {fetchClientNoAuth} from "@/lib/api";
import {useCallback, useEffect, useState} from "react";

export function chunk<T>(array: T[], size: number): T[][] {
  const result: T[][] = [];
  for (let i = 0; i < array.length; i += size) {
    result.push(array.slice(i, i + size));
  }
  return result;
}

export async function getDownloadUrl(fileKey: string, fileName: string) {
  return await fetchClientNoAuth.GET(
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