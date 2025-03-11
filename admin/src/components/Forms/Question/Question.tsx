import styles from "./Question.module.css";
import Image from "next/image";
import TextInput from "@/components/Forms/TextInput/TextInput";
import React from "react";

type QuestionProps = {
  children?: React.ReactNode;
  itemId: string;
  form: any;
  updateItem?: (itemId: string, title: string, description: string) => void;
  toggleRequired?: (itemId: string) => void;
  reorderQuestionUp: (itemId: string) => void;
  reorderQuestionDown: (itemId: string) => void;
  deleteQuestion: (itemId: string) => void;
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

const Question: React.FC<QuestionProps> = ({children, itemId, form, updateItem, toggleRequired, reorderQuestionUp, reorderQuestionDown, deleteQuestion}) => {
  const item = findItemById(form?.items ?? [], itemId);

  const handleTitleChange = (title: string) => {
    if (updateItem) {
      updateItem("641fe788-ab24-44c7-317b-d257158d16bd", title, null);
    }
  };

  const handleDescriptionChange = (description: string) => {
    if (updateItem) {
      updateItem(itemId, null, description);
    }
  };

  const requiredStatus = (itemId: string) => {
    return item?.item_question?.question.required ?? false;
  };

  return (
    <div className={styles.formWrapper}>
        <div className={styles.questionTitleWrapper}>
            <div className={styles.questionTitle}>
              <TextInput
                fontSize={18}
                width={400}
                placeholder="タイトルを入力"
                value={item?.title ?? "データなし"}
                onChange={handleTitleChange}
                args={[]}
              />
            </div>
            <div className={styles.questionDescription}>
              <TextInput
                fontSize={15}
                width={800}
                placeholder="説明を入力"
                value={item?.description ?? "データなし"}
                onChange={handleDescriptionChange}
                args={[]}
              />
            </div>
        </div>
        <div className={styles.questionWrapper}>
            {children}
        </div>
        <div className={styles.buttonsWrapper}>
            <div className={styles.arrowsWrapper}>
              <a href="#" onClick={() => reorderQuestionUp(itemId)} className={styles.arrowUp}><Image src="/forms/arrowUp.svg" width={30} height={30} alt="arrrow" /></a>
              <a href="#" onClick={() => reorderQuestionDown(itemId)}className={styles.arrowUp}><Image src="/forms/arrowDown.svg" width={30} height={30} alt="arrrow" /></a>
            </div>
            <div className={styles.buttons}>
              <label>必須</label>
              <input checked={requiredStatus(itemId)} type="checkbox" onChange={() => toggleRequired(itemId)} className={styles.checkBox} />
              <a href="#" onClick={() => deleteQuestion(itemId)} className={styles.deleteButtonWrapper}>
                <Image src="/forms/delete.svg" width={25} height={25} alt="delete" />
              </a>
              
            </div>
        </div>
    </div>
  );
};

export default Question;
