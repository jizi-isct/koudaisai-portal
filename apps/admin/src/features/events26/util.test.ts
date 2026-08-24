import { describe, expect, it } from 'vitest';
import { compareTime } from './util';

describe('compareTime', () => {
  it('日が早い時刻を前と判定する', () => {
    expect(
      compareTime(
        { date: 1, hour: 23, minute: 59 },
        { date: 2, hour: 0, minute: 0 },
      ),
    ).toBeLessThan(0);
  });

  it('同じ日では時が早い時刻を前と判定する', () => {
    expect(
      compareTime(
        { date: 1, hour: 9, minute: 59 },
        { date: 1, hour: 10, minute: 0 },
      ),
    ).toBeLessThan(0);
  });

  it('同じ日・時では分が遅い時刻を後と判定する', () => {
    expect(
      compareTime(
        { date: 2, hour: 10, minute: 30 },
        { date: 2, hour: 10, minute: 15 },
      ),
    ).toBeGreaterThan(0);
  });

  it('同じ時刻を同値と判定する', () => {
    expect(
      compareTime(
        { date: 2, hour: 10, minute: 30 },
        { date: 2, hour: 10, minute: 30 },
      ),
    ).toBe(0);
  });
});
