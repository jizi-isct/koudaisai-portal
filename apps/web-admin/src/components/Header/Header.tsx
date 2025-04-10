import styles from "./Header.module.css";
import Link from "next/link";
import Logo from "@/components/Logo/Logo";
import HeaderButton from "@/components/Button/HeaderButton/HeaderButton";

type HeaderProps = { // ①
    page: string;
};

const Header = () => {
    const buttonStyle ={
        backgroundColor: 'white',
        color: 'black',
        borderColor: '#0048FF'
    }
    return (
        <header className={styles.header}>
            <div className={styles.logoWrapper}>
                <Logo height={50} hasText/>
            </div>
            <div className={styles.menuWrapper}>
                <Link href="/"><div className={styles.headerButton} style={buttonStyle}>ホーム</div></Link>
                <HeaderButton
                    text="フォーム"
                    onClick="/forms"
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
