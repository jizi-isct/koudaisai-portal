'use client';

import styles from "./Header.module.css";
import Link from "next/link";
import { usePathname } from 'next/navigation';
import Logo from "@/components/Logo/Logo";

type HeaderProps = {
    header_type: "admin" | "members" ;
};
//ヘッダーのナビゲーションアイテムを定義
const HeaderItems = [
    { text: "ホーム", href: "/" },
    { text: "フォーム", href: "/forms/" },
    { text: "資料", href: "/documents/" },
    { text: "よくある質問", href: "/questions/" }
];

const Header = ({header_type}: HeaderProps) => {
    const pathname = usePathname();
    const isAdmin = header_type === "admin";
    return (
        <header className={`${styles.header} ${isAdmin ? styles.admin : styles.members}`}>
            <div className={styles.logoWrapper}>
                <Logo height={50} hasText/>
            </div>
            <div className={styles.menuWrapper}>
                {/* ヘッダーのナビゲーションボタン */}
                {HeaderItems.map(({ text, href }) => {
                    // 現在のパスとヘッダーのリンクのパスを比較して、アクティブなリンクを判断
                    const isActive = pathname === href;
                    return (
                    <Link
                        key={href}
                        href={href}
                        className={`${styles.headerNav} ${isActive ? styles.activeNav : styles.inactiveNav}`}
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
