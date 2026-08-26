import Papa from 'papaparse';
import { describe, expect, it, vi } from 'vitest';
import type { Project } from './project';
import { createDownloadCsv } from './downloadCsv';

vi.mock('astro:env/client', () => ({
  EVENTS26_API_URL: 'https://events26.example.com',
}));
vi.mock('@/features/api/api', () => ({ api: {} }));

describe('createDownloadCsv', () => {
  it('food-stallだけ場所の1階層目から表示名を結合する', () => {
    const occasion = {
      place: 'south.fs-south-east.1',
      timeRange: {
        start: { date: 1, hour: 10, minute: 0 },
        end: { date: 1, hour: 11, minute: 0 },
      },
    } as const;
    const common = {
      groupName: '団体',
      projectName: '企画',
      description: '概要',
      isChildFriendly: false,
      isRecommended: false,
      occasions: [occasion],
    };
    const projects = [
      { ...common, id: 'F-001', type: 'food-stall', tag: [] },
      { ...common, id: 'I-001', type: 'general', tag: [] },
    ] satisfies Project[];
    const places = [
      { id: 'south', displayName: '大岡山南地区' },
      { id: 'south.fs-south-east', displayName: '南地区東側模擬店エリア' },
      { id: 'south.fs-south-east.1', displayName: '1番' },
    ];

    const { data } = Papa.parse<{ id: string; places: string }>(
      createDownloadCsv(projects, places),
      { header: true },
    );

    expect(data.find(({ id }) => id === 'F-001')?.places).toBe(
      '大岡山南地区 南地区東側模擬店エリア 1番',
    );
    expect(data.find(({ id }) => id === 'I-001')?.places).toBe(
      '南地区東側模擬店エリア 1番',
    );
  });
});
