import {LargePulldown} from "@koudaisai/shared-ui/LargePulldown";
import {logout} from "@koudaisai/shared-auth-members";

const handleLogout = () => {
  logout();
  window.location.assign("/login/");
};

export function UserMenu() {
  return (
    <LargePulldown
      type="secondary"
      align="right"
      items={[
        {label: "ログアウト", onClick: handleLogout, danger: true},
      ]}
    >
      ユーザー
    </LargePulldown>
  );
}
