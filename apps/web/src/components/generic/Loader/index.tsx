import styles from "./Loader.module.css";

export function Loader() {
  return (
    <div className={styles.loader}>
      <div>
        <div className={styles.trail}/>
        <div className={styles.trail}/>
        <div className={styles.trail}/>
      </div>
      <div>
        <div className={styles.dot}/>
        <div className={styles.dot}/>
        <div className={styles.dot}/>
      </div>
    </div>
  )
}