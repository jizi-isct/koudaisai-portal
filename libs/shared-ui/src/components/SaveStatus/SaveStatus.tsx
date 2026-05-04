import {saved, saving, unsaved} from "../icons/save_status";
import {SaveStatus as SaveStatus_} from "@koudaisai/shared-types";
import Element = React.JSX.Element;

type SaveStatusProps = {
  status: SaveStatus_;
};

export const SaveStatus = ({status}: SaveStatusProps) => {
  const statusIcons: Record<SaveStatus_, Element> = {
    saved: saved,
    unsaved: unsaved,
    saving: saving,
  };
  const statusText: Record<SaveStatus_, string> = {
    saved: "変更内容を保存しました",
    unsaved: "変更内容は保存されていません",
    saving: "変更内容を保存中",
  };

  return (
    <div className="flex items-center">
      {statusIcons[status]}
      <h6 className="text-gray text-xs font-medium leading-normal tracking-[0.44px] ml-[10px]">{statusText[status]}</h6>
    </div>
  );
};