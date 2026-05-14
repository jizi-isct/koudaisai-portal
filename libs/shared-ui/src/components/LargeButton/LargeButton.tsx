import styles from "./LargeButton.module.css";
import {ReactNode, useCallback} from "react";
import {useRouter} from "next/navigation";

type ButtonProps = {
  children: ReactNode
  type: "primary" | "secondary";
  onClick?: () => void;
  href?: string;
}

export const LargeButton = ({children, type, onClick, href}: ButtonProps) => {
  const router = useRouter()
  const handleClick = useCallback(() => {
    if (onClick) {
      onClick();
    }
    if (href) {
      router.push(href)
    }
  }, [onClick, href, router])
  return (
    <button
      className={`${styles.button} ${type === "primary" && styles.primary} ${type === "secondary" && styles.secondary}`}
      onClick={handleClick}>
      {children}
    </button>
  );
};
