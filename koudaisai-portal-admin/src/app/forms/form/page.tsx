"use client";
import styles from "./page.module.css";
import useSWR from "swr";
import { useSearchParams } from "next/navigation";
import { useState, useEffect } from "react";
import TextInput from "@/components/Forms/TextInput/TextInput";
import ParagraphInput from "@/components/Forms/ParagraphInput/ParagraphInput";
import Question from "@/components/Forms/Question/Question";
import Text from "@/components/Forms/Question/Text/Text";
import CheckBox from "@/components/Forms/Question/CheckBox/CheckBox";
import RadioButton from "@/components/Forms/Question/RadioButton/RadioButton";

export default function Page() {
  const searchParams = useSearchParams();
  const formId = searchParams.get("formId");

  const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:4010";

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

  const [form, setForm] = useState<Form>();
  
  useEffect(() => {
    if (data && Array.isArray(data)) {
      // form_id が formId と一致するものを検索
      const foundForm = data.find((f: Form) => f.form_id === formId);
      if (foundForm) {
        //debug用の出力
        console.log(foundForm);
        setForm(foundForm);
      }
    }
  }, [data, formId]);

  const updateTitle = (title: string) => {
    setForm((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        info: {
          ...prev.info,
          ...{title},
        },
      };
    });
  };

  const updateDescription = (description: string) => {
    setForm((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        info: {
          ...prev.info,
          ...{description},
        },
      };
    });
  };

  const updateItem = (itemId: string, title: string, description: string) => {
    setForm((prevForm) => {
      if (!prevForm) return prevForm;
  
      return {
        ...prevForm,
        items: prevForm.items.map((item) =>
          item.item_id === itemId
            ? {
                ...item,
                title: title !== null ? title : item.title,
                description: description !== null ? description : item.description,
              }
            : item
        ),
      };
    });
  };

  const toggleRequired = (itemId: string) => {
    setForm((prevForm) => {
      if (!prevForm) return prevForm;
  
      return {
        ...prevForm,
        items: prevForm.items.map((item) =>
        item.item_id === itemId && item.item_question?.question
          ? {
              ...item,
              item_question: {
                ...item.item_question,
                question: {
                  ...item.item_question.question,
                  required: !item.item_question.question.required,
                },
              },
            }
          : item
        ),
      };
    });
  };

  const renderQuestions = () => {
    if (!form || !form.items) return null;
  
    return form.items.map((item) => (
      <Question key={item.item_id} itemId={item.item_id} form={form} updateItem={updateItem} toggleRequired={toggleRequired}>
        {/* itemの種類によって異なる入力コンポーネントを表示 */}
        {item.item_text && <Text />}
        {item.item_question?.question?.question_text?.paragraph ? <ParagraphInput fontSize={14} placeholder="長文回答" /> : <TextInput fontSize={14} placeholder="短文回答" />}
        {item.item_question && (
          <>
            <CheckBox />
            <RadioButton />
          </>
        )}
      </Question>
    ));
  };


  if (error) return <p>データの取得に失敗しました</p>;
  if (!data) return <p>読み込み中...</p>;
  
  // form_id に一致するフォームを検索
  // const form = data.find((f: any) => f.form_id === formId);
  return (
    <div className={styles.page}>
    <main className={styles.main}>
        <div className={styles.formTitleWrapper}>
          <TextInput
            fontSize={16}
            width={400}
            placeholder="タイトルを入力"
            value={form?.info?.title ?? "データなし"}
            onChange={updateTitle}
            args={[]}
          />
          <ParagraphInput
            fontSize={12}
            placeholder="説明文を入力"
            value={form?.info?.description ?? "データなし"}
            onChange={updateDescription}
            args={[]}
          />
        </div>
        {/* 動的に生成された Question コンポーネントを表示 */}
        {renderQuestions()}
    </main>
    </div>
  );
  }