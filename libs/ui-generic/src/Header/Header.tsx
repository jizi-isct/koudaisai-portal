"use client";

import styles from "./Header.module.css";
import Link from "next/link";
import Image from "next/image";
import { useState } from "react";
import logo from "./assets/logo.jpg";
import accountIcon from "./assets/icon_account.svg";
import arrowIcon from "./assets/arrow.svg";

type HeaderProps = {
    header_type: "admin" | "members" ;
    currentPath?: string;
    isLoggedIn?: boolean | null;
};

//ヘッダーのナビゲーションアイテムを定義
const HeaderItems = [
    { text: "ホーム", href: "/", class: "navHome" },
    { text: "フォーム", href: "/forms/", class: "navForm" },
    { text: "資料", href: "/documents/", class: "navDocuments" },
    { text: "よくある質問", href: "/questions/", class: "navQuestions" }
];

export const Header = ({header_type, currentPath, isLoggedIn}: HeaderProps) => {
    // ヘッダーのユーザアイコンのドロップダウンの状態を管理
    const [isOpen, setIsOpen] = useState(false);
    const toggleDropdown = () => {
        setIsOpen(!isOpen);
    }

    // ヘッダーのタイプによってスタイルを変更
    const isAdmin = header_type === "admin";

    return (
        <header className={`${styles.header} ${isAdmin ? styles.admin : styles.members}`}>
            <div className={styles.logoWrapper}>
            <Image
                src={logo}
                alt="Koudaisai Portal Admin Site Logo"
                width={50}
                height={50}
            />
            <div className={styles.logoTextWrapper}>
                <h1 className={styles.logoText}>{isAdmin ? "工大祭ポータル管理サイト" : "工大祭ポータル"}</h1>
            </div>
            <div className={styles.userWrapper}>
                <div className={`${styles.userWrapperLoggedIn} ${isLoggedIn ? "" : styles.hiddenUserWrapper}`}>
                    <div className={styles.user} onClick={toggleDropdown}>
                        <Image
                            src={accountIcon}
                            alt="User Account Icon"
                            width={28}
                            height={28}
                        />
                        <Image
                            src={arrowIcon}
                            alt="Arrow Down Icon"
                            className={styles.arrowIcon}
                            width={24}
                            height={24}
                        />
                    </div>
                    <div className={`${styles.userDropdownWrapper} ${isOpen ? styles.dropdownOpen : styles.dropdownClosed}`}>
                        <Link href="" className={styles.userDropdown}>ログアウト</Link>
                        <Link href="" className={styles.userDropdown}>ログアウト</Link>
                    </div>
                </div>
                <div className={`${styles.userWrapperLoggedOut} ${isLoggedIn ? styles.hiddenUserWrapper : ""}`}>
                    <Link href="/login/">ログイン</Link>
                </div>
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