import type { FormRead, GroupRead, UserRead } from '@koudaisai/shared-types';
import { ViewFormCards } from '@koudaisai-portal/shared-ui-forms';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { useEffect, useState } from 'react';
import { api } from '@/features/api/api';

function resolveFormUrl(url: string, user: UserRead, group: GroupRead): string {
  const values: Record<string, string> = {
    'user.name': user.name,
    'user.email': user.m_address,
    'user.id': user.id,
    'group.id': group.id,
    'group.name': group.name,
  };

  return Object.entries(values).reduce((result, [key, value]) => {
    const encoded = encodeURIComponent(value);
    // アドレスバーからコピーした場合に波括弧が %7B/%7D になることへの対応
    return result
      .replaceAll(`{${key}}`, encoded)
      .replaceAll(`%7B${key}%7D`, encoded);
  }, url);
}

function resolveFormUrls(
  forms: FormRead[],
  user: UserRead,
  group: GroupRead,
): FormRead[] {
  return forms.map((form) => {
    if (form.type !== 'external') return form;
    return { ...form, form_url: resolveFormUrl(form.form_url, user, group) };
  });
}

export function ViewFormsWrapper() {
  const [forms, setForms] = useState<FormRead[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      const [formsResult, userResult, groupResult] = await Promise.all([
        api.GET('/forms'),
        api.GET('/users/me'),
        api.GET('/groups/us'),
      ]);

      if (formsResult.error) {
        setError(`${formsResult.error}`);
        setForms([]);
        return;
      }

      const rawForms = formsResult.data ?? [];

      if (userResult.data && groupResult.data) {
        setForms(resolveFormUrls(rawForms, userResult.data, groupResult.data));
      } else {
        setForms(rawForms);
      }
    })().catch((caughtError) => {
      setError(`${caughtError}`);
      setForms([]);
    });
  }, []);

  if (!forms) {
    return <LoadingScreen />;
  }

  if (error) {
    return <p>フォームの取得に失敗しました: {error}</p>;
  }

  return <ViewFormCards forms={forms} />;
}
