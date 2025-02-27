import Link from "next/link";
import styles from "./HeaderButton.module.css";

type HeaderButtonProps = { // ①
    text: string;
    color?: string;
    onClick: string;
    isClicked?: boolean;
};

const HeaderButton = ({ text, color = '#0048FF', onClick, isClicked = false }: HeaderButtonProps) => {
  return (
    <Link href={onClick}>
        <div 
            className={styles.headerButton}
            style={{backgroundColor: isClicked ? color : 'white', color: isClicked ? 'white' : 'black', borderColor: color}}>
            {text}
        </div>
    </Link>
  );
};

export default HeaderButton;
