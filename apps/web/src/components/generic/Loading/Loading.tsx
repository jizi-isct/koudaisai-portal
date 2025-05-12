import styles from "./Loading.module.css"

export function Loading() {
  return (
    <div className={styles.root}>
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
        {/*<div className={styles.trail1}/>*/}
        {/*<div className={styles.trail2}/>*/}
      </div>
      <div>LOADING</div>
    </div>
  )
}