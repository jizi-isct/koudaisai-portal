"use client";
import styles from "./page.module.css";
import useSWR from "swr";
import { useSearchParams } from "next/navigation";
import { useState, useEffect } from "react";
import TextBox from "@/components/Forms/TextBox/TextBox";
import Question from "@/components/Forms/Question/Question";
import Text from "@/components/Forms/Question/Text/Text";
import CheckBox from "@/components/Forms/Question/CheckBox/CheckBox";
import RadioButton from "@/components/Forms/Question/RadioButton/RadioButton";

export default function Page() {
  const searchParams = useSearchParams();
  const formId = searchParams.get("formId");
  
  const fetcher = (url: string) => fetch(url).then((res) => res.json());
  const { data, error } = useSWR("http://localhost:4010/api/v1/forms", fetcher);

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
  

  const [item, setItem] = useState<Item[]>([]);
  const [form, setForm] = useState<Form>();
  
  useEffect(() => {
    if (data && Array.isArray(data) && data.length > 0) {
      console.log(data[0]);
      setForm(data[0] as Form);
    }
  }, [data]);

  if (error) return <p>データの取得に失敗しました</p>;
  if (!data) return <p>読み込み中...</p>;
  
  // form_id に一致するフォームを検索
  // const form = data.find((f: any) => f.form_id === formId);

  return (
    <div className={styles.page}>
    <main className={styles.main}>
        <div className={styles.formTitleWrapper}>
          <h1>{form?.info?.title ?? "データなし"}</h1>
          <p>{form?.form_id ?? "データなし"}</p>
        </div>
        <Question>
          <RadioButton />
        </Question>
    </main>
    </div>
  );
  }