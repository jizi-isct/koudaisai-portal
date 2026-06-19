import { getDownloadUrl } from './file';
import { DocumentRead } from '@koudaisai/shared-types';
import { ApiFetchClient } from '@koudaisai/shared-api';

export async function downloadDocument(
  document: DocumentRead,
  fetchClient: ApiFetchClient,
  download: (url: string, fileName: string) => void,
) {
  // format は document に平坦化された判別子(pdf/misc は file_key/file_name、
  // markdown は content を持つ)。
  if (document.format === 'pdf' || document.format === 'misc') {
    const { data: downloadUrl } = await getDownloadUrl(
      fetchClient,
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
}
