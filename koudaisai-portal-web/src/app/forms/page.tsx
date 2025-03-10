"use client";
import styles from "./page.module.css";
import { useRouter } from "next/navigation";
import useSWR from "swr";
import Forms from "@/components/Forms/Lists/Lists";

export default function Page() {
  const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL;
  const router = useRouter();

  const [authenticated, setAuthenticated] = useState(false);

  useEffect(() => {
    const access_token = localStorage.getItem("exhibitor_access_token");
    if (access_token) {
      setAuthenticated(true);
    } else {
      router.push("/login"); // トークンがない場合、ログインページにリダイレクト
    }
  }, []);

  const fetcher = (url: string) => {
    const access_token = localStorage.getItem("exhibitor_access_token");
  
    return fetch(url, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${access_token}`,
      },
    }).then((res) => res.json());
  };
  const { data, error } = useSWR(`${API_BASE_URL}/api/v1/forms`, fetcher);

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
