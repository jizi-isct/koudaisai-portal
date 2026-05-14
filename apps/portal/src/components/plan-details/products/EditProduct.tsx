import {Form, Input, InputNumber, Button, Space, Divider} from "antd";
import {PlusOutlined} from "@ant-design/icons";
import {EditOption} from "@/components/plan-details/products/EditOption";

type Props = {
  name: (string | number)[];
}

export function EditProduct({name}: Props) {
  return (
    <div style={{border: '1px solid #ddd', borderRadius: 8, padding: 16, marginBottom: 16}}>
      <Space direction="vertical" style={{width: '100%'}} size="middle">
        <Form.Item
          label={"商品名"}
          name={[...name, 'name']}
          rules={[{required: true, message: '商品名を入力してください'}]}
        >
          <Input placeholder="例: かき氷 / フランクフルト など"/>
        </Form.Item>

        <Form.Item
          label={"値段"}
          name={[...name, 'price']}
          tooltip="未入力（または0）でも可。価格が変動する場合は未入力のままにできます。"
        >
          <InputNumber style={{width: '100%'}} placeholder="例: 500"/>
        </Form.Item>

        <Divider orientation="left">トッピングやフレーバーなどのオプション</Divider>
        <Form.List name={[...name, 'options']}>
          {(fields, {add, remove}) => (
            <>
              {fields.map((field) => (
                <EditOption
                  key={field.key}
                  name={[field.name]}
                  onRemove={() => remove(field.name)}
                />
              ))}
              <Form.Item>
                <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined/>}>
                  オプションを追加
                </Button>
              </Form.Item>
            </>
          )}
        </Form.List>
      </Space>
    </div>
  )
}