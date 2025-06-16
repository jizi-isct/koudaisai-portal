"use client";

import styles from "./page.module.css";
import Image from "next/image";

export default function Page() {
    return (
        <div className={styles.page}>
            <main className={styles.main}>
                <Image
                    src="/admin/components/Logo/logo.jpg"
                    alt="Koudaisai Portal Admin Site Logo"
                    width={150}
                    height={150}
                />
                <h1>ようこそ</h1>
              <h2>ページ一覧</h2>
              <ul>
                <li>
                  <a href="/admin/forms">フォーム一覧</a>
                </li>
                <li>
                  <a href="/admin/documents">資料一覧</a>
                </li>
                <li>
                  <a href="/admin/notifications">通知一覧</a>
                </li>
              </ul>
            </main>
        </div>
    );
}
