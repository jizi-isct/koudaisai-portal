import type { GroupRead, Role, UserRead } from '@koudaisai/shared-types';
import { getRepresentativeIndex } from '@koudaisai/shared-utils';
import styles from './UserInfoCard.module.css';

type Props = {
  user: UserRead;
  group: GroupRead;
  /** 現在のユーザーの団体内での役割(`/groups/{id}/members` 由来)。 */
  role?: Role;
};

export function UserInfoCard({ user, group, role }: Props) {
  const representativeIndex = getRepresentativeIndex(role);
  const isPlanResponsible =
    group.type === 'booth_project' ||
    group.type === 'general_project' ||
    group.type === 'stage_project';

  return (
    <div className={styles.user}>
      <h1>こんにちは、{user.name} 👋</h1>
      {isPlanResponsible && (
        <h2>
          あなたは{group.name}
          <small>({group.id})</small>の第{representativeIndex}責任者です。
        </h2>
      )}
      {group.type === 'lab_project' && (
        <h2>
          あなたは{group.name}
          <small>({group.id})</small>の企画実施担当者です。
        </h2>
      )}
      {group.type === 'press' && (
        <h2>
          あなたは{group.name}
          <small>({group.id})</small>の代表者です。
        </h2>
      )}
    </div>
  );
}
