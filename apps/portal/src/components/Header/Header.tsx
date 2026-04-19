"use client";

import styles from "./Header.module.css";
import Image from "next/image";
import {usePathname} from "next/navigation";
import {useEffect, useState} from "react";
import {LargeButton} from "@koudaisai/shared-ui";

import membersLogo from "./assets/members_logo.png";

const headerItems = [
  {desktopText: "ホーム", mobileText: "ホーム", emoji: "🏠", href: "/", class: "navHome"},
  {desktopText: "フォーム", mobileText: "フォーム", emoji: "📄", href: "/forms/", class: "navForm"},
  {desktopText: "資料", mobileText: "資料", emoji: "📚", href: "/documents/", class: "navDocuments"},
  {desktopText: "よくある質問", mobileText: "FAQ", emoji: "❓", href: "/questions/", class: "navQuestions"}
];

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
      setPageSize({width, height});
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
      style={{height: `${pageSize.width <= 768 ? pageSize.height - 160 : 100}px`}}>
      <div className={styles.logoWrapper}>
        <Image
          src={membersLogo}
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
        {headerItems.map(({
                            desktopText,
                            mobileText,
                            href,
                            class: className
                          }) => {
          // 現在のパスとヘッダーのリンクのパスを比較して、アクティブなリンクを判断
          const isActive = currentPath === href;

          return (
            <LargeButton type={isActive ? "primary" : "secondary"} href={href}>
              <span className={styles.desktopText}>{desktopText}</span>
              <span className={styles.mobileText}>{mobileText}</span>
            </LargeButton>
          );
        })}
      </div>
    </header>
  );
};