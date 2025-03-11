import styles from "./FormItem.module.css";
import {Item} from "@/lib/api";
import ParagraphInput from "@/components/Forms/ParagraphInput/ParagraphInput";
import TextInput from "@/components/Forms/TextInput/TextInput";

type Props = {
  readonly item: Item,
  setValue: (value: string) => void
};

export default function FormItem({item, setValue}: Props) {
  return (
    <div className={styles.questionWrapper}>
      <h2 className={styles.title}>{item.title}</h2>
      <p className={styles.description}>{item.description}</p>
      {item?.item_question?.question.required && <p className={styles.required}>必須</p>}
      {
        item.item_question?.question?.question_text &&
        item.item_question!.question!.question_text!.paragraph ?
          <ParagraphInput
            placeholder="長文を入力"
            onChange={setValue}
          />
          :
          <TextInput
            placeholder="短文を入力"
            onChange={setValue}
          />
      }
    </div>
  );
};
