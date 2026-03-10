import styles from "./Error.module.css"

export function Error({error}: { error: Error }) {
  return (
    <div className={styles.root}>
      <div className={styles.error}>
        ⚠️
      </div>
      <div>ERROR</div>
      <div>{error.message}</div>
    </div>
  )
}