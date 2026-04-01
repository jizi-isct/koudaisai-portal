"use client";

import styles from "../ActivationPhase.module.css"
import {NextPhaseButton} from "@/components/activation/NextPhaseButton";
import Logo from "@/components/Logo/Logo";
import {login} from "@/lib";
import {useState} from "react";
import {useRouter} from "next/navigation";

type ActivationPhaseDoneProps = {
  m_address: string;
  password: string;
}

export function ActivationPhaseDone({m_address, password}: ActivationPhaseDoneProps) {
  const [error, setError] = useState<string | null>(null);
  const router = useRouter()

  const handleClick = async () => {
    try {
      await login(m_address, password)
    } catch (e) {
      setError(`${e}`)
      return
    }
    router.push("/")
  }

  return (
    <main className={styles.root}>
      <div className={styles.logoBig}>
        <Logo className={styles.item}/>
      </div>
      <p className={styles.item} style={{margin: 0}}>
        アカウントは有効化されました👍<br/>
        我々と一緒に工大祭を盛り上げましょう！
      </p>
      <div className={styles.item}><NextPhaseButton label={"工大祭ポータルを開く"} onClick={handleClick}/></div>
      {error && <p className={styles.error}>{error}</p>}
    </main>
  )
}