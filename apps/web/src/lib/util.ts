import {fetchClientNoAuth} from "@/lib/api";

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