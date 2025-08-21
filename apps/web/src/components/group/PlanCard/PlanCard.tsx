import styles from "./PlanCard.module.css";
import {ButtonCompact} from "@/components/generic/ButtonCompact/ButtonCompact";
import {PlanIcon} from "@/components/group/PlanIcon";
import {BasePlanRead} from "@/lib/plansInfoTypes";


type ExhibitorCardProps = {
  plan: BasePlanRead;
  openModal: () => void;
};

// typeのラベル変換マップ
const typeLabels: Record<string, string> = {
  booth: "模擬店",
  stage: "ステージ",
  general: "一般企画",
  labo: "研究室企画",
};

export const PlanCard = ({plan, openModal}: ExhibitorCardProps) => {
  return (
    <div className={styles.card}>
      <PlanIcon planId={plan.id}/>
      <h1>{plan.plan_name}</h1>
      <h4>{typeLabels[plan.type] || plan.type}</h4>
      <p>{plan.description}</p>
      <ButtonCompact
        text="訂正する"
        color="#0048FF"
        onClick={() => openModal()}
        isClicked={false}
        className={styles.edit_button}
      />
    </div>
  );
};
