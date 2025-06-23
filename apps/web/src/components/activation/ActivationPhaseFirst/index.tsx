import styles from "../ActivationPhase.module.css"
import {NextPhaseButton} from "@/components/activation/NextPhaseButton";
import Logo from "@/components/Logo/Logo";

type ActivationPhaseFirstProps = {
  next: () => void | Promise<void>;
}

export function ActivationPhaseFirst({next}: ActivationPhaseFirstProps) {
  return (
    <main className={styles.root}>
      <div className={styles.logoBig}>
        <Logo className={styles.item}/>
      </div>
      <h1 className={styles.item}>工大祭ポータルへようこそ</h1>
      <p className={styles.item} style={{margin: 0}}>アカウントを有効化しましょう</p>
      <div className={styles.item}><NextPhaseButton label={"次の画面へ進む"} onClick={next}/></div>
    </main>
  )
}