import { Input } from "antd";
import type { InputProps } from "antd";
import styles from "./TextInput.module.css";

const { TextArea } = Input;

type TextInputProps = InputProps & {
  // 独自拡張したいProps
};

/**
 * テキスト入力
 * paragraphがtrueの場合textareaを表示し，falseの場合はinputを表示する
 * @param width 横幅
 * @param placeholder 未入力時に表示する文字列
 * @param value 値
 * @param setValue 値更新用の関数
 * @param paragraph trueの場合，textareaを表示する
 * @constructor
 */
export const TextInput = ({ ...props }) => {
  return <TextArea {...props} />;
};
