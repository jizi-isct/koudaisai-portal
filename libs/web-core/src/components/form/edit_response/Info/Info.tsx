import styles from "./Info.module.css";
import {Info as Info_} from "../../../../lib/types";

type Props = {
  info: Info_
}
export default function Info({info}: Props) {
  return (
    <div className={styles.formTitleWrapper}>
      <h1>{info.title}</h1>
      <p>{info.description}</p>
    </div>
  )
}