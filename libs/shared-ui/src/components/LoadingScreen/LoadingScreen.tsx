import styles from "./LoadingScreen.module.css"
import {Loader} from "@/components/generic/Loader";

export function LoadingScreen() {
  return (
    <div className={styles.root}>
      <Loader/>
      <div>LOADING</div>
    </div>
  )
}