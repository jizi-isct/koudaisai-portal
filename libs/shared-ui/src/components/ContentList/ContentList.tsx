'use client'
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
    const start = (page - 1) * pageSize;
    const end = start + pageSize;
    return contents.slice(start, end);
  }, [contents, page, pageSize, pagination]);

  if (contents.length === 0) {
    return <></>;
  }

  return (
    <div className="w-full border border-darkblue rounded-[10px] py-2 px-[15px] shadow-[4px_4px_0_0_var(--darkblue)] my-8 bg-white">
      {
        currentContents.map((item, i) => (
          <React.Fragment key={`fragment-${i}`}>
            {i > 0 && <div className="h-px w-full bg-darkblue"/>}
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