interface Props {
  onChange: (value: ("question_text" | "text" | "page_break")) => void,
  value: ("question_text" | "text" | "page_break")
}

export default function FormItemTypeSelect({onChange, value}: Props) {
  return (
    <select defaultValue={value}
            onChange={(e) => onChange(e.target.value as ("question_text" | "text" | "page_break"))}>
      <option value="question_text">短文</option>
      <option value="text">テキスト</option>
      <option value="page_break">改ページ</option>
    </select>
  );
}
