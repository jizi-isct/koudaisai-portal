'use client'; // クライアントサイドコンポーネントとして実行するために追加

import React from "react";
import {$apiMembers} from "@/lib";
import {Modal} from "@/components/generic/Modal/Modal";
import {TextInput} from "@/components/generic/TextInput/TextInput";
import {FileUploader} from "@/components/common/FileUploader";
import {ButtonCompact} from "@/components/generic/ButtonCompact";
import {Heading1} from "@/components/generic";


type EditModalProps = {
  initPlanName: string;
  initDescription: string;
  initIsChildFriendly: boolean;
  modal: boolean;
  setModal: (isOpen: boolean) => void;
};

export const EditModal = (
  {
    initPlanName,
    initDescription,
    initIsChildFriendly,
    modal,
    setModal
  }: EditModalProps) => {
  const [planName, setPlanName] = React.useState<string>(initPlanName);
  const [description, setDescription] = React.useState<string>(initDescription);
  const [isChildFriendly, setIsChildFriendly] = React.useState<boolean>(initIsChildFriendly);
  const [iconKey, setIconKey] = React.useState<string | undefined>();
  const {mutateAsync: createApprovalRequest} = $apiMembers.useMutation("post", "/users/{user_id}/approval_requests")

  const handleSubmit = async () => {
    let reqPlanName: string | undefined = planName;
    let reqDescription: string | undefined = description;
    let reqIsChildFriendly: boolean | undefined = isChildFriendly;

    if (planName === initPlanName) {
      reqPlanName = undefined;
    }
    if (description === initDescription) {
      reqDescription = undefined;
    }
    if (isChildFriendly === reqIsChildFriendly) {
      reqIsChildFriendly = undefined;
    }

    // 申請
    await createApprovalRequest(
      {
        params: {
          path: {
            user_id: "me"
          }
        },
        body: {
          type_edit_exhibition_info: {
            plan_name: reqPlanName,
            description: reqDescription,
            is_child_friendly: reqIsChildFriendly,
            icon_key: iconKey
          }
        }
      }
    )
    setModal(false)
  }

  return (
    <Modal
      isOpen={modal}
      setOpen={setModal}
    >
      <Heading1 emoji={"📝"}>企画情報の訂正</Heading1>

      <label>
        企画名
        <TextInput
          value={planName}
          setValue={setPlanName}
          paragraph={false}
        />
      </label>
      <label>
        企画概要
        <TextInput
          value={description}
          setValue={setDescription}
          paragraph={true}
        />
      </label>
      <label>
        子供向け企画か否か<br/>
        <input
          type="checkbox"
          checked={isChildFriendly}
          onChange={(e) => setIsChildFriendly(e.target.checked)}
        />
      </label><br/>
      <label>
        アイコン画像
        <FileUploader fileType={"image/*"} callback={(key, _) => setIconKey(key)} client={$apiMembers}/>
      </label>
      <ButtonCompact text={"企画情報の訂正を申請する"} onClick={handleSubmit}/>
    </Modal>
  );
};