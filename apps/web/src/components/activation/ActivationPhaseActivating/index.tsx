import styles from "../ActivationPhase.module.css"
import {Loader} from "@/components/generic/Loader";
import Logo from "@/components/Logo/Logo";
import {useEffect, useRef, useState} from "react";
import {fetchClientAuth} from "@/lib";

type ActivationPhaseActivatingProps = {
  mAddress: string
  password: string
  token: string
  next: () => void | Promise<void>
}

export function ActivationPhaseActivating({mAddress, password, token, next}: ActivationPhaseActivatingProps) {
  const [error, setError] = useState<string | undefined>();
  // アカウント有効化の二重リクエスト防止用
  const hasActivated = useRef<boolean>(false)
  useEffect(() => {
    if (hasActivated.current) return
    hasActivated.current = true; // 二重リクエスト防止
    (async () => {
      const {response} = await fetchClientAuth.POST("/activate", {
        body: {
          m_address: mAddress,
          token: token,
          password: password
        }
      })

      if (response.ok) {
        next();
      } else {
        switch (response.status) {
          case 401:
            setError("有効化トークンあるいはmアドレスが無効です。もう一度やり直してください。");
            break;
          case 404:
            setError("指定されたmアドレスが見つかりません。");
            break;
          case 409:
            setError("指定されたmアドレスはすでに登録されています。");
            break;
          case 429:
            setError("リクエストが多すぎます。しばらく待ってから再度お試しください。");
            break;
        }
      }
    })()
  }, [mAddress, next, password, token]);

  if (error) {
    return (
      <main className={styles.root}>
        <div className={styles.logoBig}>
          <Logo className={styles.item}/>
        </div>
        <p className={styles.item} style={{color: "red"}}>アカウントの有効化に失敗しました</p>
        <p className={styles.item}>{error}</p>
      </main>
    )
  }
  return (
    <main className={styles.root}>
      <div className={styles.logoBig}>
        <Logo className={styles.item}/>
      </div>
      <p className={styles.item}>アカウントを有効化中</p>
      <Loader/>
    </main>
  )
}