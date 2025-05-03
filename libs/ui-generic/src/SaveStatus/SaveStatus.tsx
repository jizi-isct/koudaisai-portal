import styles from "./SaveStatus.module.css";
import {saved, saving, unsaved} from "../icons/save_status";
import {SaveStatus as SaveStatus_} from "@koudaisai-portal/util";
import Element = React.JSX.Element;

type SaveStatusProps = {
  status: SaveStatus_;
};

export const SaveStatus = ({status}: SaveStatusProps) => {
  const statusIcons: Record<SaveStatus_, Element> = {
    saved: saved,
    unsaved: unsaved,
    saving: saving,
  };
  const statusText: Record<SaveStatus_, string> = {
    saved: "変更内容を保存しました",
    unsaved: "変更内容は保存されていません",
    saving: "変更内容を保存中",
  };

  return (
    <div className={styles.saveStatus}>
      {statusIcons[status]}
      <h6 className={styles.statusText}>{statusText[status]}</h6>
    </div>
  );
};
