import type { Time } from './project';

/**
 * 2 つの時刻を日、時、分の順に比較する。
 *
 * `a` が `b` より前なら負数、同じなら 0、後なら正数を返す。
 */
export function compareTime(a: Time, b: Time): number {
  return a.date - b.date || a.hour - b.hour || a.minute - b.minute;
}
