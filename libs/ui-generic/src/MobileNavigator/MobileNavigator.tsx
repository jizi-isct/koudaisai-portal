"use client";

import styles from "./MobileNavigator.module.css";
import Link from "next/link";
import {headerItems} from "../lib/magicNumbers";
import {usePathname} from "next/navigation";


type Props = {
  header_type: "admin" | "members" ;
}

export function MobileNavigator({header_type}: Props) {
  const currentPath = usePathname();
  return (
    <nav className={`${styles.root} ${header_type === "admin" ? styles.admin : styles.members}`}>
      {/* ヘッダーのナビゲーションボタン */}
      {headerItems.map(({mobileText, href, class: className }) => {
        // 現在のパスとヘッダーのリンクのパスを比較して、アクティブなリンクを判断
        const isActive = currentPath === href;

        return (
          <Link
            key={href}
            href={href}
            className={`${styles.headerNav} ${styles[className]} ${isActive ? styles.activeNav : styles.inactiveNav}`}
          >
            <span>{mobileText}</span>
          </Link>
        );
      })}
    </nav>
  )
}