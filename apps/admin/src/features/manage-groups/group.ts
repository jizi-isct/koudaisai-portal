import type { GroupRead, Role } from '@koudaisai/shared-types';

/**
 * 参加団体の種別・団体ID・役割に関する規則をまとめたモジュール。
 */

export const groupTypeLabels: Record<GroupRead['type'], string> = {
  booth_project: '模擬店',
  stage_project: 'ステージ',
  general_project: '一般',
  lab_project: '研究室',
  press: '学内取材',
};

const groupIdPrefixByType: Record<GroupRead['type'], string> = {
  booth_project: 'M',
  stage_project: 'S',
  general_project: 'I',
  lab_project: 'L',
  press: 'P',
};

const typeByGroupIdPrefix: Record<string, GroupRead['type']> =
  Object.fromEntries(
    Object.entries(groupIdPrefixByType).map(([type, prefix]) => [
      prefix,
      type as GroupRead['type'],
    ]),
  );

export const groupIdPattern = new RegExp(
  `^([${Object.values(groupIdPrefixByType).join('')}])-(\\d{1,3})$`,
);

const parseGroupId = (idValue: string | undefined) =>
  groupIdPattern.exec(idValue?.toUpperCase() ?? '');

export const inferTypeFromGroupId = (
  idValue: string | undefined,
): GroupRead['type'] | undefined => {
  const match = parseGroupId(idValue);
  return match ? typeByGroupIdPrefix[match[1]] : undefined;
};

export const canonicalizeGroupId = (idValue: string) => {
  const match = parseGroupId(idValue);
  return match ? `${match[1]}-${match[2].padStart(3, '0')}` : idValue;
};

const responsibleRoles: Role[] = [
  'first_responsible',
  'second_responsible',
  'third_responsible',
];

export const rolesByGroupType: Record<GroupRead['type'], Role[]> = {
  booth_project: responsibleRoles,
  stage_project: responsibleRoles,
  general_project: responsibleRoles,
  lab_project: ['representative', 'operator'],
  press: ['representative'],
};

export const requiresDistinctMembers = (groupType: GroupRead['type']) =>
  rolesByGroupType[groupType] === responsibleRoles;
