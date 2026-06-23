import type { GroupType, Role } from '@koudaisai/shared-types';

export function getRepresentativeLabel(
  role: Role,
  groupType: GroupType,
): string {
  switch (role) {
    case 'first_responsible':
      return '第一責任者';
    case 'second_responsible':
      return '第二責任者';
    case 'third_responsible':
      return '第三責任者';
    case 'representative':
      if (groupType === 'lab_project') {
        return '企画責任者';
      }
      return '代表者';
    case 'operator':
      return '企画実施担当者';
  }
}
