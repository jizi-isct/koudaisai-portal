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
import { Modal } from '@koudaisai/shared-ui';
import { useEffect, useRef, useState } from 'react';
import styles from './EditAdditionalInfoModal.module.css';

type Props = {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  additionalInfo: string;
  updateAdditionalInfo: (newAdditionalInfo: string) => Promise<void>;
};

/** 企画追加情報をリッチテキスト形式で編集するモーダル。 */
export function EditAdditionalInfoModal({
  isOpen,
  setOpen,
  additionalInfo,
  updateAdditionalInfo,
}: Props) {
  const editorRef = useRef<MDXEditorMethods>(null);
  const [overlayContainer, setOverlayContainer] =
    useState<HTMLDivElement | null>(null);
  const [currentMarkdown, setCurrentMarkdown] = useState(additionalInfo);
  const [isSaving, setIsSaving] = useState(false);
  const [editorError, setEditorError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [isSaved, setIsSaved] = useState(false);

  // 閉じている間の再取得や、破棄した未保存内容を次回表示時に反映する。
  useEffect(() => {
    if (!isOpen) {
      return;
    }

    setCurrentMarkdown(additionalInfo);
    editorRef.current?.setMarkdown(additionalInfo);
    setEditorError(null);
    setSaveError(null);
    setIsSaved(false);
  }, [isOpen, additionalInfo]);

  const handleUpdate = async () => {
    setIsSaving(true);
    setSaveError(null);
    setIsSaved(false);

    try {
      await updateAdditionalInfo(currentMarkdown);
      setIsSaved(true);
    } catch (updateError) {
      setSaveError(
        updateError instanceof Error
          ? updateError.message
          : '企画追加情報を更新できませんでした。',
      );
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Modal isOpen={isOpen} setOpen={setOpen}>
      <div className={styles.modalContent}>
        <h2 className={styles.title}>企画追加情報を編集</h2>
        <div ref={setOverlayContainer} className={styles.editorTheme}>
          <MDXEditor
            ref={editorRef}
            className={styles.editor}
            contentEditableClassName={styles.typography}
            markdown={additionalInfo}
            onChange={(markdown) => {
              setCurrentMarkdown(markdown);
              setEditorError(null);
              setSaveError(null);
              setIsSaved(false);
            }}
            onError={({ error: editorError }) => {
              setEditorError(`Markdownを解析できませんでした: ${editorError}`);
            }}
            overlayContainer={overlayContainer}
            placeholder="企画の追加情報を入力してください"
            readOnly={isSaving}
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
                diffMarkdown: additionalInfo,
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

        {(editorError || saveError) && (
          <p className={styles.error} role="alert">
            {editorError ?? saveError}
          </p>
        )}
        {isSaved && (
          <p className={styles.success} role="status">
            企画追加情報を更新しました。
          </p>
        )}

        <div className={styles.actions}>
          <button
            type="button"
            className={styles.cancelButton}
            onClick={() => setOpen(false)}
            disabled={isSaving}
          >
            閉じる
          </button>
          <button
            type="button"
            className={styles.updateButton}
            onClick={handleUpdate}
            disabled={
              isSaving || currentMarkdown === additionalInfo || !!editorError
            }
          >
            {isSaving ? '更新中…' : '更新する'}
          </button>
        </div>
      </div>
    </Modal>
  );
}
