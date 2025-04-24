import {ReactNode} from "react";
import styles from "./Heading2.module.css";

type Props = {
  children: ReactNode,
  emoji: string
}

/**
 * 見出し
 * @param children 子要素
 * @param emoji 見出しの頭に表示する絵文字
 * @constructor
 */
export function Heading2({children, emoji}: Props) {
  return (
    <h1 className={styles.root}>
      <span className={styles.emoji}>
        <span className={styles.emojiBackground}>{emoji}</span>
        {emoji}
      </span>
      <span className={styles.heading}>{children}</span>
    </h1>
  )
}