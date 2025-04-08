import {FormItemType} from "@koudaisai-portal/util";

interface Props {
  onChange: (value: FormItemType) => void,
  value: FormItemType
}

export function SelectFormItemType({onChange, value}: Props) {
  return (
    <select defaultValue={value}
            onChange={(e) => onChange(e.target.value as FormItemType)}>
      <option value="text">テキスト</option>
      <option value="page_break">改ページ</option>
      <option value="question_text">質問-短文</option>
      <option value="question_radio_button">質問-ラジオボタン</option>
      <option value="question_check_box">質問-チェックボックス</option>
    </select>
  );
}
