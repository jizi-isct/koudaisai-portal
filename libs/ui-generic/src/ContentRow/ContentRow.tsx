import {Content} from "../lib/types";
import styles from "./ContentRow.module.css"

type Props = {
  content: Content
}

export function ContentRow({content}: Props) {
  return (
    <div className={styles.root} onClick={content.onClick}>
      <span className={styles.date}>{content.date}</span>
      <span className={styles.title}>{content.title}</span>
      <span className={styles.author}>{content.author}</span>
    </div>
  )
}