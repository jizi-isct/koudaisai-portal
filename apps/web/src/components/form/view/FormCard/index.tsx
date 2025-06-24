import styles from "./FormCard.module.css";
import {FormRead} from "@/lib";
import {useMemo} from "react";

type ListsProps = {
  form: FormRead
};

export function FormCard({form}: ListsProps) {
  const formLink = useMemo(() => {
    if ('type_external' in form) {
      return form.type_external.form_url;
    } else {
      return `/forms/form?formId=${form.id}`;
    }
  }, [form])

  return (
    <div key={form.id} className={styles.forms}>
      <a href={formLink}>
        <div className={styles.titleWrapper}>
          <h2 className={styles.title}>{form.form_name}</h2>
          <h2
            className={styles.dueDate}>回答期限: {form.due_date ? new Date(form.due_date).toLocaleString("ja-JP") : "なし"}</h2>
        </div>
        <p className={styles.summary}>{form.summary}</p>
      </a>
    </div>
  )
}
