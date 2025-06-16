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
            </main>
        </div>
    );
}
