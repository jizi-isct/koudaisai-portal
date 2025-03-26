import {Form, FormResponse, Item} from "../../../../lib/types";
import {default as ItemComponent} from "../../../../components/form/edit_response/Item";
import Info from "../../../../components/form/edit_response/Info";

type Props = {
  readonly form: Form,
  formResponse: FormResponse,
  setFormResponse: (newFormResponse: FormResponse) => void,
}

/**
 * フォーム回答の編集画面
 * @param form フォーム
 * @param formResponse フォーム回答
 * @param setFormResponse フォーム回答更新用の関数
 * @constructor
 */
export default function EditResponse({form, formResponse, setFormResponse}: Props) {
  const handleInputChange = (item: Item) => (value: string) => {
    setFormResponse({
      ...formResponse,
      answers: {
        ...formResponse.answers,
        [item.item_id]: {
          item_id: item.item_id,
          answer_text: {
            value: value
          }
        }
      }
    });
  };

  return (
    <div>
      <Info info={form.info}/>
      {
        form.items.map((item: Item) => (
          <ItemComponent
            key={item.item_id}
            item={item}
            setValue={handleInputChange(item)}
          />
        ))
      }
    </div>
  )
}