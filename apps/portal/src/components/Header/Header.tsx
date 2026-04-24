"use client";

import styles from "./Header.module.css";
import Image from "next/image";
import {usePathname} from "next/navigation";
import {LargeButton, LargePulldown, PulldownItem} from "@koudaisai/shared-ui";
import icon_account from './assets/icon_account.svg'

import membersLogo from "./assets/members_logo.png";

const headerItems = [
  {desktopText: "ホーム", mobileText: "ホーム", emoji: "🏠", href: "/", class: "navHome"},
  {desktopText: "フォーム", mobileText: "フォーム", emoji: "📄", href: "/forms/", class: "navForm"},
  {desktopText: "資料", mobileText: "資料", emoji: "📚", href: "/documents/", class: "navDocuments"},
  {desktopText: "よくある質問", mobileText: "FAQ", emoji: "❓", href: "/questions/", class: "navQuestions"}
];

type Props = {
  logout: () => void;
}

export function Header({logout}: Props) {
  const currentPath = usePathname();

  return (
    <header className={`${styles.header}`}>
      <div className={styles.logo}>
        <Image
          src={membersLogo}
          alt="Koudaisai Portal Logo"
          className={styles.logoMark}
          width={50}
          height={50}
        />
        <span className={styles.logoType}>工大祭ポータル</span>
      </div>
      <nav className={styles.navigation}>
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
            <LargeButton key={href} type={isActive ? "primary" : "secondary"} href={href}>
              {desktopText}
            </LargeButton>
          );
        })}
      </nav>
      <div className={styles.pulldown}>
        <LargePulldown type={"secondary"} items={[{label: "ログアウト", onClick: () => {logout()}}]}><Image height={25} alt={"ac"} src={icon_account}/></LargePulldown>
      </div>
    </header>
  );
};