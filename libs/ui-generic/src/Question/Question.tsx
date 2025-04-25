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
      <p className={styles.question}>{content.question}</p>
      <p className={styles.answer}>{content.answer}</p>
    </div>
  )
}