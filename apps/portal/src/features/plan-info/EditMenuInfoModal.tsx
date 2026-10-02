import {
  MenuInfoForm,
  type MenuInfo,
} from '@koudaisai-portal/shared-ui-events26';
import { Modal } from '@koudaisai/shared-ui';
import { useEffect, useState } from 'react';
import styles from './EditMenuInfoModal.module.css';

type Props = {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  menu: MenuInfo | undefined;
  updateMenu: (menu: MenuInfo) => Promise<void>;
};

/** 商品、価格、オプションを含む企画メニューを編集するモーダル。 */
export function EditMenuInfoModal({
  isOpen,
  setOpen,
  menu,
  updateMenu,
}: Props) {
  const [isSaving, setIsSaving] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);

  useEffect(() => {
    if (isOpen) setSaveError(null);
  }, [isOpen, menu]);

  const handleUpdate = async (newMenu: MenuInfo) => {
    setIsSaving(true);
    setSaveError(null);
    try {
      await updateMenu(newMenu);
      setOpen(false);
    } catch (updateError) {
      setSaveError(
        updateError instanceof Error
          ? updateError.message
          : 'メニュー情報を更新できませんでした。',
      );
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Modal isOpen={isOpen} setOpen={setOpen}>
      <div className={styles.modalContent}>
        <h2 className={styles.title}>メニュー情報を編集</h2>
        {isOpen && (
          <MenuInfoForm menu={menu} onSubmit={handleUpdate} disabled={isSaving}>
            {saveError && (
              <p className={styles.error} role="alert">
                {saveError}
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
                type="submit"
                className={styles.updateButton}
                disabled={isSaving}
              >
                {isSaving ? '更新中…' : '更新する'}
              </button>
            </div>
          </MenuInfoForm>
        )}
      </div>
    </Modal>
  );
}
