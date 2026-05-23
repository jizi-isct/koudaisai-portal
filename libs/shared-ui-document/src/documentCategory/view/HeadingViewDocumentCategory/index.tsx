import { Heading2 } from '@koudaisai/shared-ui';
import { DocumentCategoryRead } from '@koudaisai/shared-types';

type DocumentCategoryHeadingProps = {
  documentCategory: DocumentCategoryRead;
  defaultEmoji: string;
};

export function HeadingViewDocumentCategory({
  documentCategory,
  defaultEmoji,
}: DocumentCategoryHeadingProps) {
  return (
    <Heading2 emoji={documentCategory.emoji ?? defaultEmoji}>
      {documentCategory.title}
    </Heading2>
  );
}
