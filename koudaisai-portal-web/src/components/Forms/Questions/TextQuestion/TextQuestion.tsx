import styles from "./TextQuestion.module.css";

type TextQuestionProps = {
    title: string;
    description: string;
    required?: boolean;
};

const TextQuestion = ({title, description, required = false}: TextQuestionProps) => {
  return (
    <div className={styles.questionWrapper}>
        <h2 className={styles.title}>{title}</h2>
        <p className={styles.description}>{description}</p>
        {required && <p className={styles.required}>必須</p>}
        <input type="text" className={styles.textBox} placeholder="回答を入力"/>
    </div>
  );
};

export default TextQuestion;
