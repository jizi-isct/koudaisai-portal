import styles from "./Question.module.css"

type Props = {
  content: 
  {
    question: string
    answer: string
  }
}

export function Question({content}: Props) {
  return (
    <div className={styles.root}>
      <span className={styles.date}>{content.question}</span>
      <span className={styles.date}>{content.answer}</span>
    </div>
  )
}