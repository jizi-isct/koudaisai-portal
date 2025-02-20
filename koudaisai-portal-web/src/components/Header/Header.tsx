import styles from "./Header.module.css";
import Logo from "@/components/Logo/Logo";
import HeaderButton from "@/components/Button/HeaderButton/HeaderButton";

const Header = () => {
  return (
    <header className={styles.header}>
      <div className={styles.logoWrapper}>
        <Logo />
        <h1 className={styles.title}>工大祭ポータル</h1>
      </div>
      <div className={styles.menuWrapper}>
        <HeaderButton 
          text="ホーム"
          onClick="/" 
        />
        <HeaderButton 
          text="フォーム"
          onClick="/" 
        />
        <HeaderButton 
          text="資料資料"
          onClick="/" 
        />
        <HeaderButton 
          text="よくある質問"
          onClick="/" 
        />
      </div>
    </header>
  );
};

export default Header;
