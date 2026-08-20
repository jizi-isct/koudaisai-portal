import { parseCreateCsv } from './createCsv';
import type { Project } from './project';

/**
 * 編集 CSV を PUT 用の完全な `Project` 配列に変換する。
 * 新規作成 CSV と同じ列を使い、アイコン URL の列は持たない。
 */
export function parseEditCsv(csv: string): Promise<Project[]> {
  return parseCreateCsv(csv);
}
