import styles from "./Lists.module.css";

type ListsProps = {
    title: string;
    status: string;
    dueDate?: string;
    summary?: string;
    formId: string;
};

const Lists = ({title, status, dueDate, summary, formId}: ListsProps) => {
    const formLink = `/forms/form.html?formId=${formId}`;
  return (
    <div key={formId} className={styles.forms}>
      <a href={formLink}>
        <div className={styles.titleWrapper}>
            <h2 className={styles.title}>{title}</h2>
            <h2 className={styles.status}>{status}</h2>
            <h2 className={styles.dueDate}>回答期限: {dueDate}</h2>
        </div>
        <p className={styles.summary}>{summary}</p>
      </a>
    </div>
  );
};

export default Lists;
