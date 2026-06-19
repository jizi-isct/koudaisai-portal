/**
 * メンバーの役割(Role)を「第N責任者」の N(一/二/三)に対応付ける。
 *
 * api_v3 では代表者情報は Group ではなくメンバーシップの role
 * (`/groups/{id}/members` の `MemberRead.role`)に移った。第一/二/三責任者のみ
 * インデックスを持ち、代表者(representative)・運営担当(operator)等は undefined を返す。
 */
import type { Role } from '@koudaisai/shared-types';

export function getRepresentativeIndex(
  role: Role | undefined,
): string | undefined {
  switch (role) {
    case 'first_responsible':
      return '一';
    case 'second_responsible':
      return '二';
    case 'third_responsible':
      return '三';
    default:
      return undefined;
  }
}
