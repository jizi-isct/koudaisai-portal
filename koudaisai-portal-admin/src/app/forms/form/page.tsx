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


  //useStateでデータ管理
  const [title, setTitle] = useState("");

  if (error) return <p>データの取得に失敗しました</p>;
  if (!data) return <p>読み込み中...</p>;
  
  // form_id に一致するフォームを検索
  // const form = data.find((f: any) => f.form_id === formId);
  const form = data[0];
  

  return (
    <div className={styles.page}>
    <main className={styles.main}>
        <div className={styles.formTitleWrapper}>
            <p>{form.info.description}</p>
        </div>
        <Question>
          <RadioButton />
        </Question>
    </main>
    </div>
  );
  }