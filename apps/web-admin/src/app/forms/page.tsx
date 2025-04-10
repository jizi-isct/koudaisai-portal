"use client";
import styles from "./page.module.css";
import Lists from "../../components/Forms/Lists/Lists";
import {$apiAdmin, Form} from "@koudaisai-portal/util";
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
    const {data} = $apiAdmin.useQuery(
        "get",
        "/forms"
    )

    const renderLists = () => {
        if (!data) return null;

        return data.map((form: Form) => (
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
                <Link href={"/forms/new"}>新たなフォームを作成</Link>
                <div className={styles.formsWrapper}>
                    {renderLists()}
                </div>
            </main>
        </div>
    );
}