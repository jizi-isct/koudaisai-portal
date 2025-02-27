import styles from "./page.module.css";

export default async function Page({
    params,
  }: {
    params: Promise<{ formId: string }>
  }) {
    const formId = (await params).formId
    return (
        <div className={styles.page}>
        <main className={styles.main}>
            <div className={styles.formTitleWrapper}>
                <h1>工夜祭団体 参加申請フォーム</h1>
                <p>工夜祭2024を工大祭2日目の17:40~18:40で開催いたします。
                参加を希望される団体は「工夜祭2024参加募集要項」をご一読の上、このフォームにお答えください。</p>
            </div>
            <div className={styles.formWrapper}>
                
            </div>
        </main>
        </div>
    );
  }