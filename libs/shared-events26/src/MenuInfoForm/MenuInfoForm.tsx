import { Button, Divider, Form, Input } from 'antd';
import { useEffect, type CSSProperties, type ReactNode } from 'react';
import styles from './MenuInfoForm.module.css';

export type MenuInfo = {
  description: string;
  items: {
    name: string;
    price?: number | null;
    options: { name: string; price?: number | null }[];
  }[];
};

type MenuFormValues = {
  description: string;
  items: {
    name: string;
    price?: string;
    options: { name: string; price?: string }[];
  }[];
};

export type MenuInfoFormProps = {
  menu?: MenuInfo;
  onSubmit: (menu: MenuInfo) => void | Promise<void>;
  disabled?: boolean;
  onValuesChange?: () => void;
  style?: CSSProperties;
  children?: ReactNode;
};

function cloneMenu(menu: MenuInfo | undefined): MenuFormValues {
  return {
    description: menu?.description ?? '',
    items:
      menu?.items.map((item) => ({
        name: item.name,
        price: item.price == null ? undefined : String(item.price),
        options: item.options.map((option) => ({
          name: option.name,
          price: option.price == null ? undefined : String(option.price),
        })),
      })) ?? [],
  };
}

function validatePrice(_: unknown, value: string | undefined) {
  if (
    value === undefined ||
    value === '' ||
    (/^\d+(?:\.\d+)?$/.test(value) && Number.isFinite(Number(value)))
  ) {
    return Promise.resolve();
  }

  return Promise.reject(new Error('値段は0以上の半角数字で入力してください'));
}

function parsePrice(value: string | undefined): number | undefined {
  return value === undefined || value === '' ? undefined : Number(value);
}

function normalizeMenu(values: MenuFormValues): MenuInfo {
  return {
    description: values.description,
    items: values.items.map((item) => ({
      name: item.name,
      price: parsePrice(item.price),
      options: (item.options ?? []).map((option) => ({
        name: option.name,
        price: parsePrice(option.price),
      })),
    })),
  };
}

/** 商品、価格、オプションを編集する API 非依存のフォーム。 */
export function MenuInfoForm({
  menu,
  onSubmit,
  disabled = false,
  onValuesChange,
  style,
  children,
}: MenuInfoFormProps) {
  const [form] = Form.useForm<MenuFormValues>();

  useEffect(() => {
    form.resetFields();
    form.setFieldsValue(cloneMenu(menu));
  }, [form, menu]);

  return (
    <Form<MenuFormValues>
      form={form}
      initialValues={cloneMenu(menu)}
      layout="vertical"
      onFinish={(values) => onSubmit(normalizeMenu(values))}
      disabled={disabled}
      onValuesChange={onValuesChange}
      style={style}
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
        <Input.TextArea placeholder="例: 当日の販売方針や注意点など" rows={3} />
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
                  rules={[{ validator: validatePrice }]}
                >
                  <Input
                    className={styles.priceInput}
                    placeholder="例: 500"
                    inputMode="decimal"
                    suffix="円"
                  />
                </Form.Item>

                <Divider titlePlacement="start" plain>
                  トッピングやフレーバーなどのオプション
                </Divider>

                <Form.List name={[itemField.name, 'options']}>
                  {(optionFields, { add: addOption, remove: removeOption }) => (
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
                            rules={[{ validator: validatePrice }]}
                          >
                            <Input
                              className={styles.priceInput}
                              placeholder="例: 100"
                              inputMode="decimal"
                              suffix="円"
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

                      <Button block type="dashed" onClick={() => addOption()}>
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

      {children}
    </Form>
  );
}
