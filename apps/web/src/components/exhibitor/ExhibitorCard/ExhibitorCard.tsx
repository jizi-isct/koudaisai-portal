import styles from "./ExhibitorCard.module.css";
import Image from "next/image";
import {Exhibitor, useDownloadUrl} from "@/lib";
import {ButtonCompact} from "@/components/generic/ButtonCompact/ButtonCompact";
import {useEffect, useState} from "react";


type ExhibitorCardProps = {
    exhibitor: Exhibitor | null;
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
    const {downloadUrl, error} = useDownloadUrl(exhibitor?.icon_id, "filename");
    const fallbackImage = "/generic/no-image.png";
    const imageSrc = downloadUrl || fallbackImage;
    return (
        <div className={styles.card}>
            <Image src={imageSrc} alt="Exhibitor Image" width={100} height={100} className={styles.exhibitor_logo} />
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
