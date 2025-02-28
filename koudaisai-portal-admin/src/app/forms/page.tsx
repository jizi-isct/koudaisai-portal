import Image from "next/image";
import styles from "./page.module.css";
import HeaderLists from "@/components/Forms/Lists/Lists";

export default function Home() {
  return (
    <div className={styles.page}>
      <main className={styles.main}>
        <div className={styles.formsWrapper}>
            <HeaderLists
                title="traP缶バッジ受け取り"
                status="未回答"
                dueDate="5/20"
                summary="プログラミング基礎講習会のフィードバックアンケートに答えた受講者の方と、講習会のTAにtraP缶バッジをひとつ配布しています。 まだ回答していない人は以下のリンクからお願いします！"
            />
            <HeaderLists
                title="traP缶バッジ受け取り"
                status="未回答"
                dueDate="5/20"
                summary="プログラミング基礎講習会のフィードバックアンケートに答えた受講者の方と、講習会のTAにtraP缶バッジをひとつ配布しています。 まだ回答していない人は以下のリンクからお願いします！"
            />
            <HeaderLists
                title="traP缶バッジ受け取り"
                status="未回答"
                dueDate="5/20"
                summary="プログラミング基礎講習会のフィードバックアンケートに答えた受講者の方と、講習会のTAにtraP缶バッジをひとつ配布しています。 まだ回答していない人は以下のリンクからお願いします！"
            />
            <HeaderLists
                title="traP缶バッジ受け取り"
                status="未回答"
                dueDate="5/20"
                summary="プログラミング基礎講習会のフィードバックアンケートに答えた受講者の方と、講習会のTAにtraP缶バッジをひとつ配布しています。 まだ回答していない人は以下のリンクからお願いします！"
            />
        </div>
      </main>
    </div>
  );
}
