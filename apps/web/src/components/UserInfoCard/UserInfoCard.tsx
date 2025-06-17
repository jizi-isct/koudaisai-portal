import { User, Exhibitor } from "@/lib";
import styles from "./UserInfoCard.module.css";

type UserInfoCardProps = {
    user: User | null;
    exhibitor: Exhibitor | null;
};

export const UserInfoCard = ({user, exhibitor}: UserInfoCardProps) => {
    const representativeIndex = (user && exhibitor)
    ? exhibitor.representatives.indexOf(user.id) + 1
    : null;
    return (
    <div className={styles.user}>
        <h1>こんにちは、{user?.last_name} {user?.first_name} 👋</h1>
        <h2>あなたは{exhibitor?.exhibitor_name}の第{representativeIndex}責任者です。</h2>
    </div>
    );
};

export default UserInfoCard;