"use client";
import styles from "./page.module.css";
import {useSearchParams} from "next/navigation";
import {useState} from "react";
import {$api, fetchClient, FormResponse, Item} from "@/lib/api";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import FormItem from "@/components/Forms/FormItem/FormItem";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const searchParams = useSearchParams();
  const formId = searchParams.get("formId");

  if (!formId) {
    window.location.assign("/404")
  }

  const [formResponse, setFormResponse] = useState<FormResponse>({
    answers: {}
  }); // 回答を管理する state

  const {data, error} = $api.useQuery(
    "get",
    "/forms/{form_id}",
    {
      params: {
        path: {
          form_id: formId!
        }
      }
    }
  )
  const form = data

  if (error) return <p>データの取得に失敗しました</p>;
  if (!data) return <p>読み込み中...</p>;

  const handleInputChange = (item: Item) => (value: string) => {
    setFormResponse({
      ...formResponse,
      answers: {
        ...formResponse.answers,
        [item.item_id]: {
          item_id: item.item_id,
          answer_text: {
            value: value
          }
        }
      }
    });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const {response} = await fetchClient.POST(
      "/forms/{form_id}/responses",
      {
        params: {
          path: {
            form_id: formId!
          }
        },
        body: formResponse
      }
    )

    if (response.ok) {
      alert("フォームを送信しました！");
    } else {
      alert("エラーが発生しました。");
    }
  };

  const renderItems = () => {
    if (!form || !form.items) return null;

    return form.items.map((item) => (
      <FormItem
        key={item.item_id}
        item={item}
        setValue={handleInputChange(item)}
      />
    ));
  };

  return (
    <main className={styles.main}>
      <div className={styles.formTitleWrapper}>
        <h1>{form?.info.title}</h1>
        <p>{form?.info.description}</p>
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