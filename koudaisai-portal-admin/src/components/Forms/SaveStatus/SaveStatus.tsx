import Image from "next/image";
import styles from "./SaveStatus.module.css";

type SaveStatus = "saved" | "unsaved" | "saving";

type SaveStatusProps = {
  // ①
  status: SaveStatus;
};

const SaveStatus = ({ status }: SaveStatusProps) => {
  const statusIcons: Record<SaveStatus, string> = {
    saved: "/components/Forms/SaveStatus/saved.svg",
    unsaved: "/components/Forms/SaveStatus/unsaved.svg",
    saving: "/components/Forms/SaveStatus/saving.svg",
  };
  const statusText: Record<SaveStatus, string> = {
    saved: "変更内容を保存しました",
    unsaved: "変更内容は保存されていません",
    saving: "変更内容を保存中",
  };

  return (
    <div className={styles.saveStatus}>
      <Image
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
