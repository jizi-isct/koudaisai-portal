import styles from "./TextInput.module.css";

type TextInputProps = {
    fontSize?: number;
    width?: number;
    placeholder?: string;
    value?: string;
    onChange: (value: string) => void;
};

const TextInput = ({fontSize = 16, width = 0, placeholder = "回答を入力", value, onChange}: TextInputProps) => {
    return (
        <input
            type="text"
            className={styles.textBox}
            defaultValue={value}
            placeholder={placeholder}
            style={{fontSize: fontSize, width: width === 0 ? "100%" : width}}
            onChange={(e) => onChange(e.target.value)}
        />
    );
};

export default TextInput;
