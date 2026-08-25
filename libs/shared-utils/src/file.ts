import type { ApiFetchClient } from '@koudaisai/shared-api';

export async function getDownloadUrl(
  fetchClient: ApiFetchClient,
  fileKey: string,
  fileName: string,
) {
  return await fetchClient.GET('/files/download', {
    params: {
      query: {
        key: fileKey,
        file_name: fileName,
      },
    },
  });
}

export function getFilesRedirectUrl(apiBaseUrl: string, fileKey: string) {
  const url = new URL(`${apiBaseUrl.replace(/\/$/, '')}/files/download`);
  url.searchParams.set('key', fileKey);
  url.searchParams.set('file_name', fileKey);
  url.searchParams.set('redirect', 'true');
  return url.toString();
}
