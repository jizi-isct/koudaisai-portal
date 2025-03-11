import styles from "./FormItem.module.css";
import Image from "next/image";
import TextInput from "@/components/Forms/TextInput/TextInput";
import React from "react";
import {Item} from "@/lib/api";
import FormItemTypeSelect from "@/components/Forms/FormItem/FormItemTypeSelector/FormItemTypeSelect";

type Props = {
  readonly item: Item;
  setItem: (item: Item) => void;
  moveUp: () => void;
  moveDown: () => void;
  delete_: () => void;
};

export function FormItem({item, setItem, moveUp, moveDown, delete_}: Props) {
  const handleTitleChange = (title: string) => {
    setItem(
      {
        ...item,
        title: title
      }
    )
  };

  const handleDescriptionChange = (description: string) => {
    setItem(
      {
        ...item,
        description: description
      }
    )
  };

  const handleFormItemTypeChange = (value: ("question_text" | "text" | "page_break")) => {
    switch (value) {
      case "question_text":
        setItem(
          {
            ...item,
            item_question: {
              question: {
                question_id: crypto.randomUUID(),
                required: true,
                question_text: {
                  paragraph: false
                }
              }
            },
            item_page_break: undefined,
            item_text: undefined,
          }
        )
        break;
      case "page_break":
        setItem(
          {
            ...item,
            item_question: undefined,
            item_page_break: {},
            item_text: undefined
          }
        )
        break;
      case "text":
        setItem(
          {
            ...item,
            item_question: undefined,
            item_page_break: undefined,
            item_text: {}
          }
        )
        break;
    }
  }

  const handleToggleRequired = (value: boolean) => {
    setItem(
      {
        ...item,
        item_question: {
          question: {
            ...item.item_question!.question,
            required: value
          }
        }
      }
    )
  }
  const handleToggleParagraph = (value: boolean) => {
    setItem(
      {
        ...item,
        item_question: {
          question: {
            ...item.item_question!.question,
            question_text: {
              paragraph: value
            }
          }
        }
      }
    )
  }
  const getItemType = () => {
    if (item.item_question?.question?.question_text) {
      return "question_text"
    }
    if (item.item_text) {
      return "text"
    }
    if (item.item_page_break) {
      return "page_break"
    }
    throw Error("Illegal item type")
  }

  return (
    <div className={styles.formWrapper}>
      <div className={styles.questionTitleWrapper}>
        <div className={styles.questionTitle}>
          <TextInput
            fontSize={18}
            width={400}
            placeholder="タイトルを入力"
            value={item.title}
            onChange={handleTitleChange}
            args={[]}
          />
          <FormItemTypeSelect onChange={handleFormItemTypeChange} value={getItemType()}/>
        </div>
        <div className={styles.questionDescription}>
          <TextInput
            fontSize={15}
            width={800}
            placeholder="説明を入力"
            value={item.description}
            onChange={handleDescriptionChange}
            args={[]}
          />
        </div>
      </div>
      <div className={styles.questionWrapper}>
        {
          item.item_question?.question?.question_text &&
            <>
                <label>長文回答にする</label>
                <input defaultChecked={item.item_question?.question?.question_text.paragraph} type="checkbox"
                       onChange={(e) => handleToggleParagraph(e.target.checked)} className={styles.checkBox}/>
            </>
        }
      </div>
      <div className={styles.buttonsWrapper}>
        <div className={styles.arrowsWrapper}>
          <a href="#" onClick={() => moveUp()} className={styles.arrowUp}><Image src="/forms/arrowUp.svg" width={30}
                                                                                 height={30} alt="arrrow"/></a>
          <a href="#" onClick={() => moveDown()} className={styles.arrowUp}><Image src="/forms/arrowDown.svg" width={30}
                                                                                   height={30} alt="arrrow"/></a>
        </div>
        <div className={styles.buttons}>
          {
            item.item_question &&
              <>
                  <label>必須</label>
                  <input defaultChecked={item.item_question!.question!.required} type="checkbox"
                         onChange={(e) => handleToggleRequired(e.target.checked)} className={styles.checkBox}/>
              </>
          }
          <a href="#" onClick={() => delete_()} className={styles.deleteButtonWrapper}>
            <Image src="/forms/delete.svg" width={25} height={25} alt="delete"/>
          </a>
        </div>
      </div>
    </div>
  );
}