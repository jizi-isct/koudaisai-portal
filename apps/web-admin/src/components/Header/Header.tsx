'use client';

import styles from "./Header.module.css";
import Link from "next/link";
import { usePathname } from 'next/navigation';
import Logo from "@/components/Logo/Logo";

const headerItems = [
    { text: "ホーム", href: "/" },
    { text: "フォーム", href: "/forms/" },
    { text: "資料", href: "/documents/" },
    { text: "よくある質問", href: "/questions/" }
];

const Header = () => {
    const pathname = usePathname();
    return (
        <header className={styles.header}>
            <div className={styles.logoWrapper}>
                <Logo height={50} hasText/>
            </div>
            <div className={styles.menuWrapper}>
                {headerItems.map(({ text, href }) => {
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
