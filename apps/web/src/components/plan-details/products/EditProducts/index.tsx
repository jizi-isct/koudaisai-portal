import {Form, Input, Button, Divider, Space} from "antd";
import {LoadingOutlined, PlusOutlined, UploadOutlined} from "@ant-design/icons";
import {EditProduct} from "@/components/plan-details/products/EditProduct";
import {ProductsCreate, ProductsRead} from "@/lib";

type Props = {
  product: ProductsRead | undefined,
  updateProducts: (products: ProductsCreate) => Promise<void>,
  isLoading: boolean,
}

export function EditProducts({product, updateProducts, isLoading}: Props) {
  return (
    <Form
      layout="vertical"
      onFinish={async (values) => await updateProducts(values)}
      size="small"
      initialValues={product}
    >
      <Form.Item
        label={"商品全体の説明"}
        name={["description"]}
        rules={[{ required: true, message: "商品全体の説明を入力してください" }]}
      >
        <Input.TextArea placeholder="例: 当日の販売方針や注意点など" rows={3} />
      </Form.Item>

      <Divider orientation="left">商品一覧</Divider>

      <Form.List name={["items"]}>
        {(fields, { add, remove }) => (
          <Space direction="vertical" style={{ width: "100%" }} size="large">
            {fields.map((field) => (
              <div key={field.key}>
                <EditProduct name={[field.name]} />
                <div style={{ textAlign: "right", marginTop: -8 }}>
                  <Button danger size="small" onClick={() => remove(field.name)}>
                    この商品を削除
                  </Button>
                </div>
              </div>
            ))}

            <Form.Item>
              <Button type="dashed" onClick={() => add({ options: [] })} block icon={<PlusOutlined />}>商品を追加</Button>
            </Form.Item>
          </Space>
        )}
      </Form.List>

      <Form.Item>
        <Space>
          <Button
            type="primary"
            htmlType="submit"
            disabled={isLoading}
          >
            {isLoading ? <LoadingOutlined/> : <UploadOutlined/>} 更新
          </Button>
          <Button htmlType="reset">リセット</Button>
        </Space>
      </Form.Item>
    </Form>
  )
}