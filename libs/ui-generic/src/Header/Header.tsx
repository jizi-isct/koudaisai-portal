"use client";

import styles from "./Header.module.css";
import Link from "next/link";
import Image from "next/image";
import { usePathname } from "next/navigation";
import { isLoggedInAdmin,isLoggedInMembers } from "@koudaisai-portal/util";
import { useEffect, useState } from "react";
import adminLogo from "./assets/admin_logo.jpg";
import membersLogo from "./assets/members_logo.svg";
import accountIcon from "./assets/icon_account.svg";
import arrowIcon from "./assets/arrow.svg";

type HeaderProps = {
    header_type: "admin" | "members" ;
    titleColor?: "white" | "black";
};

//ヘッダーのナビゲーションアイテムを定義
const HeaderItems = [
    { desktopText: "ホーム", mobileText: "ホーム", href: "/", class: "navHome" },
    { desktopText: "フォーム", mobileText: "フォーム", href: "/forms/", class: "navForm" },
    { desktopText: "資料", mobileText: "資料", href: "/documents/", class: "navDocuments" },
    { desktopText: "よくある質問", mobileText: "FAQ", href: "/questions/", class: "navQuestions" }
];

export const Header = ({header_type, titleColor = "black"}: HeaderProps) => {
    // ヘッダーのユーザアイコンのドロップダウンの状態を管理
    const [isOpen, setIsOpen] = useState(false);
    const toggleDropdown = () => {
        setIsOpen(!isOpen);
    }

    // ヘッダーのタイプによってスタイルを変更
    const isAdmin = header_type === "admin";
    
    const currentPath = usePathname();

     //ログイン状態を管理するstate
    //nullはログイン状態がわからないことを示す
    const [loggedIn, setLoggedIn] = useState<boolean | null>(null);

    useEffect(() => {
        if (header_type === "admin") {
            isLoggedInAdmin().then(setLoggedIn);
        } else if (header_type === "members") {
            isLoggedInMembers().then(setLoggedIn);
        }
    }, [header_type]);

    return (
        <header className={`${styles.header} ${isAdmin ? styles.admin : styles.members}`}>
            <div className={styles.logoWrapper}>
            <Image
                src={isAdmin ? adminLogo : membersLogo}
                alt="Koudaisai Portal Admin Site Logo"
                width={50}
                height={50}
            />
            <div className={styles.logoTextWrapper}>
                <h1 className={`${styles.logoText} ${titleColor == "white" && styles.logoTextWhite}`}>{isAdmin ? "工大祭ポータル管理サイト" : "工大祭ポータル"}</h1>
            </div>
            <div className={styles.userWrapper}>
                <div className={`${styles.userWrapperLoggedIn} ${loggedIn ? "" : styles.hiddenUserWrapper}`}>
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
                <div className={`${styles.userWrapperLoggedOut} ${loggedIn ? styles.hiddenUserWrapper : ""}`}>
                    <Link href={isAdmin ? process.env.NEXT_PUBLIC_AUTH_BASE_URL + "/admin/login" : "/login/"}>ログイン</Link>
                </div>
            </div>
            </div>
            <div className={styles.menuWrapper}>
                {/* ヘッダーのナビゲーションボタン */}
                {HeaderItems.map(({ desktopText, mobileText, href, class: className }) => {
                    // 現在のパスとヘッダーのリンクのパスを比較して、アクティブなリンクを判断
                    const isActive = currentPath === href;

                    return (
                    <Link
                        key={href}
                        href={href}
                        className={`${styles.headerNav} ${styles[className]} ${isActive ? styles.activeNav : styles.inactiveNav}`}
                    >
                        <span className={styles.desktopText}>{desktopText}</span>
                        <span className={styles.mobileText}>{mobileText}</span>
                    </Link>
                    );
                })}
            </div>
        </header>
    );
};