import { Modal } from '@koudaisai/shared-ui';
import { AdditionalInfoEditor } from '@koudaisai-portal/shared-events26';
import { useEffect, useState } from 'react';
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
        {isOpen && (
          <AdditionalInfoEditor
            initialMarkdown={additionalInfo}
            onChange={(markdown) => {
              setCurrentMarkdown(markdown);
              setEditorError(null);
              setSaveError(null);
              setIsSaved(false);
            }}
            onError={setEditorError}
            readOnly={isSaving}
          />
        )}

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
