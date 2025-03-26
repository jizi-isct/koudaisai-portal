import styles from "./Item.module.css";
import {Item as Item_} from "../../../../lib/types";
import TextInput from "../../../generic/TextInput/TextInput";
import {useState} from "react";

type Props = {
  readonly item: Item_,
  setValue: (value: string) => void
};

//Itemと衝突するため関数名を変更している
//TODO: 突貫実装のためコンポーネントを分割するべし
export default function Item({item, setValue}: Props) {
  const [checkedChoices, setCheckedChoices] = useState(new Set<number>());
  const handleRadioButtonSelect = (choice: string) => () => {
    setValue(choice);
  }
  const handleCheckBoxChange = (index: number) => (checked: boolean) => {
    if (checked) {
      setCheckedChoices(checkedChoices.add(index))
    } else {
      checkedChoices.delete(index);
      setCheckedChoices(checkedChoices)
    }
    let value = "";
    for (const i of checkedChoices) {
      value += item!.item_question!.question!.question_check_box!.choices[i] + ","
    }
    setValue(value.substring(0, value.length - 1))
  }

  return (
    <div className={styles.questionWrapper}>
      <h2 className={styles.title}>{item.title}</h2>
      <p className={styles.description}>{item.description}</p>
      {item?.item_question?.question.required && <p className={styles.required}>必須</p>}
      {
        item.item_question?.question?.question_text &&
        (item.item_question!.question!.question_text!.paragraph ?
          <TextInput
            placeholder="長文を入力"
            setValue={setValue}
            paragraph={true}
          />
          :
          <TextInput
            placeholder="短文を入力"
            setValue={setValue}
            paragraph={false}
          />)
      }
      {
        item.item_question?.question?.question_radio_button &&
        item.item_question!.question!.question_radio_button!.choices.map((choice, i) => {
          return <><input key={i} type="radio" name={item.item_id} value={choice}
                          onSelect={handleRadioButtonSelect(choice)}/> {choice} <br key={-i - 1}/></>
        })
      }
      {
        item.item_question?.question?.question_check_box &&
        item.item_question!.question!.question_check_box!.choices.map((choice, i) => {
          return <><input key={i} type="checkbox" name={item.item_id} value={i}
                          onChange={(e) => handleCheckBoxChange(i)(e.target.checked)}/> {choice} <br
            key={-i - 1}/></>
        })
      }
    </div>
  );
};
