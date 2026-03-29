import {LoadingScreen} from "@koudaisai/shared-ui";
import {Cascader} from "antd";
import Form from "antd/es/form";
import {$apiAdmin} from "@/lib/api";

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
  const {data: groups} = $apiAdmin.useQuery("get", "/groups")

  if (!users || !groups) {
    return <LoadingScreen/>
  }

  const options: Option[] = [
    {
      value: 'group',
      label: '団体',
      children: [
        {
          value: 'type',
          label: '種類',
          children: [
            {
              value: 'plan_general',
              label: '一般企画',
            },
            {
              value: 'plan_booth',
              label: '模擬店企画'
            },
            {
              value: 'plan_stage',
              label: 'ステージ企画',
            },
            {
              value: 'plan_labo',
              label: '研究室企画',
            },
            {
              value: 'press',
              label: '学内取材団体',
            }
          ],
        },
        {
          value: 'id',
          label: '指定',
          children: groups.map((group) => (
            {
              value: group.id,
              label: `${group.id} - ${group.name}`
            }
          ))
        }
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
              label: `${user.group_id}の${user.name}`
            }
          ))
        }
      ],
    },
  ];

  const handleChange = (value: (string | number | null)[]) => {
    if (value.length === 0) {
      onChange("")
      return
    }
    onChange(value.map(String).join("/"))
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
