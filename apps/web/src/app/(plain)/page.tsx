'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useEffect, useState} from 'react';
import {getTokensMembers, getUserIdFromAccessToken, getUser, getExhibitor, User, Exhibitor, updateExhibitor} from "@/lib";
import {Footer, Header, Heading2, Heading1, MobileNavigator, Steps, Tab} from "@/components/generic";
import "../globals.css";
import styles from "./page.module.css";
import {topPageData} from "@/lib/lib";
import {Hero} from "@/components/Hero/Hero";
import {ExhibitorCard} from "@/components/exhibitor/ExhibitorCard/ExhibitorCard";
import {Modal} from "@/components/generic/Modal/Modal";
import {TextInput} from "@/components/generic/TextInput/TextInput";
import { set } from 'react-hook-form';

export default function Page() {
  const [authenticated, setAuthenticated] = useState(false);
  const [scrollY, setScrollY] = useState(0);
  const [innerHeight, setInnerHeight] = useState(100);
  const [user, setUser] = useState<User | null>(null);
  const [exhibitor, setExhibitor] = useState<Exhibitor | null>(null)
  const [representativeIndex, setRepresentativeIndex] = useState(null);
  const [modal, setModal] = useState(false);

  const setExhibitionName = (name: string) => {
    setExhibitor({ ...exhibitor, exhibition_name: name });
  };

  const setDescription = (description: string) => {
    setExhibitor({ ...exhibitor, description });
  };
  
  const closeModal = async () => {
    // モーダルを閉じる前に、変更があれば保存する
    if (exhibitor) {
      setModal(false); // 閉じる
      await updateExhibitor(exhibitor);
      
    }
  }

  useEffect(() => {
    (async () => {
      const tokens = await getTokensMembers()
      if (tokens) {
        setAuthenticated(true);
      }
    })();
  }, []);

  useEffect(() => {
    const handleScroll = () => {
      setScrollY(window.scrollY)
      setInnerHeight(window.innerHeight)
    }

    window.addEventListener("scroll", handleScroll)
  }, [])

  useEffect(() => {
    if (user?.exhibition_id) {
      getExhibitor(user.exhibition_id).then(setExhibitor);
    }
  }, [user]);

  useEffect(() => {
    (async () => {
      const userId = await getUserIdFromAccessToken()
      console.log("userId", userId);
      if (userId) {
        getUser(userId).then(setUser);
      }
    })();
  }, []);

  useEffect(() => {
    if (exhibitor && user?.id) {
      const index = exhibitor.representatives.indexOf(user?.id);
      setRepresentativeIndex(index + 1);
    }
  }, [exhibitor, user]);


  return (
    <>
      {authenticated ? (
        <>
          <main className="content">
            <Header header_type="members"></Header>
            <div className={styles.user}>
              <h1>こんにちは、{user?.last_name} {user?.first_name} 👋</h1>
              <h2>あなたは{exhibitor?.exhibitor_name}の第{representativeIndex}責任者です。</h2>
            </div>
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
                label="企画名"
                value={exhibitor?.exhibition_name || ""}
                setValue={setExhibitionName}
                paragraph={false}
              />
              <TextInput
                label="企画内容"
                value={exhibitor?.description || ""}
                setValue={setDescription}
                paragraph={true}
              />
            </Modal>
          </main>
          <Footer />
        </>
      ) : (
        <>
          <main className="content">
            <Header header_type="members" titleColor={scrollY > innerHeight - 80 ? "black" : "white"}></Header>
            <Hero />
            <Heading2 emoji={"ℹ️"}>
              このサイトについて
            </Heading2>
            <p>
              このサイトは工大祭実行委員会公式の参加団体向けポータルサイトです。<br/>
              このサイトを通じて工大祭への参加に関する各種手続きを行うことができます。<br/>
              一緒に工大祭を創りあげましょう！
            </p>
            <Tab tabs={
              new Map([
                [
                  "模擬店企画",
                  <>
                    <Heading2 emoji={"🕺"}>
                      参加申請までの流れ
                    </Heading2>
                    <Steps
                      steps={
                        topPageData.booth[0]
                      }
                    />
                    <Heading2 emoji={"📅"}>
                      工大祭までの流れ
                    </Heading2>
                    <Steps
                      steps={
                        topPageData.booth[1]
                      }
                    />
                  </>
                ],
                [
                  "一般企画",
                  <>
                    <Heading2 emoji={"🕺"}>
                      参加申請までの流れ
                    </Heading2>
                    <Steps
                      steps={
                        topPageData.general[0]
                      }
                    />
                    <Heading2 emoji={"📅"}>
                      工大祭までの流れ
                    </Heading2>
                    <Steps
                      steps={
                        topPageData.general[1]
                      }
                    />
                  </>
                ],
                [
                  "ステージ企画",
                  <>
                    <Heading2 emoji={"🕺"}>
                      参加申請までの流れ
                    </Heading2>
                    <Steps
                      steps={
                        topPageData.stage[0]
                      }
                    />
                    <Heading2 emoji={"📅"}>
                      工大祭までの流れ
                    </Heading2>
                    <Steps
                      steps={
                        topPageData.stage[1]
                      }
                    />
                  </>
                ],
                [
                  "研究室企画",
                  <>
                    <Heading2 emoji={"📅"}>
                      工大祭までの流れ
                    </Heading2>
                    <Steps
                      steps={
                        topPageData.labo[0]
                      }
                    />
                  </>
                ],
              ])
            } />
            <MobileNavigator header_type="members"/>
          </main>
          <Footer />
        </>
      )}
    </>
  );
}