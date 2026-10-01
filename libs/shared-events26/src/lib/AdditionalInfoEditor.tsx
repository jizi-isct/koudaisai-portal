import '@mdxeditor/editor/style.css';

import {
  AdmonitionDirectiveDescriptor,
  BlockTypeSelect,
  BoldItalicUnderlineToggles,
  CodeToggle,
  CreateLink,
  DiffSourceToggleWrapper,
  InsertTable,
  InsertThematicBreak,
  ListsToggle,
  MDXEditor,
  type MDXEditorMethods,
  Separator,
  StrikeThroughSupSubToggles,
  UndoRedo,
  diffSourcePlugin,
  directivesPlugin,
  headingsPlugin,
  imagePlugin,
  linkDialogPlugin,
  linkPlugin,
  listsPlugin,
  markdownShortcutPlugin,
  quotePlugin,
  tablePlugin,
  thematicBreakPlugin,
  toolbarPlugin,
} from '@mdxeditor/editor';
import { useEffect, useRef, useState } from 'react';
import styles from './AdditionalInfoEditor.module.css';

export type AdditionalInfoEditorProps = {
  initialMarkdown: string;
  onChange: (markdown: string) => void;
  onError?: (message: string) => void;
  readOnly?: boolean;
  placeholder?: string;
};

/** 企画追加情報用の Markdown エディター。 */
export function AdditionalInfoEditor({
  initialMarkdown,
  onChange,
  onError,
  readOnly = false,
  placeholder = '企画の追加情報を入力してください',
}: AdditionalInfoEditorProps) {
  const editorRef = useRef<MDXEditorMethods>(null);
  const [overlayContainer, setOverlayContainer] =
    useState<HTMLDivElement | null>(null);

  useEffect(() => {
    editorRef.current?.setMarkdown(initialMarkdown);
  }, [initialMarkdown]);

  return (
    <div ref={setOverlayContainer} className={styles.editorTheme}>
      <MDXEditor
        ref={editorRef}
        className={`${styles.editor} mdxeditor-full-height`}
        contentEditableClassName={styles.typography}
        markdown={initialMarkdown}
        onChange={(markdown) => {
          onChange(markdown);
        }}
        onError={({ error: editorError }) => {
          onError?.(`Markdownを解析できませんでした: ${editorError}`);
        }}
        overlayContainer={overlayContainer}
        placeholder={placeholder}
        readOnly={readOnly}
        plugins={[
          toolbarPlugin({
            toolbarContents: () => (
              <DiffSourceToggleWrapper>
                <UndoRedo />
                <BlockTypeSelect />
                <Separator />
                <BoldItalicUnderlineToggles />
                <StrikeThroughSupSubToggles />
                <Separator />
                <ListsToggle />
                <Separator />
                <CreateLink />
                <CodeToggle />
                <Separator />
                <InsertTable />
                <InsertThematicBreak />
              </DiffSourceToggleWrapper>
            ),
          }),
          imagePlugin(),
          diffSourcePlugin({
            diffMarkdown: initialMarkdown,
            viewMode: 'rich-text',
          }),
          headingsPlugin({ allowedHeadingLevels: [2, 3, 4] }),
          listsPlugin(),
          tablePlugin(),
          quotePlugin(),
          thematicBreakPlugin(),
          linkPlugin(),
          linkDialogPlugin(),
          directivesPlugin({
            directiveDescriptors: [AdmonitionDirectiveDescriptor],
          }),
          markdownShortcutPlugin(),
        ]}
      />
    </div>
  );
}
