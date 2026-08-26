import Papa from 'papaparse';
import { parseCategory, parseTime } from './project';
import type { Occasion, PlaceId, Project, Time } from './project';

type CreateProjectRow = {
  id: string;
  group_name: string;
  project_name: string;
  description: string;
  is_child_friendly: string;
  is_recommended: string;
  day1_start_time: string;
  day1_end_time: string;
  day2_start_time: string;
  day2_end_time: string;
  place: string;
  is_lab_tour: string;
  offering: string;
  category: string;
};

const PROJECT_TYPE_BY_ID_PREFIX: Record<string, Project['type']> = {
  M: 'food-stall',
  S: 'stage',
  I: 'general',
  L: 'laboratory',
};

function projectTypeFromId(id: string): Project['type'] {
  const type = PROJECT_TYPE_BY_ID_PREFIX[id.charAt(0).toUpperCase()];
  if (!type) {
    throw new Error(
      `企画番号は M / S / I / L のいずれかで始まる必要があります: ${id}`,
    );
  }
  return type;
}

function requireField(value: string, column: string, id: string): string {
  const trimmed = value?.trim() ?? '';
  if (trimmed === '') {
    throw new Error(`${column} は必須です(${id || '企画番号不明'})`);
  }
  return trimmed;
}

function requireBoolean(value: string, column: string, id: string): boolean {
  const trimmed = requireField(value, column, id).toLowerCase();
  if (trimmed !== 'true' && trimmed !== 'false') {
    throw new Error(
      `${column} は true か false で指定してください(${id}): ${value}`,
    );
  }
  return trimmed === 'true';
}

function buildOccasions(row: CreateProjectRow): Occasion[] {
  const place = row.place?.trim() ?? '';
  const days: {
    date: Time['date'];
    start: string;
    end: string;
    startColumn: string;
    endColumn: string;
  }[] = [
    {
      date: 1,
      start: row.day1_start_time?.trim() ?? '',
      end: row.day1_end_time?.trim() ?? '',
      startColumn: 'day1_start_time',
      endColumn: 'day1_end_time',
    },
    {
      date: 2,
      start: row.day2_start_time?.trim() ?? '',
      end: row.day2_end_time?.trim() ?? '',
      startColumn: 'day2_start_time',
      endColumn: 'day2_end_time',
    },
  ];

  return days
    .filter((day) => {
      if (day.start === '' && day.end === '') return false;
      if (day.start === '' || day.end === '') {
        throw new Error(
          `${day.startColumn} と ${day.endColumn} は同時に指定してください(${row.id})`,
        );
      }
      return true;
    })
    .map((day) => ({
      ...(place === '' ? {} : { place: place as PlaceId }),
      timeRange: {
        start: parseTime(day.date, day.start),
        end: parseTime(day.date, day.end),
      },
    }));
}

function buildProject(row: CreateProjectRow): Project {
  const id = requireField(row.id, 'id', row.id);
  const category = parseCategory(row.category);
  const base = {
    id,
    groupName: requireField(row.group_name, 'group_name', id),
    projectName: requireField(row.project_name, 'project_name', id),
    description: requireField(row.description, 'description', id),
    isChildFriendly: requireBoolean(
      row.is_child_friendly,
      'is_child_friendly',
      id,
    ),
    isRecommended: requireBoolean(row.is_recommended, 'is_recommended', id),
    occasions: buildOccasions(row),
    ...(category ? { category } : {}),
  };

  switch (projectTypeFromId(id)) {
    case 'food-stall':
      return {
        ...base,
        type: 'food-stall',
        tag: [],
        ...(row.offering?.trim() ? { offering: row.offering.trim() } : {}),
      };
    case 'general':
      return { ...base, type: 'general', tag: [] };
    case 'stage':
      return { ...base, type: 'stage' };
    case 'laboratory':
      return {
        ...base,
        type: 'laboratory',
        isTour: requireBoolean(row.is_lab_tour, 'is_lab_tour', id),
      };
  }
}

/** 新規作成 CSV を `Project` の配列に変換する。 */
export function parseCreateCsv(csv: string): Promise<Project[]> {
  return new Promise((resolve, reject) => {
    const projects: Project[] = [];
    Papa.parse<CreateProjectRow>(csv, {
      header: true,
      skipEmptyLines: true,
      encoding: 'UTF-8',
      step: (result, parser) => {
        try {
          projects.push(buildProject(result.data));
        } catch (error) {
          parser.abort();
          reject(error instanceof Error ? error : new Error(String(error)));
        }
      },
      complete: () => resolve(projects),
      error: (error: Error) => reject(error),
    });
  });
}
