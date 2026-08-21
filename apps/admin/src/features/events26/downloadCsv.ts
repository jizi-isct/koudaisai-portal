import Papa from 'papaparse';
import { formatTags, formatTime } from './project';
import type { Project } from './project';

type ProjectRow = {
  id: string;
  type: string;
  group_name: string;
  project_name: string;
  description: string;
  is_child_friendly: string;
  is_recommended: string;
  is_tour: string;
  offering: string;
  category: string;
  tags: string;
  day1_place: string;
  day1_start: string;
  day1_end: string;
  day2_place: string;
  day2_start: string;
  day2_end: string;
};

const CSV_COLUMNS: (keyof ProjectRow)[] = [
  'id',
  'type',
  'group_name',
  'project_name',
  'description',
  'is_child_friendly',
  'is_recommended',
  'is_tour',
  'offering',
  'category',
  'tags',
  'day1_place',
  'day1_start',
  'day1_end',
  'day2_place',
  'day2_start',
  'day2_end',
];

function toRow(project: Project): ProjectRow {
  const day1 = project.occasions.find(
    (occasion) => occasion.timeRange.start.date === 1,
  );
  const day2 = project.occasions.find(
    (occasion) => occasion.timeRange.start.date === 2,
  );

  return {
    id: project.id,
    type: project.type,
    group_name: project.groupName,
    project_name: project.projectName,
    description: project.description,
    is_child_friendly: project.isChildFriendly ? 'true' : 'false',
    is_recommended: project.isRecommended ? 'true' : 'false',
    is_tour:
      project.type === 'laboratory' ? (project.isTour ? 'true' : 'false') : '',
    offering: project.type === 'food-stall' ? (project.offering ?? '') : '',
    category: project.category ?? '',
    tags: formatTags(project),
    day1_place: day1?.place ?? '',
    day1_start: day1 ? formatTime(day1.timeRange.start) : '',
    day1_end: day1 ? formatTime(day1.timeRange.end) : '',
    day2_place: day2?.place ?? '',
    day2_start: day2 ? formatTime(day2.timeRange.start) : '',
    day2_end: day2 ? formatTime(day2.timeRange.end) : '',
  };
}

/** 一覧確認用のダウンロード CSV を作る。編集 CSV とは別スキーマ。 */
export function createDownloadCsv(projects: Project[]): string {
  return Papa.unparse(projects.map(toRow), {
    columns: CSV_COLUMNS,
    newline: '\r\n',
  });
}
