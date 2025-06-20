import Form from "antd/es/form";
import {$apiAdmin} from "@/lib";
import {LoadingScreen} from "@/components/generic";
import {Cascader} from "antd";

interface Option {
  value: string;
  label: string;
  children?: Option[];
}

type TargetSpecifierProps = {
  name: string,
  onChange: (value: string) => void;
}


export function TargetSpecifier({name, onChange}: TargetSpecifierProps) {
  const {data: users} = $apiAdmin.useQuery("get", "/users")

  if (!users) {
    return <LoadingScreen/>
  }

  const options: Option[] = [
    {
      value: 'exhibitor',
      label: '参加団体',
      children: [
        {
          value: 'hangzhou',
          label: '種類',
          children: [
            {
              value: 'general',
              label: '一般',
            },
            {
              value: 'booth',
              label: '模擬店'
            },
            {
              value: 'stage',
              label: 'ステージ',
            },
            {
              value: 'labo',
              label: '研究室',
            }
          ],
        },
      ],
    },
    {
      value: 'user',
      label: 'ユーザー',
      children: [
        {
          value: 'nologin',
          label: '非ログイン',
        },
        {
          value: 'id',
          label: '個人',
          children: users.map((user) => (
            {
              value: user.id,
              label: `${user.exhibition_id}の${user.last_name} ${user.first_name}`
            }
          ))
        }
      ],
    },
  ];

  const handleChange = (value: string[]) => {
    onChange(value.join("/"))
  }


  return (
    <Form.Item name={name} noStyle rules={[{required: true}]}>
      <Cascader
        options={options}
        onChange={handleChange}
        showSearch
      />
    </Form.Item>
  )
}