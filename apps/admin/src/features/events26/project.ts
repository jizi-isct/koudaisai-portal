/**
 * 企画情報(events26)の一覧・編集で共有する型とヘルパー。
 *
 * CSV の読み書きは一覧側でしか使わないので [`Events26Page`] に残してある。
 */
import type { events26Components } from '@koudaisai/shared-types';
import { EVENTS26_API_URL } from 'astro:env/client';
import { api } from '@/features/api/api';

export type Project = events26Components['schemas']['Project'];
export type Occasion = events26Components['schemas']['Occasion'];
export type Place = NonNullable<Occasion['place']>;
export type Time = events26Components['schemas']['Time'];
export type FoodStallTag = events26Components['schemas']['FoodStallTag'];
export type GeneralTag = events26Components['schemas']['GeneralTag'];
export type Category = events26Components['schemas']['Category'];

export const PROJECT_TYPE_LABEL: Record<
  Project['type'],
  { text: string; color: string }
> = {
  'food-stall': { text: '模擬店企画', color: 'red' },
  general: { text: '一般企画', color: 'blue' },
  stage: { text: 'ステージ企画', color: 'green' },
  laboratory: { text: '研究室公開企画', color: 'orange' },
};

export const GENERAL_TAGS: GeneralTag[] = [
  'experience',
  'display',
  'performance',
  'food',
  'lecture',
];

/** events26 が企画カテゴリーとして受け付ける値。 */
export const CATEGORIES: Category[] = [
  'hearty',
  'street_food',
  'sweets',
  'performance',
  'play',
  'cafe',
  'laboratory',
  'display',
];

/** 空欄は未設定として扱い、それ以外は API に渡して検証する。 */
export function parseCategory(value: string | undefined): Category | undefined {
  const category = value?.trim() ?? '';
  if (category === '') return undefined;
  return category as Category;
}

export const CATEGORY_LABEL: Record<Category, string> = {
  hearty: 'ガッツリ',
  street_food: '食べ歩き',
  sweets: 'スイーツ',
  performance: 'パフォーマンス',
  play: '遊び',
  cafe: 'カフェ',
  laboratory: '研究室',
  display: '展示',
};

/** 模擬店タグの `tag` ごとに選べる `tag2`。`drink` は `tag2` を持たない。 */
export const FOOD_STALL_TAG2: Record<'main' | 'sweet', string[]> = {
  main: ['rice', 'noodle_flour', 'skewer_grill', 'snack', 'soup', 'world'],
  sweet: ['japanese', 'western', 'cold', 'snack', 'drink', 'world'],
};

export function formatTime(time: Time): string {
  return `${String(time.hour).padStart(2, '0')}:${String(time.minute).padStart(2, '0')}`;
}

/** `HH:MM` を `date` 日目の [`Time`] に変換する。 */
export function parseTime(date: Time['date'], value: string): Time {
  const match = /^(\d{1,2}):(\d{2})$/.exec(value.trim());
  if (!match) {
    throw new Error(`時刻は HH:MM 形式で指定してください: ${value}`);
  }
  const hour = Number(match[1]);
  const minute = Number(match[2]);
  if (hour > 23 || minute > 59) {
    throw new Error(`時刻の範囲が不正です: ${value}`);
  }
  return { date, hour, minute };
}

export function formatTags(project: Project): string {
  if (project.type === 'food-stall') {
    return project.tag
      .map((tag) => ('tag2' in tag ? `${tag.tag}:${tag.tag2}` : tag.tag))
      .join(';');
  }
  if (project.type === 'general') {
    return project.tag.join(';');
  }
  return '';
}

/** events26 がアイコンとして受け付ける形式。ここに無いファイルは送らずに飛ばす。 */
export const ICON_CONTENT_TYPES = [
  'image/png',
  'image/jpeg',
  'image/gif',
  'image/webp',
  'image/heic',
];

/** アイコン原本の公開 URL。events26 が認証なしで配信している。 */
export function iconUrl(projectId: string, cacheBuster: number): string {
  return `${EVENTS26_API_URL}/v1/projects/${encodeURIComponent(projectId)}/icon?v=${cacheBuster}`;
}

/**
 * 応答が失敗ならエラーにする。
 *
 * openapi-fetch は本文の無い応答(`204` や `Content-Length: 0`)を成功・失敗に
 * かかわらず `{ error: undefined }` で返す。backend の失敗応答(403 / 404 / 415 /
 * 422 / 500)はいずれも本文が無いので、`error` だけを見ると失敗を成功と取り違える。
 * ステータスで判定し、本文があれば内容を添える。
 */
export function ensureOk(
  result: { error?: unknown; response: Response },
  label: string,
): void {
  if (result.response.ok) return;
  const detail =
    result.error === undefined
      ? ''
      : `: ${typeof result.error === 'string' ? result.error : JSON.stringify(result.error)}`;
  throw new Error(`${label}に失敗しました(${result.response.status})${detail}`);
}

/**
 * アイコンを 1 件 PUT する。
 *
 * events26 の `/admin/v1` は Cloudflare Access 配下なので backend 中継を通す。
 * ボディは画像そのもので JSON ではないため、openapi-fetch の既定シリアライザを
 * 素通しに差し替え、形式判定に使う `Content-Type` を明示する。
 */
export async function putIcon(projectId: string, image: Blob): Promise<void> {
  const result = await api.PUT('/events26/projects/{project_id}/icon', {
    params: { path: { project_id: projectId } },
    body: image as unknown as string,
    bodySerializer: (body: unknown) => body as BodyInit,
    headers: { 'Content-Type': image.type },
  });
  ensureOk(result, 'アイコンのアップロード');
}
