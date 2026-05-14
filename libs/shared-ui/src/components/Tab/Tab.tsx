'use client';

import styles from "./Tab.module.css"
import {JSX, useEffect, useState} from "react";

type Props = {
  tabs: Map<string, JSX.Element>,
  queryParam?: string,
  queryValues?: Map<string, string>,
}

export function Tab({tabs, queryParam, queryValues}: Props) {
  const [selectedTab, setSelectedTab] = useState(tabs.keys().next().value)
  const entries = Array.from(tabs.entries());
  const getQueryValue = (tab: string) => queryValues?.get(tab) ?? tab;

  useEffect(() => {
    if (!queryParam) {
      return;
    }

    const selectFromQuery = () => {
      const value = new URLSearchParams(window.location.search).get(queryParam);
      const matchedTab = Array.from(tabs.keys()).find((tab) => getQueryValue(tab) === value);
      setSelectedTab(matchedTab ?? tabs.keys().next().value);
    };

    selectFromQuery();
    window.addEventListener("popstate", selectFromQuery);
    return () => {
      window.removeEventListener("popstate", selectFromQuery);
    };
  }, [queryParam, queryValues, tabs]);

  const selectTab = (tab: string) => {
    setSelectedTab(tab);

    if (!queryParam) {
      return;
    }

    const url = new URL(window.location.href);
    url.searchParams.set(queryParam, getQueryValue(tab));
    window.history.pushState({}, "", url);
  };

  if (queryParam) {
    const defaultTab = tabs.keys().next().value;
    const script = `
      (() => {
        const root = document.currentScript?.previousElementSibling;
        if (!root) return;

        const queryParam = root.dataset.queryParam;
        const selectedClass = root.dataset.selectedClass;
        const triggers = Array.from(root.querySelectorAll("[data-tab-trigger]"));
        const panels = Array.from(root.querySelectorAll("[data-tab-panel]"));

        const activate = (value, updateUrl) => {
          triggers.forEach((trigger) => {
            const selected = trigger.dataset.tabValue === value;
            trigger.classList.toggle(selectedClass, selected);
            trigger.setAttribute("aria-selected", String(selected));
          });

          panels.forEach((panel) => {
            panel.hidden = panel.dataset.tabPanel !== value;
          });

          if (updateUrl && queryParam) {
            const url = new URL(window.location.href);
            url.searchParams.set(queryParam, value);
            window.history.pushState({}, "", url);
          }
        };

        const getValueFromUrl = () => {
          const value = new URLSearchParams(window.location.search).get(queryParam);
          return triggers.some((trigger) => trigger.dataset.tabValue === value)
            ? value
            : root.dataset.defaultValue;
        };

        triggers.forEach((trigger) => {
          trigger.addEventListener("click", (event) => {
            event.preventDefault();
            activate(trigger.dataset.tabValue, true);
          });
        });

        window.addEventListener("popstate", () => activate(getValueFromUrl(), false));
        activate(getValueFromUrl(), false);
      })();
    `;

    return (
      <>
        <div
          data-default-value={getQueryValue(defaultTab)}
          data-query-param={queryParam}
          data-selected-class={styles.tabSelected}
        >
          <div className={styles.tabs} role="tablist">
            {
              entries.map(([label], i) => {
                const value = getQueryValue(label);

                return (
                  <a
                    aria-selected={label === defaultTab}
                    className={`${label === defaultTab ? styles.tabSelected : ""} ${styles.tab}`}
                    data-tab-trigger
                    data-tab-value={value}
                    href={`?${queryParam}=${encodeURIComponent(value)}`}
                    key={`tab-${i}`}
                    role="tab"
                  >
                    {label}
                  </a>
                );
              })
            }
          </div>
          {
            entries.map(([label, content], i) => {
              const value = getQueryValue(label);

              return (
                <div data-tab-panel={value} hidden={label !== defaultTab} key={`tab-panel-${i}`}>
                  {content}
                </div>
              );
            })
          }
        </div>
        <script dangerouslySetInnerHTML={{__html: script}} />
      </>
    );
  }

  return (
    <>
      <div className={styles.tabs}>
        {
          entries.map((entry, i) => (
            <a
              key={`tab-${i}`}
              className={`${entry[0] === selectedTab ? styles.tabSelected : ""} ${styles.tab}`}
              onClick={(event) => {
                event.preventDefault();
                selectTab(entry[0]);
              }}
            >
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
