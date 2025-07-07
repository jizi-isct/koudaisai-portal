'use client'; // クライアントサイドコンポーネントとして実行するために追加

import React, {useCallback} from "react";
import {$apiMembers, ExhibitionUpdate, User} from "@/lib";
import {Modal} from "@/components/generic/Modal/Modal";
import {TextInput} from "@/components/generic/TextInput/TextInput";
import {FileUploader} from "@/components/common/FileUploader";
import {ButtonCompact} from "@/components/generic/ButtonCompact";


type EditModalProps = {
  user: User;
  modal: boolean;
  setModal: (isOpen: boolean) => void;
};

export const EditModal = ({modal, setModal}: EditModalProps) => {
  const [exhibitor, setExhibitor] = React.useState<ExhibitionUpdate>({});
  const handleFileUpload = useCallback(async (fileKey: string) => {
    if (!exhibitor) return;
    setExhibitor({...exhibitor, icon_id: fileKey});
  }, [exhibitor]);
  const {mutateAsync: createApprovalRequest} = $apiMembers.useMutation("post", "/users/{user_id}/approval_requests")

  const handleSubmit = async () => {
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
            exhibition_name: exhibitor.exhibition_name,
            description: exhibitor.description,
            icon_id: exhibitor.icon_id
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
      <label>
        企画名
        <TextInput
          value={exhibitor?.exhibition_name || ""}
          setValue={(value) => {
            setExhibitor(exhibitor ? {...exhibitor, exhibition_name: value} : exhibitor);
          }}
          paragraph={false}
        />
      </label>
      <label>
        企画概要
        <TextInput
          value={exhibitor?.description || ""}
          setValue={(value) => {
            setExhibitor(exhibitor ? {...exhibitor, description: value} : exhibitor);
          }}
          paragraph={true}
        />
      </label>
      <label>
        アイコン画像
        <FileUploader callback={handleFileUpload} client={$apiMembers}/>
      </label>
      <ButtonCompact text={"企画情報の変更を申請する"} onClick={handleSubmit}/>
    </Modal>
  );
};
