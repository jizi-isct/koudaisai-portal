import type {FormRead} from "@koudaisai/shared-types";
import {ViewFormCards} from "@koudaisai-portal/shared-ui-forms";
import {LoadingScreen} from "@koudaisai/shared-ui";
import {useEffect, useState} from "react";
import {api} from "@/features/api/api";

export function ViewFormsWrapper() {
  const [forms, setForms] = useState<FormRead[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      const {data, error} = await api.GET("/forms");

      if (error) {
        setError(`${error}`);
        setForms([]);
        return;
      }

      setForms(data ?? []);
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
