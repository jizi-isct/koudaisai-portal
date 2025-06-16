import styles from "./ButtonCompact.module.css";

type ButtonCompactProps = { // ①
  text: string;
  color?: string;
  onClick: () => void;
  isClicked?: boolean;
};

export const ButtonCompact = ({text, color = '#0048FF', onClick, isClicked = false}: ButtonProps) => {
  return (
    <div
      className={styles.button}
      style={{
        backgroundColor: isClicked ? color : 'white',
        color: isClicked ? 'white' : 'black',
        borderColor: color
      }}
      onClick={onClick}>
      {text}
    </div>
  );
};
