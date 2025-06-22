import {NextPhaseButton} from "@/components/activation/NextPhaseButton";
import {Input} from "@/components/common/Input";
import styles from "../ActivationPhase.module.css"
import Logo from "@/components/Logo/Logo";

type ActivationPhaseMAddressProps = {
  next: (mAddress: string) => void | Promise<void>;
}

export function ActivationPhaseMAddress({next}: ActivationPhaseMAddressProps) {
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const mAddress = formData.get("mAddress") as string;
    console.log(mAddress);
    next(mAddress);
  }
  return (
    <form className={styles.root} onSubmit={handleSubmit}>
      <Logo className={styles.item}/>
      <h1 className={styles.item}>工大祭ポータルへようこそ</h1>
      <p className={styles.item} style={{margin: 0}}>参加申請フォームに入力したmアドレスを入力してください。</p>
      <div className={styles.item}>
        <Input
          placeholder={"mアドレスを入力してください"}
          required
          name={"mAddress"}
          type={"email"}
        />
      </div>
      <div className={styles.item}>
        <NextPhaseButton type={"submit"} label={"次の画面へ進む"}/>
      </div>
    </form>
  )
}