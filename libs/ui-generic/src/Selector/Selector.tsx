import styles from "./Selector.module.css"

type Props = {
  options: string[],
  slectedOption: string,
  setOption: (value: string) => void
}

export function Selector({options, slectedOption, setOption}: Props) {
  return (
    <div className={styles.root}>
      <div className={styles.optionContainer}>
        {
          options.map((option, i) => (
            <div key={i} className={`${styles.option} ${slectedOption === option ? styles.selected : ""}`} onClick={() => setOption(option)}>
              {option}
            </div>
          ))
        }
      </div>
    </div>
  )
}