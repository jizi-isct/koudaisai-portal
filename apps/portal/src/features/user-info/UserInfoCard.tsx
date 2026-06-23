import type { GroupRead, Role, UserRead } from '@koudaisai/shared-types';
import { getRepresentativeLabel } from '@koudaisai/shared-utils';
import styles from './UserInfoCard.module.css';

type Props = {
  user: UserRead;
  group: GroupRead;
  /** 現在のユーザーの団体内での役割(`/groups/{id}/members` 由来)。 */
  roles?: Role[];
};

export function UserInfoCard({ user, group, roles }: Props) {
  const representativeLabel =
    roles?.map((r) => getRepresentativeLabel(r, group.type)).join(', ') ??
    '役職なし';

  return (
    <div className={styles.user}>
      <h1>こんにちは、{user.name} 👋</h1>
      <h2>
        あなたは{group.name}
        <small>({group.id})</small>の{representativeLabel}です。
      </h2>
    </div>
  );
}
