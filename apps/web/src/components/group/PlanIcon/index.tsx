import Image from "next/image";
import styles from "./PlanIcon.module.css"

type ExhibitorIconProps = {
  planId: string
}

export function PlanIcon({planId}: ExhibitorIconProps) {
  return (
    <Image
      src={`https://api2025.jizi.jp/cdn-cgi/images/format=webp,quality=80,height=256,width=256/v1/plans/${planId}/icon`}
      onError={(e) => {
        const target = e.currentTarget;
        target.onerror = null; // 無限ループ防止
        target.src = "/generic/no-image.png";
      }}
      alt={`${planId}`}
      width={100}
      height={100}
      className={styles.exhibitor_logo}/>
  )
}