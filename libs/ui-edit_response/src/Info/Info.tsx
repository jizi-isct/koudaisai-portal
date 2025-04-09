import styles from "./Info.module.css";
import {Info as Info_} from "@koudaisai-portal/util";

type Props = {
  info: Info_
}

export function Info({info}: Props) {
  return (
    <div className={styles.root}>
      <h1>{info.title}</h1>
      <p>{info.description}</p>
    </div>
  )
}