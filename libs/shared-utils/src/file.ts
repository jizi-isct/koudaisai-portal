import type {ApiFetchClient} from "@koudaisai/shared-api";


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

export function getFilesRedirectUrl(fileKey: string) {
  return `${process.env.NEXT_PUBLIC_API_BASE_URL}/files/download?key=${fileKey}&file_name=${fileKey}&redirect=true`
}