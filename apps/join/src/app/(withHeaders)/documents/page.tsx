import {getApiFetchClient} from "@koudaisai/shared-api";
import ViewDocumentsWrapper from "../../../components/ViewDocumentsWrapper";
import {Heading1} from "@koudaisai/shared-ui";
import styles from "./page.module.css";

export default async function Page_() {
  const fetchClient = getApiFetchClient("https://portal.koudaisai.jp/api/v2")

  // 資料一覧の取得
  const {data, error} = await fetchClient.GET("/documents/by-category")
  if (error || !data) {
    throw error;
  }

  return (
    <main className={styles.main}>
      <Heading1 emoji={"📚"}>資料</Heading1>
      <div className={styles.container}>
        <ViewDocumentsWrapper documents={data}/>
      </div>
    </main>
    
  )
}