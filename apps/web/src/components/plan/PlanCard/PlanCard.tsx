import styles from "./PlanCard.module.css";
import {Button} from "antd";
import {PlanIcon} from "@/components/plan/PlanIcon";
import {BasePlanRead} from "@/lib/plansInfoTypes";

type ExhibitorCardProps = {
  plan: BasePlanRead;
  openModal: () => void;
  disableEdit: boolean;
};

// typeのラベル変換マップ
const typeLabels: Record<string, string> = {
  booth: "模擬店",
  stage: "ステージ",
  general: "一般企画",
  labo: "研究室企画",
};

export const PlanCard = ({plan, openModal, disableEdit}: ExhibitorCardProps) => {
  return (
    <div className={styles.card}>
      <PlanIcon planId={plan.id}/>
      <h1>{plan.plan_name}</h1>
      <h4>{typeLabels[plan.type] || plan.type}</h4>
      <p>{plan.description}</p>
      <Button
        type="primary"
        style={{ alignSelf: "flex-start" }}
        onClick={() => openModal()}
        className={styles.edit_button}
        disabled={disableEdit}
      >訂正する</Button>
    </div>
  );
};
