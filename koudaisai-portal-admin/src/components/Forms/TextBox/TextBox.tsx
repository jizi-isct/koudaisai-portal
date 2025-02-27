import styles from "./TextBox.module.css";

type TextBoxProps = {
    fontSize?: number;
    width: number;
    placeholder?: string;
};

const TextBox = ({fontSize = 16, width, placeholder = "回答を入力"}: TextBoxProps) => {
  return (
    <input 
        type="text" 
        className={styles.textBox}
        placeholder={placeholder}
        style={{}} />
  );
};

export default TextBox;
