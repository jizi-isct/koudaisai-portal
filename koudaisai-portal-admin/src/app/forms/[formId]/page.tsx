"use client";

import Image from "next/image";
import styles from "./page.module.css";
import useSWR from "swr";
import { use } from "react";
import TextBox from "@/components/Forms/TextBox/TextBox";
import Question from "@/components/Forms/Question/Question";
import Text from "@/components/Forms/Question/Text/Text";
import CheckBox from "@/components/Forms/Question/CheckBox/CheckBox";
import RadioButton from "@/components/Forms/Question/RadioButton/RadioButton";

export default function Page({ params }: { params: Promise<{ formId: string }> }) {
  const { formId } = use(params);
    const fetcher = (url: string) => fetch(url).then((res) => res.json());
    const { data, error } = useSWR("http://localhost:4010/api/v1/forms", fetcher);
    if (error) return <p>データの取得に失敗しました</p>;
    if (!data) return <p>読み込み中...</p>;
    return (
        <div className={styles.page}>
        <main className={styles.main}>
            <div className={styles.formTitleWrapper}>
                <h1>{data[0].info.title}</h1>
                <p>{data[0].info.description}</p>
            </div>
            <Question>
              <RadioButton />
            </Question>
        </main>
        </div>
    );
  }