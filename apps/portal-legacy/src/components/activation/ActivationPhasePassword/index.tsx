import styles from "../ActivationPhase.module.css"
import {NextPhaseButton} from "@/components/activation/NextPhaseButton";
import {Input} from "@/components/common/Input";
import React, {useState} from "react";
import Logo from "@/components/Logo/Logo";
import {validatePassword} from "@koudaisai/shared-utils";

type ActivationPhasePasswordProps = {
  mAddress: string;
  next: (password: string) => void | Promise<void>;
}

export function ActivationPhasePassword({mAddress, next}: ActivationPhasePasswordProps) {
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const password = formData.get("password") as string;
    const passwordConfirm = formData.get("passwordConfirm") as string;

    if (password !== passwordConfirm) {
      setError("パスワードが一致しません。");
      return;
    }

    if (!validatePassword(password)) {
      setError("パスワードは8文字以上、英大文字、英小文字、数字、記号を含む必要があります。");
      return;
    }

    next(password)
  }
  return (
    <form className={styles.root} onSubmit={handleSubmit}>
      <div className={styles.logoSmall}>
        <Logo className={styles.item}/>
      </div>
      <h1 className={styles.item}>工大祭ポータルへようこそ</h1>
      <p className={styles.item} style={{margin: 0}}>ログインに使用するパスワードを決めましょう</p>
      {/*パスワードマネージャーにmアドレスを認識させるためのダミーインプット*/}
      <input
        name={"username"}
        type={"email"}
        value={mAddress}
        readOnly
        hidden
      />
      <Input
        placeholder={"パスワードを入力してください"}
        required
        name={"password"}
        type={"password"}
        className={styles.item}
      />
      <Input
        placeholder={"パスワードをもう一度入力してください"}
        required
        name={"passwordConfirm"}
        type={"password"}
        className={styles.item}
      />
      {error && <p className={styles.error}>{error}</p>}
      <div className={styles.item}>
        <NextPhaseButton type={"submit"} label={"次の画面へ進む"}/>
      </div>
    </form>
  )
}