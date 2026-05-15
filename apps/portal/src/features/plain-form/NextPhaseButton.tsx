import type {ButtonHTMLAttributes} from "react";
import styles from "./PlainForm.module.css";

type Props = {
  label: string;
} & ButtonHTMLAttributes<HTMLButtonElement>;

export function NextPhaseButton({label, ...props}: Props) {
  return (
    <button className={styles.button} {...props}>
      <span className={styles.buttonLabel}>{label}</span>
      <span className={styles.buttonIcon} aria-hidden="true">
        <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            d="M6.66667 16H25.3333M25.3333 16L16 6.66666M25.3333 16L16 25.3333"
            stroke="#253661"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        </svg>
      </span>
    </button>
  );
}
