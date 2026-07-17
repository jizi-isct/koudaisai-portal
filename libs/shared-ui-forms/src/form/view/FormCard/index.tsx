import styles from './FormCard.module.css';
import { FormRead } from '@koudaisai/shared-types';
import { useMemo } from 'react';

type FormCardProps = {
  form: FormRead;
};

export function FormCard({ form }: FormCardProps) {
  const formLink = useMemo(() => {
    if (form.type === 'external') {
      return form.form_url;
    } else {
      return `/forms/form?formId=${form.id}`;
    }
  }, [form]);

  // フォームの回答期限を12/12 12:34のような形式で表示
  const formattedDueDate = useMemo(() => {
    if (!form.due_date) return 'なし';

    return new Intl.DateTimeFormat('ja-JP', {
      month: 'numeric',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
      timeZone: 'Asia/Tokyo',
    }).format(new Date(form.due_date));
  }, [form.due_date]);

  return (
    <a href={formLink} className={styles.forms}>
      <h2 className={styles.title}>{form.name}</h2>
      <p className={styles.summary}>{form.summary}</p>
      <h2 className={styles.dueDate}>回答期限: {formattedDueDate}</h2>
    </a>
  );
}
