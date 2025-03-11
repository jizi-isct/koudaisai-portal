"use client";
import styles from "./page.module.css";
import useSWR, {mutate} from "swr";
import {useSearchParams} from "next/navigation";
import {useEffect, useState} from "react";
import TextInput from "@/components/Forms/TextInput/TextInput";
import ParagraphInput from "@/components/Forms/ParagraphInput/ParagraphInput";
import Question from "@/components/Forms/Question/Question";
import SaveStatus from "@/components/Forms/SaveStatus/SaveStatus";

export default function Page() {
  const searchParams = useSearchParams();
  const formId = searchParams.get("formId");

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

  const [form, setForm] = useState<Form>();
  const [saveStatus, setSaveStatus] = useState<"saving" | "saved" | "unsaved">("saved");

  // フォームのデータをサーバーへ保存
  const saveForm = async (updatedForm: Form) => {
    if (!formId) return;
    setSaveStatus("saving");

    try {
      await fetch(`${API_BASE_URL}/api/v1/forms/${formId}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(updatedForm),
      });

      setSaveStatus("saved");
      mutate(`${API_BASE_URL}/api/v1/forms`); // キャッシュを更新して再フェッチ
    } catch (error) {
      console.error("保存に失敗しました", error);
      setSaveStatus("unsaved");
    }
  };
  
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
      const updatedForm = {
        ...prev,
        info: { ...prev.info, title },
      };
      saveForm(updatedForm); // 変更をサーバーに保存
      return updatedForm;
    });
  };

  const updateDescription = (description: string) => {
    setForm((prev) => {
      if (!prev) return prev;
      const updatedForm = {
        ...prev,
        info: { ...prev.info, description },
      };
      saveForm(updatedForm); // 変更をサーバーに保存
      return updatedForm;
    });
  };

  const updateItem = (itemId: string, title: string, description: string) => {
    setForm((prevForm) => {
      if (!prevForm) return prevForm;
  
      const updatedForm = {
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
      saveForm(updatedForm); // 変更をサーバーに保存
      return updatedForm;
    });
  };

  const toggleRequired = (itemId: string) => {
    setForm((prevForm) => {
      if (!prevForm) return prevForm;
  
      const updatedForm = {
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
      saveForm(updatedForm); // 変更をサーバーに保存
      return updatedForm;
    });
  };

  const reorderQuestionUp = (itemId: string) => {
    setForm((prevForm) => {
      if (!prevForm) return prevForm;
  
      const index = prevForm.items.findIndex((item) => item.item_id === itemId);
      if (index === 0) return prevForm;
  
      const updatedForm = {
        ...prevForm,
        items: [
          ...prevForm.items.slice(0, index - 1),
          prevForm.items[index],
          prevForm.items[index - 1],
          ...prevForm.items.slice(index + 1),
        ],
      };
      saveForm(updatedForm); // 変更をサーバーに保存
      return updatedForm;
    });
  };

  const reorderQuestionDown = (itemId: string) => {
    setForm((prevForm) => {
      if (!prevForm) return prevForm;
  
      const index = prevForm.items.findIndex((item) => item.item_id === itemId);
      if (index === -1) return prevForm;
  
      const updatedForm = {
        ...prevForm,
        items: [
          ...prevForm.items.slice(0, index),
          prevForm.items[index + 1],
          prevForm.items[index],
          ...prevForm.items.slice(index + 2),
        ],
      };
      saveForm(updatedForm); // 変更をサーバーに保存
      return updatedForm;
    });
  };

  const deleteQestion = (itemId: string) => {
    setForm((prevForm) => {
      if (!prevForm) return prevForm;
  
      const updatedForm = {
        ...prevForm,
        items: prevForm.items.filter((item) => item.item_id !== itemId),
      };
      saveForm(updatedForm); // 変更をサーバーに保存
      return updatedForm;
    });
  };

  const createNewItem = (itemType: string) => {
    setForm((prevForm) => {
      if (!prevForm) return prevForm;
      
      const newItem: Item = {
        item_id: crypto.randomUUID(),
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        title: "新しい質問",
        description: "",
        ...(itemType === "text" && { item_text: {} }),
        ...(itemType === "page_break" && { item_page_break: {} }),
        ...(itemType === "question" && {
          item_question: {
            question: {
              question_id: crypto.randomUUID(),
              created_at: new Date().toISOString(),
              updated_at: new Date().toISOString(),
              required: false,
              question_text: {
                paragraph: false,
              },
            },
          },
        }),
      };
      
      const updatedForm = {
        ...prevForm,
        items: [...prevForm.items, newItem],
      };
      
      saveForm(updatedForm); // 変更をサーバーに保存
      return updatedForm;
    });
  };

  const renderItems = () => {
    if (!form || !form.items) return null;
  
    return form.items.map((item) => (
      <Question key={item.item_id} itemId={item.item_id} form={form} updateItem={updateItem} toggleRequired={toggleRequired} reorderQuestionUp={reorderQuestionUp} reorderQuestionDown={reorderQuestionDown} deleteQuestion={deleteQestion}>
        {/* itemの種類によって異なる入力コンポーネントを表示 */}
        {item.item_question?.question?.question_text?.paragraph ? <ParagraphInput fontSize={14} placeholder="長文回答" /> : <TextInput fontSize={14} placeholder="短文回答" />}
        {item.item_question && (
          <>
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
          <div className={styles.formMenuWrapper}>
            <SaveStatus status={saveStatus} />
          </div>
        </div>
        {/* 動的に生成された Question コンポーネントを表示 */}
        {renderItems()}
        <div className={styles.newItemWrapper}>
          <button onClick={() => createNewItem("text")}>短文回答</button>
          <button onClick={() => createNewItem("question")}>質問</button>
          <button onClick={() => createNewItem("page_break")}>ページ区切り</button>
        </div>
    </main>
    </div>
  );
  }