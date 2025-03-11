import styles from "./ParagraphInput.module.css";

type ParagraphInputProps = {
  fontSize?: number;
  width?: number;
  placeholder?: string;
  value?: string;
  onChange?: (value: string) => void;
  args?: string[];
};

const ParagraphInput = ({
                          fontSize = 16, width = 0, placeholder = "回答を入力", value, onChange = () => {
  }
                        }: ParagraphInputProps) => {
  return (
    <textarea
      className={styles.Paragraph}
      value={value ?? placeholder}
      placeholder={placeholder}
      style={{fontSize: fontSize, width: width === 0 ? "100%" : width}}
      onChange={(e) => onChange(e.target.value)}
    />
  );
};

export default ParagraphInput;
