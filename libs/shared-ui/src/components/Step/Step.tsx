import { ReactNode } from 'react';
import styles from './Step.module.css';

type Props = {
  title: string;
  date?: string;
  children: ReactNode;
  isFirst: boolean;
};

export function Step({ title, date, children, isFirst }: Props) {
  return (
    <div
      className={`${styles.root} ${isFirst ? styles.rootFirst : styles.rootAfterSecond}`}
    >
      {date && <div className={styles.date}>🕓 {date}</div>}
      <h2 className={styles.heading}>{title}</h2>
      <p className={styles.paragraph}>{children}</p>
    </div>
  );
}
