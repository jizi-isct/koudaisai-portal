import { describe, expect, it } from 'vitest';
import {
  compactTimeRanges,
  compareTime,
  enrichPlaceFloors,
  formatPlace,
  formatTime,
} from './util';

describe('enrichPlaceFloors', () => {
  it('企画が使用するroomだけfloorを個別取得して補完する', async () => {
    const places = [
      { id: 'south.s2', type: 'building', displayName: '南2号館' },
      { id: 'south.s2.s2-203', type: 'room', displayName: 'S2-203' },
      { id: 'south.s2.s2-204', type: 'room', displayName: 'S2-204' },
    ];
    const requested: string[] = [];

    const result = await enrichPlaceFloors(
      [
        {
          occasions: [
            { place: 'south.s2.s2-203' },
            { place: 'south.s2.s2-203' },
          ],
        },
      ],
      places,
      async (placeId) => {
        requested.push(placeId);
        return '2F';
      },
    );

    expect(requested).toEqual(['south.s2.s2-203']);
    expect(formatPlace('south.s2.s2-203', result)).toBe('南2号館 2F S2-203');
    expect(result.find(({ id }) => id === 'south.s2.s2-204')?.floor).toBe(
      undefined,
    );
  });
});

describe('formatPlace', () => {
  const places = [
    { id: 'main', displayName: '大岡山キャンパス' },
    { id: 'main.m', displayName: '本館' },
    { id: 'main.m.m-101', displayName: 'M-101', floor: '1階' },
  ];

  it('2階層目から末端までの表示名を順に結合する', () => {
    expect(formatPlace('main.m.m-101', places)).toBe('本館 1階 M-101');
  });

  it('指定された場合は1階層目から表示名を結合する', () => {
    expect(formatPlace('main.m.m-101', places, true)).toBe(
      '大岡山キャンパス 本館 1階 M-101',
    );
  });

  it('floorがなければ表示名だけを結合する', () => {
    expect(
      formatPlace('east.taki-plaza.tp-b1-event', [
        { id: 'east', displayName: '大岡山東地区' },
        { id: 'east.taki-plaza', displayName: 'Taki Plaza' },
        {
          id: 'east.taki-plaza.tp-b1-event',
          displayName: '地下1階イベントスペース',
        },
      ]),
    ).toBe('Taki Plaza 地下1階イベントスペース');
  });
});

describe('formatTime', () => {
  it.each([
    [{ date: 1, hour: 0, minute: 5 } as const, '00:05'],
    [{ date: 1, hour: 9, minute: 7 } as const, '09:07'],
    [{ date: 2, hour: 23, minute: 59 } as const, '23:59'],
  ])('%o を %s にフォーマットする', (time, expected) => {
    expect(formatTime(time)).toBe(expected);
  });
});

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

describe('compactTimeRanges', () => {
  it('単一の時間範囲をそのまま返す', () => {
    expect(
      compactTimeRanges([
        {
          start: { date: 1, hour: 10, minute: 30 },
          end: { date: 1, hour: 11, minute: 45 },
        },
      ]),
    ).toEqual({
      start: { date: 1, hour: 10, minute: 30 },
      end: { date: 1, hour: 11, minute: 45 },
    });
  });

  it('順不同の時間範囲を最も早い開始時刻と最も遅い終了時刻にまとめる', () => {
    expect(
      compactTimeRanges([
        {
          start: { date: 2, hour: 9, minute: 30 },
          end: { date: 2, hour: 11, minute: 0 },
        },
        {
          start: { date: 1, hour: 13, minute: 15 },
          end: { date: 1, hour: 15, minute: 30 },
        },
        {
          start: { date: 2, hour: 8, minute: 45 },
          end: { date: 2, hour: 10, minute: 0 },
        },
      ]),
    ).toEqual({
      start: { date: 1, hour: 13, minute: 15 },
      end: { date: 2, hour: 11, minute: 0 },
    });
  });
});
