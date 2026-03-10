import styles from "./Faq.module.css"

type Props = {
  number: number
  content: 
  {
    question: string
    answer: string
  }
}

export function Faq({number, content}: Props) {
  return (
    <div className={styles.root}>
      <p className={styles.question}>
        <span className={styles.prefix}>Q{number}. </span>
        {content.question}
      </p>
      <p className={styles.answer}>
        <span className={styles.prefix}>A{number}. </span>
        {content.answer}
      </p>
    </div>
  )
}