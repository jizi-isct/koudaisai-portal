import styles from "./Header.module.css";
import Logo from "@/components/Logo/Logo";

const Header = () => {
  return (
    <header className={styles.header}>
      <div className={styles.logoWrapper}>
        <Logo />
        <h1 className={styles.title}>工大祭ポータル</h1>
      </div>
      <div className={styles.menuWrapper}>
        
      </div>
    </header>
  );
};

export default Header;
