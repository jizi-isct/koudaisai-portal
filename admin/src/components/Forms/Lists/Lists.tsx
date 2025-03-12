import styles from "./Lists.module.css";

type HeaderListsProps = {
    title: string;
    status: string;
    dueDate?: string;
    summary?: string;
    formId: string;
};

const HeaderLists = ({title, status, dueDate, summary, formId}: HeaderListsProps) => {
  return (
      <a href={`/forms/form.html?formId=${formId}`} className={styles.forms}>
      <div className={styles.titleWrapper}>
          <h2 className={styles.title}>{title}</h2>
          <h2 className={styles.status}>{status}</h2>
          <h2 className={styles.dueDate}>回答期限: {dueDate}</h2>
        </div>
        <p className={styles.summary}>{summary}</p>
    </a>
    
  );
};

export default HeaderLists;
