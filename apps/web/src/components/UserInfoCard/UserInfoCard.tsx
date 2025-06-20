import {Exhibitor, User} from "@/lib";
import styles from "./UserInfoCard.module.css";

type UserInfoCardProps = {
    user: User;
    exhibitor: Exhibitor;
};

// 数字→漢数字のラベル変換マップ
const typeLabels: Record<string, string> = {
    1: "一",
    2: "二",
    3: "三",
};

export const UserInfoCard = ({user, exhibitor}: UserInfoCardProps) => {
    const representativeIndex = (user && exhibitor)
    ? exhibitor.representatives.indexOf(user.id) + 1
      : "?";
    return (
    <div className={styles.user}>
        <h1>こんにちは、{user?.last_name} {user?.first_name} 👋</h1>
        <h2>あなたは{exhibitor?.exhibitor_name}の第{typeLabels[representativeIndex] || representativeIndex}責任者です。</h2>
    </div>
    );
};

export default UserInfoCard;