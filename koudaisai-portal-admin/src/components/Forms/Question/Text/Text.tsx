import styles from "./Text.module.css";
import TextBox from "@/components/Forms/TextInput/TextInput";


const Text = () => {
  return (
    <TextBox fontSize={18} width={150} placeholder="回答を入力"/>
  );
};

export default Text;
