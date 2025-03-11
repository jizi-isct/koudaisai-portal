import {Form} from "@/lib/api";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";

type Props = {
  form: Form
  setForm: (form: Form) => void,
}
export default function FormMetadata({form, setForm}: Props) {
  const handleDateChange = (isoTime?: string) => {
    setForm(
      {
        ...form,
        info: {
          ...form.info,
          deadline: isoTime
        }
      }
    )
  }

  const handleDeadlineSet = () => {
    setForm(
      {
        ...form,
        info: {
          ...form.info,
          deadline: new Date().toISOString()
        }
      }
    )
  }

  const handleDeadlineReset = () => {
    setForm(
      {
        ...form,
        info: {
          ...form.info,
          deadline: undefined
        }
      }
    )
  }
  type RoleTypes = "none" | "BOOTH" | "GENERAL" | "STAGE" | "LABO";
  const handleAccessControl = (value: RoleTypes, checked: boolean) => {
    let roles = new Set(form.access_control.roles);
    checked ? roles.add(value) : roles.delete(value);
    setForm(
      {
        ...form,
        access_control: {
          roles: roles.values().toArray()
        }
      }
    )
  }

  return (
    <div>
      ID：{form.form_id}<br/>
      作成日時：{form.created_at} <br/>
      最終更新日時：{form.updated_at} <br/>
      <div className="datepicker">
        回答締め切り：
        {form.info.deadline === undefined ?
          <>
            なし
            <button onClick={handleDeadlineSet}>設定する</button>
          </>
          :
          <>
            <DatePicker
              locale={"ja"}
              dateFormat="yyyy-MM-dd'T'HH:mm:ss.SSSSSS'Z'"
              selected={new Date(Date.parse(form.info.deadline!))}
              onChange={(e) => handleDateChange(e?.toISOString())}
              showTimeSelect
            />
            <button onClick={handleDeadlineReset}>締め切りをなくす</button>
          </>

        }
      </div>
      <div>
        アクセスコントロール-ロール(チェックした対象が閲覧可能になります)：<br/>
        &emsp;&emsp;&emsp;<input type="checkbox" name="非ログイン" value="none"
                                 defaultChecked={form.access_control.roles.includes("none")}
                                 onChange={(e) => handleAccessControl(e.target.value as RoleTypes, e.target.checked)}/>非ログイン
        &#8201;<input type="checkbox" name="模擬店" value="BOOTH"
                      defaultChecked={form.access_control.roles.includes("BOOTH")}
                      onChange={(e) => handleAccessControl(e.target.value as RoleTypes, e.target.checked)}/>模擬店
        &#8201;<input type="checkbox" name="一般" value="GENERAL"
                      defaultChecked={form.access_control.roles.includes("GENERAL")}
                      onChange={(e) => handleAccessControl(e.target.value as RoleTypes, e.target.checked)}/>一般
        &#8201;<input type="checkbox" name="ステージ" value="STAGE"
                      defaultChecked={form.access_control.roles.includes("STAGE")}
                      onChange={(e) => handleAccessControl(e.target.value as RoleTypes, e.target.checked)}/>ステージ
        &#8201;<input type="checkbox" name="研究室" value="LABO"
                      defaultChecked={form.access_control.roles.includes("LABO")}
                      onChange={(e) => handleAccessControl(e.target.value as RoleTypes, e.target.checked)}/>研究室
      </div>
    </div>
  );
}