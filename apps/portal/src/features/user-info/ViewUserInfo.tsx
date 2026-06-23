import type { GroupRead, Role, UserRead } from '@koudaisai/shared-types';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { useEffect, useState } from 'react';
import { api } from '@/features/api/api';
import { UserInfoCard } from './UserInfoCard';

export function ViewUserInfo() {
  const [user, setUser] = useState<UserRead | null>(null);
  const [group, setGroup] = useState<GroupRead | null>(null);
  const [roles, setRoles] = useState<Role[] | undefined>(undefined);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      // 現在のユーザーと所属団体は me / us エイリアスで取得する。
      const { data: user, error: userError } = await api.GET('/users/me');

      if (userError || !user) {
        setError(
          userError ? `${userError}` : 'ユーザー情報を取得できませんでした。',
        );
        setIsLoading(false);
        return;
      }

      const { data: group, error: groupError } = await api.GET('/groups/us');

      if (groupError || !group) {
        setError(
          groupError ? `${groupError}` : '団体情報を取得できませんでした。',
        );
        setIsLoading(false);
        return;
      }

      // 団体内での役割(第N責任者/代表者など)はメンバー一覧から自分を引く。
      const { data: members } = await api.GET('/groups/{id}/members', {
        params: { path: { id: group.id } },
      });
      setRoles(
        members?.filter((m) => m.user_id === user.id).map((m) => m.role),
      );

      setUser(user);
      setGroup(group);
      setIsLoading(false);
    })().catch((caughtError) => {
      setError(`${caughtError}`);
      setIsLoading(false);
    });
  }, []);

  if (isLoading) {
    return <LoadingScreen />;
  }

  if (error) {
    return <p>{error}</p>;
  }

  if (!user || !group) {
    return null;
  }

  return <UserInfoCard user={user} group={group} roles={roles} />;
}
