import styles from "./Item.module.css";
import {TextInput} from "@koudaisai-portal/ui-generic";
import {FormItemType, Item as Item_} from "@koudaisai-portal/util"
import {SelectFormItemType} from "../SelectFormItemType";
import {arrowDown, arrowUp, delete_ as delete__} from "../icons/form_edit";

type Props = {
  readonly item: Item_;
  setItem: (item: Item_) => void;
  moveUp: () => void;
  moveDown: () => void;
  delete_: () => void;
};

export function Item({item, setItem, moveUp, moveDown, delete_}: Props) {
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

  const handleFormItemTypeChange = (value: FormItemType) => {
    switch (value) {
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
      case "question_text":
        setItem(
          {
            ...item,
            item_question: {
              question: {
                required: true,
                question_text: {
                  paragraph: false
                },
                question_radio_button: undefined,
                question_check_box: undefined,
              }
            },
            item_page_break: undefined,
            item_text: undefined,
          }
        )
        break;
      case "question_radio_button":
        setItem(
          {
            ...item,
            item_question: {
              question: {
                required: true,
                question_text: undefined,
                question_radio_button: {
                  choices: []
                },
                question_check_box: undefined,
              }
            },
            item_page_break: undefined,
            item_text: undefined,
          }
        )
        break;
      case "question_check_box":
        setItem(
          {
            ...item,
            item_question: {
              question: {
                required: true,
                question_text: undefined,
                question_radio_button: undefined,
                question_check_box: {
                  choices: []
                },
              }
            },
            item_page_break: undefined,
            item_text: undefined,
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
            ...item.item_question?.question,
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

  const handleDeleteRadioButtonChoice = (index: number) => {
    const choices = structuredClone(item.item_question!.question!.question_radio_button!.choices)
    setItem(
      {
        ...item,
        item_question: {
          question: {
            ...item.item_question!.question,
            question_radio_button: {
              choices: choices.toSpliced(index, 1)
            }
          }
        }
      }
    )
  }

  const handleAddRadioButtonChoice = () => {
    const choices = structuredClone(item.item_question!.question!.question_radio_button!.choices)
    choices.push("選択肢" + choices.length)
    setItem(
      {
        ...item,
        item_question: {
          question: {
            ...item.item_question!.question,
            question_radio_button: {
              choices: choices
            }
          }
        }
      }
    )
  }

  const handleDeleteCheckBoxChoice = (index: number) => {
    const choices = structuredClone(item.item_question!.question!.question_check_box!.choices)
    setItem(
      {
        ...item,
        item_question: {
          question: {
            ...item.item_question!.question,
            question_check_box: {
              choices: choices.toSpliced(index, 1)
            }
          }
        }
      }
    )
  }

  const handleAddCheckBoxChoice = () => {
    const choices = structuredClone(item.item_question!.question!.question_check_box!.choices)
    choices.push("選択肢" + choices.length)
    setItem(
      {
        ...item,
        item_question: {
          question: {
            ...item.item_question!.question,
            question_check_box: {
              choices: choices
            }
          }
        }
      }
    )
  }

  const getItemType = () => {
    if (item.item_text) {
      return "text"
    }
    if (item.item_page_break) {
      return "page_break"
    }
    if (item.item_question?.question?.question_text) {
      return "question_text"
    }
    if (item.item_question?.question?.question_radio_button) {
      return "question_radio_button"
    }
    if (item.item_question?.question?.question_check_box) {
      return "question_check_box"
    }
    throw Error("Illegal item type")
  }

  return (
    <div className={styles.formWrapper}>
      <div className={styles.questionTitleWrapper}>
        <div className={styles.questionTitle}>
          <TextInput
            width={400}
            placeholder="タイトルを入力"
            value={item.title}
            setValue={handleTitleChange}
            paragraph={false}
          />
          <SelectFormItemType onChange={handleFormItemTypeChange} value={getItemType()}/>
        </div>
        <div className={styles.questionDescription}>
          <TextInput
            width={800}
            placeholder="説明を入力"
            value={item.description}
            setValue={handleDescriptionChange}
            paragraph={true}
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
        {
          item.item_question?.question?.question_radio_button &&
                <ul>
                  {
                    item.item_question!.question!.question_radio_button!.choices.map((choice: string, i: number) => {
                      return <li key={i}>{choice}
                        <button onClick={() => handleDeleteRadioButtonChoice(i)}>削除</button>
                      </li>
                    })
                  }
                  <li>
                    <button onClick={() => handleAddRadioButtonChoice()}>追加</button>
                  </li>
                </ul>
        }
        {
          item.item_question?.question?.question_check_box &&
                <ul>
                  {
                    item.item_question!.question!.question_check_box!.choices.map((choice: string, i: number) => {
                      return <li key={i}>{choice}
                        <button onClick={() => handleDeleteCheckBoxChoice(i)}>削除</button>
                      </li>
                    })
                  }
                  <li>
                    <button onClick={() => handleAddCheckBoxChoice()}>追加</button>
                  </li>
                </ul>
        }
      </div>
      <div className={styles.buttonsWrapper}>
        <div className={styles.arrowsWrapper}>
          <a href="#" onClick={() => moveUp()} className={styles.arrowUp}>
            {arrowUp}
          </a>
          <a href="#" onClick={() => moveDown()} className={styles.arrowUp}>
            {arrowDown}
          </a>
        </div>
        <div className={styles.buttons}>
          {
            item.item_question &&
                  <>
                    <label>必須</label>
                    <input defaultChecked={item.item_question!.question!.required} type="checkbox"
                           onChange={(e) => handleToggleRequired(e.target.checked)}
                           className={styles.checkBox}/>
                  </>
          }
          <a href="#" onClick={() => delete_()} className={styles.deleteButtonWrapper}>
            {delete__}
          </a>
        </div>
      </div>
    </div>
  );
}
