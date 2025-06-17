'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useEffect, useState} from 'react';
import {getUser, getExhibitor, User, Exhibitor} from "@/lib";
import {Footer, Header, Heading1} from "@/components/generic";
import {ExhibitorCard} from "@/components/exhibitor/ExhibitorCard/ExhibitorCard";
import {UserInfoCard} from "@/components/UserInfoCard/UserInfoCard";
import {EditModal} from "@/components/exhibitor/EditModal/EditModal";

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
            <EditModal
              user={user}
              exhibitor={exhibitor}
              setExhibitor={setExhibitor}
              modal={modal}
              setModal={setModal}
            />
          </main>
          <Footer />
    </>
  );
};

export default TopPageAuthenticated;