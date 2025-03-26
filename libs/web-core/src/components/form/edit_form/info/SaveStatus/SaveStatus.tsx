import styles from "./SaveStatus.module.css";
import saved from "../../../../assets/images/save-status/saved.svg";
import unsaved from "../../../../assets/images/save-status/unsaved.svg"
import saving from "../../../../assets/images/save-status/saving.svg"
import {SaveStatus as SaveStatus_} from "../../../../../lib";

type SaveStatusProps = {
  status: SaveStatus_;
};

const SaveStatus = ({status}: SaveStatusProps) => {
  const statusIcons: Record<SaveStatus_, string> = {
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
      <img
        className={`${styles.statusIcon} ${status === "saving" ? styles.savingIcon : ""}`}
        src={statusIcons[status]}
        alt="status"
        width={25}
        height={25}
      />
      <h6 className={styles.statusText}>{statusText[status]}</h6>
    </div>
  );
};

export default SaveStatus;
