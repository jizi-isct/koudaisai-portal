import styles from "./Question.module.css";

type QuestionProps = {
    children?: React.ReactNode;
    itemId: string;
    form: any;
};

type Item = {
  item_id: string;
  created_at: string;
  updated_at: string;
  title: string;
  description?: string;
  item_page_break?: object; // ページ区切りアイテム
  item_text?: object;       // テキストアイテム
  item_question?: {
    question: {
      question_id: string;
      created_at: string;
      updated_at: string;
      required: boolean;
      question_text: {
        paragraph: boolean;
      };
    };
  };
};

const findItemById = (items: Item[], itemId: string): Item | undefined => {
  return items.find(item => item.item_id === itemId);
};

const Question: React.FC<QuestionProps> = ({children, itemId, form}: QuestionProps) => {
  const item = findItemById(form?.items ?? [], itemId);
  return (
    <div className={styles.questionWrapper}>
        <h2 className={styles.title}>{item?.title ?? "データなし"}</h2>
        <p className={styles.description}>{item?.description ?? "データなし"}</p>
        {item?.item_question?.question.required && <p className={styles.required}>必須</p>}
        {children}
    </div>
  );
};

export default Question;
