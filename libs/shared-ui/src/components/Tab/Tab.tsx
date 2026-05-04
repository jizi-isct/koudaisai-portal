'use client';

import {JSX, useState} from "react";

type Props = {
  tabs: Map<string, JSX.Element>
}

export function Tab({tabs}: Props) {
  const [selectedTab, setSelectedTab] = useState(tabs.keys().next().value)
  return (
    <>
      <div className="py-[0.7em] px-[1em] flex flex-wrap justify-center gap-[1em] my-[1em] mx-auto w-fit bg-white border border-darkblue rounded-[1em] drop-shadow-[4px_4px_0_var(--darkblue)]">
        {
          Array.from(tabs.entries()).map((entry, i) => (
            <a
              key={`tab-${i}`}
              className={`cursor-pointer text-[16px] ${entry[0] === selectedTab ? "font-bold" : ""}`}
              onClick={() => setSelectedTab(entry[0])}
            >
              {entry[0]}
            </a>
          ))
        }
      </div>
      <div>
        {selectedTab && tabs.get(selectedTab)}
      </div>
    </>
  )
}