import styles from "./ButtonCompact.module.css";

type ButtonCompactProps = { // ①
  text: string;
  color?: string;
  onClick: () => void;
  isClicked?: boolean;
  className?: string;
};

export const ButtonCompact = ({text, color = '#0048FF', onClick, isClicked = false, className}: ButtonProps) => {
  return (
    <div
      className={`${styles.button} ${className}`}
      
      onClick={onClick}>
      {text}
    </div>
  );
};
