import styles from "./ExhibitorCard.module.css";
import {Exhibitor} from "@/lib";

type ExhibitorCardProps = {
    exhibitor: Exhibitor;
    setExhibitor: (exhibitor: Exhibitor) => void;
};

export const ExhibitorCard = ({exhibitor, setExhibitor}: ExhibitorCardProps) => {
    return (
        <div className={styles.card}>
            {exhibitor?.exhibition_name}
            {exhibitor?.description}
            {exhibitor?.type}
        </div>
    );
};
