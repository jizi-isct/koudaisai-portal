"use client";

import { DocumentFormatMarkdownRead } from "@koudaisai/shared-types";
import Markdown from "react-markdown";

type ViewDocumentFormatMarkdownProps = {
  format: DocumentFormatMarkdownRead
}

/**
 * Markdown形式のドキュメントを表示するコンポーネント
 * @param format
 * @constructor
 */
export function ViewDocumentFormatMarkdown({format}: ViewDocumentFormatMarkdownProps) {
  return (
    <div style={{textAlign: "left", padding: "10px"}}>
      <Markdown>
        {format.content}
      </Markdown>
    </div>
  )
}