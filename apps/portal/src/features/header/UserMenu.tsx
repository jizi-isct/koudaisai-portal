import { LargePulldown } from '@koudaisai/shared-ui/LargePulldown';
import { logout } from '@koudaisai/shared-auth-members';
import { authFetchClient } from '../api/api';

const handleLogout = async () => {
  await logout(authFetchClient);
  window.location.assign('/login/');
};

export function UserMenu() {
  return (
    <LargePulldown
      type="secondary"
      align="right"
      items={[{ label: 'ログアウト', onClick: handleLogout, danger: true }]}
    >
      ユーザー
    </LargePulldown>
  );
}
