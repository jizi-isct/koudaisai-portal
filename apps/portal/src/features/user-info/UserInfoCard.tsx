import type {GroupRead, UserRead} from "@koudaisai/shared-types";
import {getRepresentativeIndex} from "@koudaisai/shared-utils";
import styles from "./UserInfoCard.module.css";

type Props = {
  user: UserRead;
  group: GroupRead;
};

export function UserInfoCard({user, group}: Props) {
  const representativeIndex = getRepresentativeIndex(user, group);

  return (
    <div className={styles.user}>
      <h1>こんにちは、{user.name} 👋</h1>
      {group.type_plan?.type_booth && (
        <h2>
          あなたは{group.name}<small>({group.id})</small>の第{representativeIndex}責任者です。
        </h2>
      )}
      {group.type_plan?.type_general && (
        <h2>
          あなたは{group.name}<small>({group.id})</small>の第{representativeIndex}責任者です。
        </h2>
      )}
      {group.type_plan?.type_stage && (
        <h2>
          あなたは{group.name}<small>({group.id})</small>の第{representativeIndex}責任者です。
        </h2>
      )}
      {group.type_plan?.type_labo && (
        <h2>
          あなたは{group.name}<small>({group.id})</small>の企画実施担当者です。
        </h2>
      )}
      {group.type_press && (
        <h2>
          あなたは{group.name}<small>({group.id})</small>の代表者です。
        </h2>
      )}
    </div>
  );
}
