import Image from "next/image";
import styles from "./Logo.module.css";

type LogoProps = { // ①
    height?: number;
    hasText?: boolean;
  className?: string;
};

const Logo = ({height = 40, hasText = false, className = ""}: LogoProps) => {
    return (
      <div className={styles.logo + " " + className}>
            <Image
              src="/members_logo.png"
                alt="Koudaisai Portal Logo"
                width={height}
                height={height}
              className={styles.image}
            />
        <div className={styles.logoTextWrapper} style={{height: height, display: hasText ? 'inline-block' : 'none'}}>
                <h1
                    className={styles.logoText}
                    style={{marginTop: height / 2, display: hasText ? 'inline-block' : 'none', fontSize: height * 0.4}}
                >工大祭ポータル</h1>
            </div>

        </div>

    );
};

export default Logo;
