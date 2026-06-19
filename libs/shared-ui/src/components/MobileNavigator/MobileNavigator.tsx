'use client';

import styles from './MobileNavigator.module.css';
import { headerItemsAdmin, headerItemsMembers } from '../lib/magicNumbers';
import { Button } from 'antd';
import { useState } from 'react';

type Props = {
  header_type: 'admin' | 'members';
  logout: () => Promise<void>;
  isLoggedIn?: boolean;
};

export function MobileNavigator({ header_type, logout, isLoggedIn }: Props) {
  const [currentPath] = useState(() =>
    typeof window !== 'undefined' ? window.location.pathname : '',
  );

  const handleLogout = async () => {
    await logout();
    window.location.assign('/');
  };
  return (
    <div className={styles.root}>
      {isLoggedIn === undefined ? (
        <></>
      ) : isLoggedIn ? (
        <div className={styles.logout}>
          <Button
            type="primary"
            style={{ alignSelf: 'flex-start' }}
            onClick={handleLogout}
          >
            ログアウト
          </Button>
        </div>
      ) : (
        <div className={styles.login}>
          <Button
            type="primary"
            style={{ alignSelf: 'flex-start' }}
            onClick={() => window.location.assign('/login')}
          >
            ログイン
          </Button>
        </div>
      )}
      <nav
        className={`${styles.nav} ${header_type === 'admin' ? styles.admin : styles.members}`}
      >
        {/* ヘッダーのナビゲーションボタン */}
        {(header_type === 'members'
          ? headerItemsMembers
          : headerItemsAdmin
        ).map(({ mobileText, emoji, href, class: className }) => {
          // 現在のパスとヘッダーのリンクのパスを比較して、アクティブなリンクを判断
          const isActive = currentPath === href;

          return (
            <a
              key={href}
              href={href}
              className={`${styles.headerNav} ${styles[className]} ${isActive ? styles.activeNav : styles.inactiveNav}`}
            >
              <span className={styles.emoji}>{emoji}</span>
              <span className={styles.text}>{mobileText}</span>
            </a>
          );
        })}
      </nav>
    </div>
  );
}
