"use client";

import styles from "./MobileNavigator.module.css";
import Link from "next/link";
import {headerItemsAdmin, headerItemsMembers} from "../lib/magicNumbers";
import { Button } from "antd";
import {usePathname, useRouter} from "next/navigation";


type Props = {
  header_type: "admin" | "members";
  logout: () => Promise<void>;
  isLoggedIn?: boolean;
}

export function MobileNavigator({header_type, logout, isLoggedIn}: Props) {
  const router = useRouter();
  const currentPath = usePathname();
  const handleLogout = async () => {
    await logout();
    router.push("/");
  }
  return (
    <div className={styles.root}>
      {
        isLoggedIn === undefined ? <></> :
          isLoggedIn
            ? <div className={styles.logout}>
              <Button type="primary" style={{ alignSelf: "flex-start" }} onClick={handleLogout}>ログアウト</Button>
            </div>
            : <div className={styles.login}>
              <Button type="primary" style={{ alignSelf: "flex-start" }} onClick={() => router.push("/login")}>ログイン</Button>
            </div>
      }
      <nav className={`${styles.nav} ${header_type === "admin" ? styles.admin : styles.members}`}>
        {/* ヘッダーのナビゲーションボタン */}
        {(header_type === "members" ? headerItemsMembers : headerItemsAdmin).map(({
                                                                                    mobileText,
                                                                                    emoji,
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
              <span className={styles.emoji}>{emoji}</span>
              <span className={styles.text}>{mobileText}</span>
            </Link>
          );
        })}
      </nav>
    </div>
  )
}