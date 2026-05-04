import styles from "./NextPhaseButton.module.css";
import {ButtonHTMLAttributes} from "react";

type NextPhaseButton = {
  label: string;
}

export function NextPhaseButton(props: NextPhaseButton & ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button className={styles.root} {...props}>
      <span className={styles.label}>{props.label}</span>
      <div className={styles.button}><RightArrow/></div>
    </button>
  );
}

function RightArrow() {
  return (
    <svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M6.66667 16H25.3333M25.3333 16L16 6.66666M25.3333 16L16 25.3333" stroke="#253661" strokeWidth="2"
            strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}