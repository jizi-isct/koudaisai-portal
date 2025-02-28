"use client";
import Image from "next/image";
import styles from "./page.module.css";
import useSWR from "swr";
import Forms from "@/components/Forms/Lists/Lists";

export default function Page() {

  const fetcher = (url: string) => fetch(url).then((res) => res.json());
  const { data, error } = useSWR("http://localhost:4010/api/v1/forms", fetcher);

  if (error) return <p>データの取得に失敗しました</p>;
  if (!data) return <p>読み込み中...</p>;

  return (
    <div className={styles.page}>
      <main className={styles.main}>
        <div className={styles.formsWrapper}>
          {data.map((form: any) => (
            <Forms
              formId={form.form_id}
              title={form.info.title}
              status={"未回答"}
              dueDate={"5/11"}
              summary={form.info.description}
              key={form.form_id}
            />
          ))}
        </div>
      </main>
    </div>
  );
}
