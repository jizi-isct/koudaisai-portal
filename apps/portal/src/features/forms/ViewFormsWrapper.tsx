import type { FormRead } from '@koudaisai/shared-types';
import { ViewFormCards } from '@koudaisai-portal/shared-ui-forms';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { useEffect, useState } from 'react';
import { api } from '@/features/api/api';

const PLACEHOLDER_RE = /\{[\w.]+\}/;
const URL_PLACEHOLDER_RE = /\{[\w.]+\}|%7[Bb][\w.]+%7[Dd]/;

function resolveFormUrl(url: string, values: Record<string, string>): string {
  try {
    const urlObj = new URL(url);
    for (const [paramKey, paramValue] of [...urlObj.searchParams.entries()]) {
      const substituted = Object.entries(values).reduce(
        (result, [key, value]) => result.replaceAll(`{${key}}`, value),
        paramValue,
      );
      if (PLACEHOLDER_RE.test(substituted)) {
        urlObj.searchParams.delete(paramKey);
      } else {
        urlObj.searchParams.set(paramKey, substituted);
      }
    }
    if (![...urlObj.searchParams.keys()].some((k) => k.startsWith('entry.'))) {
      urlObj.searchParams.delete('usp');
    }
    return urlObj.toString();
  } catch {
    return url;
  }
}

function resolveFormUrls(
  forms: FormRead[],
  values: Record<string, string>,
): FormRead[] {
  return forms.map((form) => {
    if (form.type !== 'external') return form;
    return { ...form, form_url: resolveFormUrl(form.form_url, values) };
  });
}

export function ViewFormsWrapper() {
  const [forms, setForms] = useState<FormRead[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      const formsResult = await api.GET('/forms');

      if (formsResult.error) {
        setError(`${formsResult.error}`);
        setForms([]);
        return;
      }

      const rawForms = formsResult.data ?? [];

      const needsSubstitution = rawForms.some(
        (form) =>
          form.type === 'external' && URL_PLACEHOLDER_RE.test(form.form_url),
      );

      if (!needsSubstitution) {
        setForms(rawForms);
        return;
      }

      const [userResult, groupResult] = await Promise.all([
        api.GET('/users/me'),
        api.GET('/groups/us'),
      ]);

      if (userResult.data && groupResult.data) {
        const user = userResult.data;
        const group = groupResult.data;
        setForms(
          resolveFormUrls(rawForms, {
            'user.name': user.name,
            'user.email': user.m_address,
            'user.id': user.id,
            'group.id': group.id,
            'group.name': group.name,
          }),
        );
      } else {
        setForms(resolveFormUrls(rawForms, {}));
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
