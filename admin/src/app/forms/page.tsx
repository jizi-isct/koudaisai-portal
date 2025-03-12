"use client";
import styles from "./page.module.css";
import Lists from "@/components/Forms/Lists/Lists";
import {$api} from "@/lib/api";
import {components} from "@/lib/api_v1";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import Link from "next/link";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const {data, error} = $api.useQuery(
    "get",
    "/forms"
  )

  type Item = components["schemas"]["Item"]
  type Form = components["schemas"]["Form"]

  const renderLists = () => {
    if (!data) return null;

    return data.map((form:Form) => (
      <Lists
        key={form.form_id}
        title={form.info.title}
        status="未回答"
        dueDate="2022/10/10"
        summary={form.info.description}
        formId={form.form_id!}
      />
    ));
  };
  return (
    <div className={styles.page}>
      <main className={styles.main}>
        <Link href={"/admin/forms/new"}>新たなフォームを作成</Link>
        <div className={styles.formsWrapper}>
          {renderLists()}
        </div>
      </main>
    </div>
  );
}