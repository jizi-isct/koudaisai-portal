import Image from "next/image";
import styles from "./Logo.module.css";

type LogoProps = { // ①
    width?: number;
};

const Logo = ({width = 40}: LogoProps) => {
  return (
    <div className={styles.logo}>
        <Image
            src="/components/Logo/logo_tmp.svg"
            alt="Koudaisai Portal Logo"
            width={width}
            height={width}
        />
    </div>

  );
};

export default Logo;
