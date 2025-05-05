import styles from "./ContentList.module.css";
import {Content} from "../lib/types";
import {ContentRow} from "../ContentRow";
import React from "react";

type Props = {
  contents: Array<Content>
}

export function ContentList({contents}: Props) {
  return (
    <div className={styles.root}>
      {
        contents.map((content, i) => (
          <React.Fragment key={`fragment-${i}`}>
            { i > 0 && <div className={styles.separator}/>}
            <ContentRow content={content} />
          </React.Fragment>
        ))
      }
    </div>
  )
}