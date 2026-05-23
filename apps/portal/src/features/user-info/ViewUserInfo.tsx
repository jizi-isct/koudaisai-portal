import type { GroupRead, UserRead } from '@koudaisai/shared-types';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { useEffect, useState } from 'react';
import { api } from '@/features/api/api';
import { UserInfoCard } from './UserInfoCard';

export function ViewUserInfo() {
  const [user, setUser] = useState<UserRead | null>(null);
  const [group, setGroup] = useState<GroupRead | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      const { data: user, error: userError } = await api.GET(
        '/users/{user_id}',
        {
          params: {
            path: {
              user_id: 'me',
            },
          },
        },
      );

      if (userError || !user) {
        setError(
          userError ? `${userError}` : 'ユーザー情報を取得できませんでした。',
        );
        setIsLoading(false);
        return;
      }

      const { data: group, error: groupError } = await api.GET('/groups/{id}', {
        params: {
          path: {
            id: user.group_id,
          },
        },
      });

      if (groupError || !group) {
        setError(
          groupError ? `${groupError}` : '団体情報を取得できませんでした。',
        );
        setIsLoading(false);
        return;
      }

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

  return <UserInfoCard user={user} group={group} />;
}
