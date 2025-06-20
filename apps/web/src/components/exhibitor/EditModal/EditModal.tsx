'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useCallback} from "react";
import {$apiMembers, Exhibitor, updateExhibitor, User} from "@/lib";
import {Modal} from "@/components/generic/Modal/Modal";
import {TextInput} from "@/components/generic/TextInput/TextInput";
import {FileUploader} from "@/components/common/FileUploader";


type EditModalProps = {
  user: User;
  exhibitor: Exhibitor;
  setExhibitor: (exhibitor: Exhibitor | null) => void;
  modal: boolean;
  setModal: (isOpen: boolean) => void;
};

export const EditModal = ({exhibitor, setExhibitor, modal, setModal}: EditModalProps) => {
  const handleFileUpload = useCallback(async (fileKey: string) => {
    if (!exhibitor) return;
    setExhibitor({...exhibitor, icon_id: fileKey});
  }, [exhibitor]);

  const closeModal = async () => {
    // モーダルを閉じる前に、変更があれば保存する
    if (exhibitor) {
      setModal(false); // 閉じる
      await updateExhibitor(exhibitor);

    }
  }

  return (
    <Modal
      isOpen={modal}
      setOpen={closeModal}
    >
      <TextInput
        value={exhibitor?.exhibition_name || ""}
        setValue={(value) => {
          setExhibitor(exhibitor ? {...exhibitor, exhibition_name: value} : exhibitor);
        }}
        paragraph={false}
      />
      <TextInput
        value={exhibitor?.description || ""}
        setValue={(value) => {
          setExhibitor(exhibitor ? {...exhibitor, description: value} : exhibitor);
        }}
        paragraph={true}
      />
      <FileUploader callback={handleFileUpload} client={$apiMembers}/>
    </Modal>
  );
};
