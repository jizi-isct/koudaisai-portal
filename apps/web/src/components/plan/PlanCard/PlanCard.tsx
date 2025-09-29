import styles from "./PlanCard.module.css";
import {Button, Tag, Tooltip} from "antd";
import {PlanIcon} from "@/components/plan/PlanIcon";
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
      <h1>
        {plan.plan_name}
        {
          plan.is_recommended && <Tooltip title={"おすすめ企画"}><Tag color={"gold"}>⭐️</Tag></Tooltip>
        }
        {
          plan.is_child_friendly && <Tooltip title={"子供向け企画"}><Tag color={"cyan"}>👦</Tag></Tooltip>
        }
      </h1>
      <h4>{typeLabels[plan.type] || plan.type}</h4>
      <p>{plan.description}</p>
      <Button
        type="primary"
        style={{ alignSelf: "flex-start" }}
        onClick={() => openModal()}
        className={styles.edit_button}
      >訂正する</Button>
    </div>
  );
};
