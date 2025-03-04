import styles from "./ParagraphInput.module.css";

type ParagraphInputProps = {
    fontSize?: number;
    width?: number;
    placeholder?: string;
    value?: string;
    onChange?: (value: string, value: string) => void;
    args?: string[];
};

const ParagraphInput = ({fontSize = 16, width = 0, placeholder = "回答を入力", value, onChange, args}: ParagraphInput) => {
  return (
    <textarea
        type="text"
        className={styles.Paragraph}
        value={value ?? "データなし"}
        placeholder={placeholder}
        style={{fontSize: fontSize, width: width === 0 ? "100%" : width}}
        onChange={(e) => onChange(e.target.value, args)}
      />
  );
};

export default ParagraphInput;
