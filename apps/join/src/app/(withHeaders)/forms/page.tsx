import styles from "../documents/page.module.css";
import {Heading1} from "@koudaisai/shared-ui";
import {getApiFetchClient} from "@koudaisai/shared-api";
import {ViewFormCards} from "../../../components/form/view/ViewFormCards";

export default async function Page() {
  const fetchClient = getApiFetchClient("https://portal.koudaisai.jp/api/v2")

  // フォーム一覧の取得
  const {data, error} = await fetchClient.GET("/forms")
  if (error || !data) {
    throw error;
  }

  return (
    <main className={styles.main}>
      <Heading1 emoji={"📝"}>フォーム</Heading1>
      <div className={styles.container}>
        <ViewFormCards forms={data}/>
      </div>
    </main>
  )
}