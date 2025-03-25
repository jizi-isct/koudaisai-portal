"use client";
import styles from "./page.module.css";
import Forms from "@/components/Forms/Lists/Lists";
import {$api} from "@/lib/api";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";

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
                            dueDate={form.info.deadline}
                            summary={form.info.description}
                            key={form.form_id}
                        />
                    ))}
                </div>
            </main>
        </div>
    );
}