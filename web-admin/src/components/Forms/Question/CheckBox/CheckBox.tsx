import styles from "./CheckBox.module.css";
import Image from "next/image";
import TextBox from "@/components/Forms/TextInput/TextInput";


const CheckBox = () => {
    return (
        <>
            <div className={styles.checkbox}>
                <input type="checkbox"/>
                <label htmlFor="checkbox">
                    <TextBox fontSize={16} width={150} placeholder="選択肢"/>
                </label>
                <Image src="/forms/close.svg" width={30} height={30} alt="close" className={styles.closeButton}/>
            </div>
            <a href="#">選択肢を追加</a>
        </>
    );
};

export default CheckBox;
