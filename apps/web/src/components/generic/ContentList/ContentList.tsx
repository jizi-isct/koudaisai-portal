import styles from "./ContentList.module.css";
import React, {ReactNode} from "react";

type Props = {
  contents: Array<ReactNode>
}

export function ContentList({contents}: Props) {
  if (contents.length === 0) {
    return <></>;
  }
  return (
    <div className={styles.root}>
      {
        contents.map((content, i) => (
          <React.Fragment key={`fragment-${i}`}>
            { i > 0 && <div className={styles.separator}/>}
            {content}
          </React.Fragment>
        ))
      }
    </div>
  )
}