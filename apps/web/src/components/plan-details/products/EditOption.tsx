import {Form, Input, InputNumber, Space, Button} from "antd";
import {DeleteOutlined} from "@ant-design/icons";

type Props = {
  name: (string | number)[];
  onRemove?: () => void;
}

export function EditOption({name, onRemove}: Props) {
  return (
    <div style={{border: '1px solid #eee', borderRadius: 6, padding: 12, marginBottom: 12}}>
      <Space align="start" style={{width: '100%'}} direction="vertical">
        <Form.Item
          label={"オプション名"}
          name={[...name, 'name']}
          rules={[{required: true, message: 'オプション名を入力してください'}]}
        >
          <Input placeholder="例: 大盛り / いちごフレーバー など"/>
        </Form.Item>

        <Form.Item
          label={"値段"}
          name={[...name, 'price']}
          tooltip="未入力（または0）でも可。必要に応じて上乗せ額を入力してください。"
        >
          <InputNumber style={{width: '100%'}} placeholder="例: 100"/>
        </Form.Item>

        {onRemove && (
          <Button danger icon={<DeleteOutlined/>} onClick={onRemove} size="small">
            このオプションを削除
          </Button>
        )}
      </Space>
    </div>
  )
}