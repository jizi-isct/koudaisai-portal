'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useEffect, useState, useCallback} from 'react';

import {getUser, getExhibitor, User, Exhibitor, updateExhibitor} from "@/lib";
import {Footer, Header, Heading1} from "@/components/generic";
import {ExhibitorCard} from "@/components/exhibitor/ExhibitorCard/ExhibitorCard";
import {Modal} from "@/components/generic/Modal/Modal";
import {TextInput} from "@/components/generic/TextInput/TextInput";
import {FileUploader} from "@/components/common/FileUploader";
import {UserInfoCard} from "@/components/UserInfoCard/UserInfoCard";

export const TopPageAuthenticated = () => {

  const [user, setUser] = useState<User | null>(null);
  const [exhibitor, setExhibitor] = useState<Exhibitor | null>(null)
  const [modal, setModal] = useState(false);

  
  useEffect(() => {
    if (user?.exhibition_id) {
      getExhibitor(user.exhibition_id).then(setExhibitor);
    }
  }, [user]);

  useEffect(() => {
    getUser().then(setUser);
  }, []);

  const handleFileUpload = useCallback(async (fileKey: string, fileName: string) => {
    if (!exhibitor) return;
    setExhibitor({ ...exhibitor, icon_id: fileKey });
  }, [exhibitor]);

  const closeModal = async () => {
    // モーダルを閉じる前に、変更があれば保存する
    if (exhibitor) {
      setModal(false); // 閉じる
      await updateExhibitor(exhibitor);
      
    }
  }  
  
  return (
    <>
      <main className="content">
            <Header header_type="members"></Header>
            <UserInfoCard user={user} exhibitor={exhibitor} />
            <Heading1 emoji={"📄"}>
              企画情報
            </Heading1>
            <ExhibitorCard
              exhibitor={exhibitor}
              openModal={() => setModal(true)}
            />
            <Modal
              isOpen={modal}
              setOpen={closeModal}
            >
              <TextInput
                value={exhibitor?.exhibition_name || ""}
                setValue={(value) => {
                  setExhibitor(prev => prev ? { ...prev, exhibition_name: value } : prev);
                }}
                paragraph={false}
              />
              <TextInput
                value={exhibitor?.description || ""}
                setValue={(value) => {
                  setExhibitor(prev => prev ? { ...prev, description: value } : prev);
                }}
                paragraph={true}
              />
              <FileUploader callback={handleFileUpload} isMembers={true}/>
            </Modal>
          </main>
          <Footer />
    </>
  );
};

export default TopPageAuthenticated;