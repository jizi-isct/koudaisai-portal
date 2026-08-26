import Papa from 'papaparse';
import { parseCategory, parseTime } from './project';
import type { Occasion, PlaceId, Project, Time } from './project';

type EditProjectRow = {
  id: string;
  group_name?: string;
  project_name?: string;
  description?: string;
  is_child_friendly?: string;
  is_recommended?: string;
  day1_start_time?: string;
  day1_end_time?: string;
  day2_start_time?: string;
  day2_end_time?: string;
  place?: string;
  is_lab_tour?: string;
  offering?: string;
  category?: string;
};

const has = (row: EditProjectRow, column: keyof EditProjectRow) =>
  Object.hasOwn(row, column);

/** true / false は変換し、それ以外は API に渡して検証させる。 */
function parseBoolean(value: string | undefined): boolean {
  const normalized = value?.trim().toLowerCase();
  if (normalized === 'true') return true;
  if (normalized === 'false') return false;
  return value as unknown as boolean;
}

function editOccasions(row: EditProjectRow, project: Project): Occasion[] {
  const hasPlace = has(row, 'place');
  const days: {
    date: Time['date'];
    startColumn: 'day1_start_time' | 'day2_start_time';
    endColumn: 'day1_end_time' | 'day2_end_time';
  }[] = [
    { date: 1, startColumn: 'day1_start_time', endColumn: 'day1_end_time' },
    { date: 2, startColumn: 'day2_start_time', endColumn: 'day2_end_time' },
  ];

  if (
    !hasPlace &&
    days.every(
      ({ startColumn, endColumn }) =>
        !has(row, startColumn) && !has(row, endColumn),
    )
  ) {
    return project.occasions;
  }

  return days.flatMap(({ date, startColumn, endColumn }) => {
    const existing = project.occasions.find(
      (occasion) => occasion.timeRange.start.date === date,
    );
    const hasStart = has(row, startColumn);
    const hasEnd = has(row, endColumn);
    const startValue = row[startColumn]?.trim() ?? '';
    const endValue = row[endColumn]?.trim() ?? '';

    if (hasStart && hasEnd && startValue === '' && endValue === '') return [];
    if (!existing && !hasStart && !hasEnd) return [];

    const start = hasStart
      ? parseTime(date, startValue)
      : existing?.timeRange.start;
    const end = hasEnd ? parseTime(date, endValue) : existing?.timeRange.end;
    if (!start || !end) {
      throw new Error(
        `${startColumn} と ${endColumn} は、新しい開催予定では両方指定してください`,
      );
    }

    const place = hasPlace ? row.place?.trim() : existing?.place;
    return [
      {
        ...(place ? { place: place as PlaceId } : {}),
        timeRange: { start, end },
      },
    ];
  });
}

function editProject(row: EditProjectRow, project: Project): Project {
  const common = {
    ...project,
    ...(has(row, 'group_name') ? { groupName: row.group_name?.trim() } : {}),
    ...(has(row, 'project_name')
      ? { projectName: row.project_name?.trim() }
      : {}),
    ...(has(row, 'description')
      ? { description: row.description?.trim() }
      : {}),
    ...(has(row, 'is_child_friendly')
      ? { isChildFriendly: parseBoolean(row.is_child_friendly) }
      : {}),
    ...(has(row, 'is_recommended')
      ? { isRecommended: parseBoolean(row.is_recommended) }
      : {}),
    ...(has(row, 'category') ? { category: parseCategory(row.category) } : {}),
    occasions: editOccasions(row, project),
  };

  if (common.type === 'food-stall' && has(row, 'offering')) {
    return { ...common, offering: row.offering?.trim() || undefined };
  }
  if (common.type === 'laboratory' && has(row, 'is_lab_tour')) {
    return { ...common, isTour: parseBoolean(row.is_lab_tour) };
  }
  return common;
}

/** 存在する列だけを既存企画へ重ね、PUT 用の完全な `Project` 配列にする。 */
export function parseEditCsv(
  csv: string,
  existingProjects: Project[],
): Promise<Project[]> {
  const projectsById = new Map(
    existingProjects.map((project) => [project.id, project]),
  );

  return new Promise((resolve, reject) => {
    const projects: Project[] = [];
    Papa.parse<EditProjectRow>(csv, {
      header: true,
      skipEmptyLines: true,
      encoding: 'UTF-8',
      step: (result, parser) => {
        try {
          const id = result.data.id?.trim() ?? '';
          if (id === '') throw new Error('id は必須です');
          const project = projectsById.get(id);
          if (!project) throw new Error(`既存の企画が見つかりません: ${id}`);
          projects.push(editProject(result.data, project));
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
