import styles from './LargeButton.module.css';
import { ReactNode, useCallback } from 'react';

type ButtonProps = {
  children: ReactNode;
  type: 'primary' | 'secondary';
  onClick?: () => void;
  href?: string;
};

export const LargeButton = ({ children, type, onClick, href }: ButtonProps) => {
  const handleClick = useCallback(() => {
    if (onClick) {
      onClick();
    }
    if (href) {
      window.location.assign(href);
    }
  }, [onClick, href]);
  return (
    <button
      className={`${styles.button} ${type === 'primary' && styles.primary} ${type === 'secondary' && styles.secondary}`}
      onClick={handleClick}
    >
      {children}
    </button>
  );
};
