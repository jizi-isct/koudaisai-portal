"use client";
import styles from "./page.module.css";
import { useSearchParams } from "next/navigation";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import useSWR from "swr";
import Question from "@/components/Forms/Questions/Question";

export default function Page() {
  const searchParams = useSearchParams();
  const formId = searchParams.get("formId");

  const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL;
  const router = useRouter();

  const [authenticated, setAuthenticated] = useState(false);
  const [answers, setAnswers] = useState({}); // 回答を管理する state

  useEffect(() => {
    const access_token = localStorage.getItem("exhibitor_access_token");
    if (access_token) {
      setAuthenticated(true);
    } else {
      router.push("/login"); // トークンがない場合、ログインページにリダイレクト
    }
  }, []);

  const fetcher = (url: string) => fetch(url).then((res) => res.json());
  const { data, error } = useSWR(`${API_BASE_URL}/api/v1/forms`, fetcher);

  if (error) return <p>データの取得に失敗しました</p>;
  if (!data) return <p>読み込み中...</p>;

  const form = data.find((f: any) => f.formId === formId);
  if (!form) return <p>フォームが見つかりません</p>;

  const handleInputChange = (itemId: string, value: string) => {
    setAnswers((prev) => ({
      ...prev,
      [itemId]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const access_token = localStorage.getItem("exhibitor_access_token");
    if (!access_token) {
      alert("認証情報がありません。再ログインしてください。");
      return;
    }

    // `answers` を API のフォーマットに変換
    const formattedAnswers = Object.entries(answers).reduce((acc, [key, value], index) => {
      acc[`additionalProp${index + 1}`] = {
        question_id: key,
        answer_text: { value },
      };
      return acc;
    }, {});

    const response = await fetch(`${API_BASE_URL}/api/v1/forms/${formId}/responses`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${access_token}`,
      },
      body: JSON.stringify({ answers: formattedAnswers }),
    });

    if (response.ok) {
      alert("フォームを送信しました！");
    } else {
      alert("エラーが発生しました。");
    }
  };

  const renderItems = () => {
    if (!form || !form.items) return null;
  
    return form.items.map((item) => (
      <Question key={item.item_id} itemId={item.item_id} form={form}>
        {/* itemの種類によって異なる入力コンポーネントを表示 */}
        {item.item_question && (
          <>
          </>
        )}
      </Question>
    ));
  };

  return (
    <main className={styles.main}>
      <div className={styles.formTitleWrapper}>
        <h1>{form.info.title}</h1>
        <p>{form.info.description}</p>
      </div>
      <form onSubmit={handleSubmit}>
        <div>
          { renderItems() }
        </div>
        <button type="submit">送信</button>
      </form>
    </main>
  );
}
