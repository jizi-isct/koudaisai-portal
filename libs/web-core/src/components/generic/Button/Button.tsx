import styles from "./Button.module.css";

type ButtonProps = { // ①
  text: string;
  color?: string;
  onClick: () => void;
  isClicked?: boolean;
};

const Button = ({text, color = '#0048FF', onClick, isClicked = false}: ButtonProps) => {
  return (
    <div
      className={styles.button}
      style={{
        backgroundColor: isClicked ? color : 'white',
        color: isClicked ? 'white' : 'black',
        borderColor: color
      }}
      onClick={() => onClick}>
      {text}
    </div>
  );
};

export default Button;
