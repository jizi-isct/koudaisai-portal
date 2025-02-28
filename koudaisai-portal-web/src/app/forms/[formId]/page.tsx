"use client";
import styles from "./page.module.css";
import { use } from "react";
import useSWR from "swr";

export default function Page({ params }: { params: Promise<{ formId: string }> }) {
  const { formId } = use(params);

  const fetcher = (url: string) => fetch(url).then((res) => res.json());
  const { data, error } = useSWR("http://localhost:4010/api/v1/forms", fetcher);

  if (error) return <p>データの取得に失敗しました</p>;
  if (!data) return <p>読み込み中...</p>;

  // form_id に一致するフォームを検索
  // const form = data.find((f: any) => f.form_id === formId);
  const form = data[0];

    return (
        <div className={styles.page}>
        <main className={styles.main}>
            <div className={styles.formTitleWrapper}>
                <h1>{form.info.title}</h1>
                <p>{form.info.description}</p>
            </div>
            <div className={styles.formWrapper}>
                
            </div>
        </main>
        </div>
    );
  }