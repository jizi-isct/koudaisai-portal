import type { apiComponents } from '@koudaisai/shared-types';
import { EVENTS26_API_URL } from 'astro:env/client';

export type Project = apiComponents['schemas']['Project'];

/**
 * 企画情報API(events26)の公開エンドポイントから企画を取得する。
 *
 * api_v3 が中継しているのは admin 向けの登録・更新・削除だけで参照系が無いため、
 * 型だけ backend 経由(`apiComponents`)で受け取り、取得は公開 API を直接叩く。
 *
 * @returns 企画が未登録の場合は null
 */
export async function getProject(projectId: string): Promise<Project | null> {
  const response = await fetch(
    `${EVENTS26_API_URL}/v1/projects/${encodeURIComponent(projectId)}`,
  );

  if (response.status === 404) {
    return null;
  }

  if (!response.ok) {
    throw new Error(`企画情報の取得に失敗しました。(${response.status})`);
  }

  return (await response.json()) as Project;
}

/**
 * 企画実施場所の表示名を取得する。認証は不要。
 *
 * `Place` は api_v3 の components に載っていない(backend が中継しているのは
 * 企画のみ)ため、表示に使う displayName だけ型を書く。
 *
 * @returns 取得できなかった場合は placeId をそのまま返す
 */
export async function getPlaceLabel(placeId: string): Promise<string> {
  const response = await fetch(
    `${EVENTS26_API_URL}/v1/places/${encodeURIComponent(placeId)}`,
  );

  if (!response.ok) {
    return placeId;
  }

  const place = (await response.json()) as { displayName: string };
  return place.displayName;
}

/**
 * 企画アイコン(原本画像)の URL を組み立てる。認証は不要。
 *
 * `GET /v1/projects/{projectId}/icon` は画像バイナリを直接返し、未登録なら 404 になる。
 * `<img>` から読むだけなので fetch は挟まず URL だけを返す。
 */
export function getProjectIconUrl(projectId: string): string {
  return `${EVENTS26_API_URL}/v1/projects/${encodeURIComponent(projectId)}/icon`;
}
