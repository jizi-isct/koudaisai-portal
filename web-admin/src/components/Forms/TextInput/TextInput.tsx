import styles from "./TextInput.module.css";

type TextInputProps = {
    fontSize?: number;
    width?: number;
    placeholder?: string;
    value?: string;
    onChange?: (value: string, args: string[] | undefined) => void;
    args?: string[];
};

const TextInput = ({fontSize = 16, width = 0, placeholder = "回答を入力", value, onChange, args}: TextInputProps) => {
    return (
        <input
            type="text"
            className={styles.textBox}
            value={value ?? "データなし"}
            placeholder={placeholder}
            style={{fontSize: fontSize, width: width === 0 ? "100%" : width}}
            onChange={(e) => (onChange ?? ((value: string, args: string[] | undefined) => {
            }))(e.target.value, args)}
        />
    );
};

export default TextInput;
