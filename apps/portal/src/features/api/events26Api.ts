import { EVENTS26_API_URL } from 'astro:env/client';

/**
 * 企画アイコン(原本画像)の URL を組み立てる。認証は不要。
 *
 * `GET /v1/projects/{projectId}/icon` は画像バイナリを直接返し、未登録なら 404 になる。
 * `<img>` から読むだけなので fetch は挟まず URL だけを返す。
 */
export function getProjectIconUrl(projectId: string): string {
  return `${EVENTS26_API_URL}/v1/projects/${encodeURIComponent(projectId)}/icon`;
}
