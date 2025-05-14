'use client';

import styles from "./Tab.module.css"
import {JSX, useState} from "react";

type Props = {
  tabs: Map<string, JSX.Element>
}

export function Tab({tabs}: Props) {
  const [selectedTab, setSelectedTab] = useState(tabs.keys().next().value)
  return (
    <>
      <div className={styles.tabs}>
        {
          Array.from(tabs.entries()).map((entry, i) => (
            <a key={`tab-${i}`} className={`${entry[0] === selectedTab ? styles.tabSelected : ""} ${styles.tab}`}
               onClick={() => setSelectedTab(entry[0])}>
              {entry[0]}
            </a>
          ))
        }
      </div>
      <div>
        {
          selectedTab && tabs.get(selectedTab)
        }
      </div>
    </>
  )
}