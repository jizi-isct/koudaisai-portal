import type { apiComponents } from '@koudaisai/shared-types';
import { Modal } from '@koudaisai/shared-ui';
import { Button, Divider, Form, Input, InputNumber } from 'antd';
import { useEffect, useState } from 'react';
import styles from './EditMenuInfoModal.module.css';

export type MenuInfo =
  apiComponents['schemas']['GetProjectDetails200ResponseMenu'];

type Props = {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  menu: MenuInfo | undefined;
  updateMenu: (menu: MenuInfo) => Promise<void>;
};

const EMPTY_MENU: MenuInfo = {
  description: '',
  items: [],
};

function cloneMenu(menu: MenuInfo | undefined): MenuInfo {
  return {
    description: menu?.description ?? '',
    items:
      menu?.items.map((item) => ({
        ...item,
        options: item.options.map((option) => ({ ...option })),
      })) ?? [],
  };
}

/** 商品、価格、オプションを含む企画メニューを編集するモーダル。 */
export function EditMenuInfoModal({
  isOpen,
  setOpen,
  menu,
  updateMenu,
}: Props) {
  const [form] = Form.useForm<MenuInfo>();
  const [isSaving, setIsSaving] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);

  // 閉じて破棄した内容を残さず、再取得した最新のメニューを表示する。
  useEffect(() => {
    if (!isOpen) {
      return;
    }

    form.resetFields();
    form.setFieldsValue(cloneMenu(menu));
    setSaveError(null);
  }, [form, isOpen, menu]);

  const handleUpdate = async (values: MenuInfo) => {
    setIsSaving(true);
    setSaveError(null);

    const normalizedMenu: MenuInfo = {
      description: values.description,
      items: values.items.map((item) => ({
        name: item.name,
        price: item.price,
        options: item.options ?? [],
      })),
    };

    try {
      await updateMenu(normalizedMenu);
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
        <Form<MenuInfo>
          form={form}
          initialValues={EMPTY_MENU}
          layout="vertical"
          onFinish={handleUpdate}
          disabled={isSaving}
        >
          <Form.Item
            label="メニュー全体の説明"
            name="description"
            rules={[
              {
                required: true,
                whitespace: true,
                message: 'メニュー全体の説明を入力してください',
              },
            ]}
          >
            <Input.TextArea
              placeholder="例: 当日の販売方針や注意点など"
              rows={3}
            />
          </Form.Item>

          <Divider titlePlacement="start">商品一覧</Divider>

          <Form.List name="items">
            {(itemFields, { add: addItem, remove: removeItem }) => (
              <div className={styles.list}>
                {itemFields.map((itemField, itemIndex) => (
                  <section className={styles.itemCard} key={itemField.key}>
                    <h3 className={styles.itemTitle}>商品 {itemIndex + 1}</h3>
                    <Form.Item
                      label="商品名"
                      name={[itemField.name, 'name']}
                      rules={[
                        {
                          required: true,
                          whitespace: true,
                          message: '商品名を入力してください',
                        },
                      ]}
                    >
                      <Input placeholder="例: かき氷 / フランクフルト など" />
                    </Form.Item>

                    <Form.Item
                      label="値段"
                      name={[itemField.name, 'price']}
                      tooltip="未入力（または0）でも構いません。価格が変動する場合は未入力にできます。"
                    >
                      <InputNumber
                        className={styles.priceInput}
                        placeholder="例: 500"
                        addonAfter="円"
                      />
                    </Form.Item>

                    <Divider titlePlacement="start" plain>
                      トッピングやフレーバーなどのオプション
                    </Divider>

                    <Form.List name={[itemField.name, 'options']}>
                      {(
                        optionFields,
                        { add: addOption, remove: removeOption },
                      ) => (
                        <div className={styles.optionList}>
                          {optionFields.map((optionField, optionIndex) => (
                            <section
                              className={styles.optionCard}
                              key={optionField.key}
                            >
                              <h4 className={styles.optionTitle}>
                                オプション {optionIndex + 1}
                              </h4>
                              <Form.Item
                                label="オプション名"
                                name={[optionField.name, 'name']}
                                rules={[
                                  {
                                    required: true,
                                    whitespace: true,
                                    message: 'オプション名を入力してください',
                                  },
                                ]}
                              >
                                <Input placeholder="例: 大盛り / いちごフレーバー など" />
                              </Form.Item>

                              <Form.Item
                                label="値段"
                                name={[optionField.name, 'price']}
                                tooltip="未入力（または0）でも構いません。必要に応じて追加料金を入力してください。"
                              >
                                <InputNumber
                                  className={styles.priceInput}
                                  placeholder="例: 100"
                                  addonAfter="円"
                                />
                              </Form.Item>

                              <div className={styles.removeButtonLayout}>
                                <Button
                                  danger
                                  size="small"
                                  type="default"
                                  onClick={() => removeOption(optionField.name)}
                                >
                                  このオプションを削除
                                </Button>
                              </div>
                            </section>
                          ))}

                          <Button
                            block
                            type="dashed"
                            onClick={() => addOption()}
                          >
                            オプションを追加
                          </Button>
                        </div>
                      )}
                    </Form.List>

                    <div className={styles.removeButtonLayout}>
                      <Button
                        danger
                        size="small"
                        type="default"
                        onClick={() => removeItem(itemField.name)}
                      >
                        この商品を削除
                      </Button>
                    </div>
                  </section>
                ))}

                <Button
                  block
                  type="dashed"
                  onClick={() => addItem({ options: [] })}
                >
                  商品を追加
                </Button>
              </div>
            )}
          </Form.List>

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
        </Form>
      </div>
    </Modal>
  );
}
