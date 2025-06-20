import styles from "./ExhibitorCard.module.css";
import {Exhibitor} from "@/lib";
import {ButtonCompact} from "@/components/generic/ButtonCompact/ButtonCompact";
import {ExhibitorIcon} from "@/components/exhibitor/ExhibitorIcon";


type ExhibitorCardProps = {
  exhibitor: Exhibitor;
  openModal: () => void;
};

// typeのラベル変換マップ
const typeLabels: Record<string, string> = {
  booth: "模擬店",
  stage: "ステージ",
  general: "一般企画",
  labo: "研究室企画",
};

export const ExhibitorCard = ({exhibitor, openModal}: ExhibitorCardProps) => {
  return (
    <div className={styles.card}>
      <ExhibitorIcon iconId={exhibitor.icon_id ?? ""}/>
      <h1>{exhibitor?.exhibition_name}</h1>
      <h4>{typeLabels[exhibitor?.type] || exhibitor?.type}</h4>
      <p>{exhibitor?.description}</p>
      <ButtonCompact
        text="編集する"
        color="#0048FF"
        onClick={() => openModal()}
        isClicked={false}
        className={styles.edit_button}
      />
    </div>
  );
};
