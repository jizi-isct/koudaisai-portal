import styles from "./Selector.module.css"

type Props = {
  options: string[],
  selectedOption: string,
  setOption: (value: string) => void
}

export function Selector({options, selectedOption, setOption}: Props) {
  return (
    <div className={styles.root}>
      <div className={styles.optionContainer}>
        {
          options.map((option, i) => (
            <div key={i} className={`${styles.option} ${selectedOption === option ? styles.selected : ""}`} onClick={() => setOption(option)}>
              {option}
            </div>
          ))
        }
      </div>
    </div>
  )
}