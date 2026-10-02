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
  /** エディターの初期 Markdown。値が変わると編集内容を置き換えます。 */
  initialMarkdown: string;
  /** 編集後の Markdown を受け取ります。API による保存は呼び出し元で行います。 */
  onChange: (markdown: string) => void;
  /** Markdown の解析エラーを文字列で受け取ります。 */
  onError?: (message: string) => void;
  /** 保存中などに編集を無効にします。 */
  readOnly?: boolean;
  /** 現在の実装では表示に使用されません。 */
  placeholder?: string;
};

/**
 * 企画追加情報用の Markdown エディター。
 *
 * @remarks
 * 初期値を `initialMarkdown` に渡し、編集結果を `onChange` で受け取ります。
 * 解析エラーの表示には `onError`、保存中の編集防止には `readOnly` を使用できます。
 * モーダルの開閉、API による取得・保存、保存結果の表示は呼び出し元で実装します。
 *
 * @example
 * ```tsx
 * import { AdditionalInfoEditor } from '@koudaisai-portal/shared-ui-events26';
 *
 * <AdditionalInfoEditor
 *   initialMarkdown={additionalInfo}
 *   onChange={setMarkdown}
 *   onError={setError}
 *   readOnly={isSaving}
 * />
 * ```
 */
export function AdditionalInfoEditor({
  initialMarkdown,
  onChange,
  onError,
  readOnly = false,
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
