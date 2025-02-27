import styles from "./Question.module.css";
import Image from "next/image";
import TextBox from "@/components/Forms/TextBox/TextBox";
import React from "react";

type QuestionProps = {
    children: React.ReactNode;
  };

const Question: React.FC<QuestionProps> = ({children}) => {
  return (
    <div className={styles.formWrapper}>
        <div className={styles.questionTitleWrapper}>
            <div className={styles.questionTitle}><TextBox fontSize={18} width={250} placeholder="質問"/></div>
            <div className={styles.questionDescription}><TextBox fontSize={16} width={650} placeholder="説明"/></div>
        </div>
        <div className={styles.questionWrapper}>
            {children}
        </div>
        <div className={styles.buttonsWrapper}>
            <div className={styles.arrowsWrapper}>
            <Image src="/forms/arrowUp.svg" width={30} height={30} alt="arrrow" />
            <Image src="/forms/arrowDown.svg" width={30} height={30} alt="arrrow" />
            </div>
            <div className={styles.buttons}>
            <label>必須</label>
            <input type="checkbox" className={styles.checkBox} />
            <Image src="/forms/delete.svg" width={30} height={30} alt="delete" />
            </div>
        </div>
    </div>
  );
};

export default Question;
