import { Button } from "antd";
import type { ButtonProps } from "antd";
import styles from "./ButtonCompact.module.css";

type ButtonCompactProps = { // ①
  text: string;
  color?: string;
  onClick: () => void;
  isClicked?: boolean;
  className?: string;
};

export const ButtonCompact = ({text, onClick, className}: ButtonCompactProps) => {
  return (
    <Button type="primary" className={`${styles.button} ${className}`} onClick={onClick}>{text}</Button>
  );
};
