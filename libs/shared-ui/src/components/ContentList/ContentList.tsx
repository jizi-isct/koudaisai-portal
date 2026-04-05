'use client'
import styles from "./ContentList.module.css";
import React, {ReactNode, useMemo, useState} from "react";
import {Pagination} from "antd";

type Props = {
  contents: Array<ReactNode>;
  pagination?: boolean;
  pageSize?: number;
}

export function ContentList({contents, pagination = false, pageSize = 10}: Props) {
  const [page, setPage] = useState(1);

  const currentContents = useMemo(() => {
    if(!pagination) {
      return contents;
    }
    // ページ管理
    const start = (page - 1) * pageSize;
    const end = start + pageSize;
    return contents.slice(start, end);
  }, [contents, page, pageSize, pagination]);

  if (contents.length === 0) {
    return <></>;
  }

  return (
    <div className={styles.root}>
      {
        currentContents.map((item, i) => (
          <React.Fragment key={`fragment-${i}`}>
            { i > 0 && <div className={styles.separator}/>}
            {item}
          </React.Fragment>
        ))
      }
      {pagination && (
        <Pagination
          current={page}
          pageSize={pageSize}
          total={contents.length}
          onChange={(p) => setPage(p)}
          showSizeChanger={false}
          align="center"
        />
      )}
    </div>
  )
}