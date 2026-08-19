import { useState } from 'react';
import { getProjectIconUrl } from '@/features/api/events26Api';
import styles from './PlanIcon.module.css';

type Props = {
  projectId: string;
};

export function PlanIcon({ projectId }: Props) {
  // アイコン未登録の企画では 404 が返る。プレースホルダは出さず、何も表示しない。
  const [hasError, setHasError] = useState(false);

  if (hasError) {
    return null;
  }

  return (
    <img
      src={getProjectIconUrl(projectId)}
      alt={`${projectId} のアイコン`}
      width={64}
      height={64}
      className={styles.icon}
      onError={() => setHasError(true)}
    />
  );
}
