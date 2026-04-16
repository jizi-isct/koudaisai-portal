"use client";

import styles from "./Header.module.css";
import Link from "next/link";
import Image from "next/image";
import {usePathname} from "next/navigation";
import {useEffect, useState} from "react";
import {headerItemsMembers} from "../lib/magicNumbers";

import membersLogo from "./assets/members_logo.png";

type HeaderProps = {
    titleColor?: "white" | "black";
};

export const Header = ({titleColor = "black"}: HeaderProps) => {
    const currentPath = usePathname();

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
        className={`${styles.header} ${styles.members}`}
        style= {{ height: `${pageSize.width <= 768 ? pageSize.height - 160 : 100}px` }} >
          <div className={styles.logoWrapper}>
              <Image
                src={ membersLogo}
                alt="Koudaisai Portal Logo"
                width={50}
                height={50}
              />
              <div className={styles.logoTextWrapper}>
                  <h1 className={`${styles.logoText} ${titleColor === "white" && styles.logoTextWhite}`}>{"工大祭ポータル"}</h1>
              </div>
          </div>
          <div className={`${styles.menuWrapper}`}>
              {/* ヘッダーのナビゲーションボタン */}
              {headerItemsMembers.map(({
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