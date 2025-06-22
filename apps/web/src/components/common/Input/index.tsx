import styles from "./Input.module.css"
import {InputHTMLAttributes} from "react";

export function Input(props: InputHTMLAttributes<HTMLInputElement>) {
  return (
    <input
      {...props}
      className={styles.button}
    />
  )
}