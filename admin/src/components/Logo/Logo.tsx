import Image from "next/image";
import styles from "./Logo.module.css";

type LogoProps = { // ①
  height?: number;
  hasText?: boolean;
};

const Logo = ({height = 40, hasText = false}: LogoProps) => {
  return (
    <div className={styles.logo}>
        <Image
          src="/components/Logo/logo.jpg"
          alt="Koudaisai Portal Admin Site Logo"
            width={height}
            height={height}
        />
        <div className={styles.logoTextWrapper} style={{height: height}}>
          <h1 
            className={styles.logoText}
            style={{marginTop: height / 2  ,display: hasText ? 'inline-block' : 'none', fontSize: height * 0.4}}
          >工大祭ポータル管理サイト</h1>
        </div>
        
    </div>

  );
};

export default Logo;
