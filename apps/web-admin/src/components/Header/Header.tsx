'use client';

import styles from "./Header.module.css";
import Link from "next/link";
import { usePathname } from 'next/navigation';
import Image from "next/image";

type HeaderProps = {
    header_type: "admin" | "members" ;
    currentPath?: string;
};
//ヘッダーのナビゲーションアイテムを定義
const HeaderItems = [
    { text: "ホーム", href: "/", class: "navHome" },
    { text: "フォーム", href: "/forms/", class: "navForm" },
    { text: "資料", href: "/documents/", class: "navDocuments" },
    { text: "よくある質問", href: "/questions/", class: "navQuestions" }
];

const Header = ({header_type, currentPath}: HeaderProps) => {
    const isAdmin = header_type === "admin";
    return (
        <header className={`${styles.header} ${isAdmin ? styles.admin : styles.members}`}>
            <div className={styles.logoWrapper}>
            <Image
                src="/admin/components/Logo/logo.jpg"
                alt="Koudaisai Portal Admin Site Logo"
                width={50}
                height={50}
            />
            <div className={styles.logoTextWrapper}>
                <h1 className={styles.logoText}>{isAdmin ? "工大祭ポータル管理サイト" : "工大祭ポータル"}</h1>
            </div>
            </div>
            <div className={styles.menuWrapper}>
                {/* ヘッダーのナビゲーションボタン */}
                {HeaderItems.map(({ text, href, class: className }) => {
                    // 現在のパスとヘッダーのリンクのパスを比較して、アクティブなリンクを判断
                    const isActive = currentPath === href;
                    return (
                    <Link
                        key={href}
                        href={href}
                        className={`${styles.headerNav} ${styles[className]} ${isActive ? styles.activeNav : styles.inactiveNav}`}
                    >
                        {text}
                    </Link>
                    );
                })}
            </div>
        </header>
    );
};

export default Header;
