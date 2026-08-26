import Papa from 'papaparse';
import { CATEGORY_LABEL } from './project';
import type { Project } from './project';
import {
  compactTimeRanges,
  formatPlace,
  formatTime,
  type PlaceInfo,
} from './util';
import { EVENTS26_API_URL } from 'astro:env/client';

type ProjectRow = {
  id: string;
  category: string;
  group_name: string;
  project_name: string;
  description: string;
  is_child_friendly: string;
  is_recommended: string;
  day1_start_time: string;
  day1_end_time: string;
  day2_start_time: string;
  day2_end_time: string;
  places: string;
  is_lab_tour: boolean | string;
  offering: string | undefined;
  icon_url: string;
};

const CSV_COLUMNS: (keyof ProjectRow)[] = [
  'id',
  'category',
  'group_name',
  'project_name',
  'description',
  'is_child_friendly',
  'is_recommended',
  'day1_start_time',
  'day1_end_time',
  'day2_start_time',
  'day2_end_time',
  'places',
  'is_lab_tour',
  'offering',
  'icon_url',
];

function toRow(project: Project, placeInfos: readonly PlaceInfo[]): ProjectRow {
  const day1 = compactTimeRanges(
    project.occasions
      .filter((occasion) => occasion.timeRange.start.date === 1)
      .map((o) => o.timeRange),
  );
  const day2 = compactTimeRanges(
    project.occasions
      .filter((occasion) => occasion.timeRange.start.date === 2)
      .map((o) => o.timeRange),
  );
  const places = [
    ...project.occasions.reduce((acc, x) => {
      if (x.place) {
        const place = formatPlace(
          x.place,
          placeInfos,
          project.type === 'food-stall',
        );
        if (place) return acc.add(place);
      }
      return acc;
    }, new Set<string>()),
  ].join('・');

  return {
    id: project.id,
    category: project.category ? CATEGORY_LABEL[project.category] : 'なし',
    group_name: project.groupName,
    project_name: project.projectName,
    description: project.description,
    is_child_friendly: project.isChildFriendly ? 'true' : 'false',
    is_recommended: project.isRecommended ? 'true' : 'false',
    day1_start_time: formatTime(day1.start),
    day1_end_time: formatTime(day1.end),
    day2_start_time: formatTime(day2.start),
    day2_end_time: formatTime(day2.end),
    places: places,
    is_lab_tour: project.type === 'laboratory' ? project.isTour : '',
    offering: project.type === 'food-stall' ? project.offering : '',
    icon_url: new URL(
      `/v1/projects/${project.id}/icon`,
      EVENTS26_API_URL,
    ).toString(),
  };
}

/** 一覧確認用のダウンロード CSV を作る。編集 CSV とは別スキーマ。 */
export function createDownloadCsv(
  projects: Project[],
  placeInfos: readonly PlaceInfo[],
): string {
  return Papa.unparse(
    projects.map((project) => toRow(project, placeInfos)),
    {
      columns: CSV_COLUMNS,
      newline: '\r\n',
    },
  );
}
