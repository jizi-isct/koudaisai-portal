import styles from "./TextInput.module.css";

type TextInputProps = {
  width?: number;
  placeholder?: string;
  value?: string;
  setValue: (newValue: string) => void;
  paragraph: boolean;
};

/**
 * テキスト入力
 * paragraphがtrueの場合textareaを表示し，falseの場合はinputを表示する
 * @param width 横幅
 * @param placeholder 未入力時に表示する文字列
 * @param value 値
 * @param setValue 値更新用の関数
 * @param paragraph trueの場合，textareaを表示する
 * @constructor
 */
const TextInput = ({width, placeholder = "テキストを入力", value, setValue, paragraph}: TextInputProps) => {
  if (paragraph) {
    return (
      <textarea
        className={styles.Paragraph}
        value={value ?? placeholder}
        placeholder={placeholder}
        style={{fontSize: 12, width: width === 0 ? "100%" : width}}
        onChange={(e) => (setValue(e.target.value))}
      />
    )
  } else {
    return (
      <input
        type="text"
        className={styles.textBox}
        value={value ?? "データなし"}
        placeholder={placeholder}
        style={{fontSize: 16, width: width ? width : "100%"}}
        onChange={(e) => (setValue(e.target.value))}
      />
    );
  }
};

export default TextInput;
