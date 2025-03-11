import styles from "./Header.module.css";
import Logo from "@/components/Logo/Logo";
import HeaderButton from "@/components/Button/HeaderButton/HeaderButton";

const Header = () => {
  return (
    <header className={styles.header}>
      <div className={styles.logoWrapper}>
        <Logo height={50} hasText />
      </div>
      <div className={styles.menuWrapper}>
        <HeaderButton
          text="ホーム"
          onClick="/"
          isClicked={true}
        />
        <HeaderButton 
          text="フォーム"
          onClick="/admin/forms"
        />
        <HeaderButton 
          text="資料"
          onClick="/" 
        />
        <HeaderButton 
          text="よくある質問"
          onClick="/questions" 
        />
      </div>
    </header>
  );
};

export default Header;
