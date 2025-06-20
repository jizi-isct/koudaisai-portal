import Image from "next/image";
import {useDownloadUrl} from "@/lib";
import styles from "./ExhibitorIcon.module.css"

type ExhibitorIconProps = {
  iconId: string
}

export function ExhibitorIcon({iconId}: ExhibitorIconProps) {
  const {downloadUrl} = useDownloadUrl(iconId, "image");
  if (downloadUrl) {
    return (
      <Image src={downloadUrl} alt="Exhibitor Image" width={100} height={100} className={styles.exhibitor_logo}/>
    )
  } else {
    return (
      <Image src={"/generic/no-image.png"} alt="Exhibitor Image" width={100} height={100}
             className={styles.exhibitor_logo}/>
    )
  }
}