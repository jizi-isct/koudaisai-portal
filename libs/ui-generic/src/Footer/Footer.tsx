"use client";

import styles from "./Footer.module.css";
import Link from "next/link";
import Image from "next/image";
import logo from "./assets/members_logo.svg";

export const Footer = () => {
    return (
        <footer className={styles.footer}>
            <div className={styles.logoWrapper}>
                <Image
                    src={logo}
                    alt="Koudaisai Portal Admin Site Logo"
                    width={40}
                    height={40}
                />
            </div>
            <div className={styles.navWrapper}>
                <Link href="/" className={styles.navItem}>ホーム</Link>
                <Link href="/questions/" className={styles.navItem}>よくある質問</Link>
                <Link href="/forms/" className={styles.navItem}>申請一覧</Link>
            </div>
            <p className={styles.copyrightText}>©︎ 2025 JIZI All Rights Reserved.</p>
        </footer>
    );
};