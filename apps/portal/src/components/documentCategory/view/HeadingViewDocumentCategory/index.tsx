"use client";

import {Heading2} from "@koudaisai/shared-ui";
import { DocumentCategoryRead } from "@koudaisai/shared-types";

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