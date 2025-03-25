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
                    width={300}
                    height={300}
                />
                <h1>ようこそ</h1>
                <a href={process.env.NEXT_PUBLIC_AUTH_BASE_URL + "/admin/login"}>ログインする</a>
            </main>
        </div>
    );
}
