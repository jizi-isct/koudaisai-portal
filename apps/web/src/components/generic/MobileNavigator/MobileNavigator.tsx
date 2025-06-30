"use client";

import styles from "./MobileNavigator.module.css";
import Link from "next/link";
import {headerItemsAdmin, headerItemsMembers} from "../lib/magicNumbers";
import {usePathname, useRouter} from "next/navigation";
import {ButtonCompact} from "@/components/generic/ButtonCompact";
import {useIsLoggedInMembers} from "@/lib";


type Props = {
  header_type: "admin" | "members";
}

export function MobileNavigator({header_type}: Props) {
  const router = useRouter();
  const isLoggedIn = useIsLoggedInMembers(); // 仮のログイン状態
  const currentPath = usePathname();
  return (
    <div className={styles.root}>
      {
        !isLoggedIn && <div className={styles.login}>
                <ButtonCompact text={"ログイン"} onClick={() => router.push("/login")}/>
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