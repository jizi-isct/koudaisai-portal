"use client";

import {Heading2} from "@/components/generic";
import {DocumentCategoryRead} from "@/lib";

type DocumentCategoryHeadingProps = {
  documentCategory: DocumentCategoryRead
  emoji: string
}

export function HeadingViewDocumentCategory({documentCategory, emoji}: DocumentCategoryHeadingProps) {
  return (
    <>
      <Heading2 emoji={documentCategory.emoji ?? emoji}>
        {documentCategory.title}
      </Heading2>
    </>
  )
}