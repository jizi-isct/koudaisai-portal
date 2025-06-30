"use client";

import styles from "./Header.module.css";
import Link from "next/link";
import Image from "next/image";
import {usePathname, useRouter} from "next/navigation";
import {isLoggedInAdmin, isLoggedInMembers, logout} from "@/lib";
import {useEffect, useState} from "react";
import {headerItemsAdmin, headerItemsMembers} from "../lib/magicNumbers";

const adminLogo = "/components/generic/Header/admin_logo.jpg"
const membersLogo = "/components/generic/Header/members_logo.png"
const accountIcon = "/components/generic/Header/icon_account.svg"
const arrowIcon = "/components/generic/Header/arrow.svg"

type HeaderProps = {
    header_type: "admin" | "members" ;
    titleColor?: "white" | "black";
};

export const Header = ({header_type, titleColor = "black"}: HeaderProps) => {
    const router = useRouter();
    // ヘッダーのユーザアイコンのドロップダウンの状態を管理
    const [isOpen, setIsOpen] = useState(false);
    const handleLogout = async () => {
        await logout()
        router.push("/")
    }
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


    // ページのサイズを管理するstate
    const [pageSize, setPageSize] = useState<{
        width: number;
        height: number;
    }>({
        width: 1024, // 仮の幅
        height: 1000, // 仮の高さ
    });

    // 画面回転時などにページサイズを更新する
    useEffect(() => {
        if (typeof window === 'undefined') return;

        const updatePageSize = () => {
        const width = window.innerWidth;
        const height = document.body.offsetHeight + document.body.getBoundingClientRect().top;
        console.log("width", width, "height", height);
        setPageSize({ width, height });
        };

        updatePageSize(); // 初回実行

        window.addEventListener('resize', updatePageSize);
        return () => {
        window.removeEventListener('resize', updatePageSize);
        };
    }, []);


    return (
      <header
        className={`${styles.header} ${isAdmin ? styles.admin : styles.members}`}
        style= {{ height: `${pageSize.width <= 768 ? pageSize.height - 160 : 100}px` }} >
          <div className={styles.logoWrapper}>
              <Image
                src={isAdmin ? adminLogo : membersLogo}
                alt="Koudaisai Portal Admin Site Logo"
                width={50}
                height={50}
              />
              <div className={styles.logoTextWrapper}>
                  <h1 className={`${styles.logoText} ${titleColor === "white" && styles.logoTextWhite}`}>{isAdmin ? "工大祭ポータル管理サイト" : "工大祭ポータル"}</h1>
              </div>
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
                      <a onClick={handleLogout} className={styles.userDropdown}>ログアウト</a>
                  </div>
              </div>
              <div className={`${styles.userWrapperLoggedOut} ${loggedIn ? styles.hiddenUserWrapper : ""}`}>
                  <Link href={isAdmin ? process.env.NEXT_PUBLIC_AUTH_BASE_URL + "/admin/login" : "/login/"}>ログイン</Link>
              </div>
          </div>
          <div className={`${styles.menuWrapper}`}>
              {/* ヘッダーのナビゲーションボタン */}
              {(header_type === "members" ? headerItemsMembers : headerItemsAdmin).map(({
                                                                                            desktopText,
                                                                                            mobileText,
                                                                                            href,
                                                                                            class: className
                                                                                        }) => {
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