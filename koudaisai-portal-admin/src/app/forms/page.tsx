"use client";
import Image from "next/image";
import styles from "./page.module.css";
import useSWR from "swr";
import Lists from "@/components/Forms/Lists/Lists";

export default function Page() {
  const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL;

  const fetcher = (url: string) => fetch(url).then((res) => res.json());
  const { data, error } = useSWR(`${API_BASE_URL}/api/v1/forms`, fetcher);

  type Item = {
    item_id: string;
    created_at: string;
    updated_at: string;
    title: string;
    description?: string;
    item_page_break?: object; // ページ区切りアイテム
    item_text?: object;       // テキストアイテム
    item_question?: {
      question: {
        question_id: string;
        created_at: string;
        updated_at: string;
        required: boolean;
        question_text: {
          paragraph: boolean;
        };
      };
    };
  };

  type Form = {
    form_id: string;
    created_at: string;
    updated_at: string;
    info: {
      title: string;
      document_title: string;
      description: string;
    };
    description?: string;
    items: Item[];
    access_control?: {
      AccessControl: {
        roles: string[];
      };
    };
  };

  const renderLists = () => {
    if (!data) return null;
  
    return data.map((form:Form) => (
      <Lists
        key={form.form_id}
        title={form.info.title}
        status="未回答"
        dueDate="2022/10/10"
        summary={form.info.description}
        formId={form.form_id}
      />
    ));
  };
  return (
    <div className={styles.page}>
      <main className={styles.main}>
        <div className={styles.formsWrapper}>
          {renderLists()}
        </div>
      </main>
    </div>
  );
}
