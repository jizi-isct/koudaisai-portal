'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useEffect, useState} from 'react';
import {getTokensMembers, getUserIdFromAccessToken, getUser} from "@/lib";
import {Footer, Header, Heading2, Heading1, MobileNavigator, Steps, Tab} from "@/components/generic";
import "../globals.css";
import {topPageData} from "@/lib/lib";
import {Hero} from "@/components/Hero/Hero";
import { set } from 'react-hook-form';

export default function Page() {
  const [authenticated, setAuthenticated] = useState(false);
  const [scrollY, setScrollY] = useState(0);
  const [innerHeight, setInnerHeight] = useState(100);
  const [user, setUser] = useState(null);

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
    (async () => {
      const userId = await getUserIdFromAccessToken()
      console.log("userId", userId);
      if (userId) {
        getUser(userId).then(setUser);
      }
    })();
  }, []);

  return (
    <>
      {authenticated ? (
        <>
          <main className="content">
            <Header header_type="members"></Header>
            <h1>こんにちは、{user?.last_name} {user?.first_name} 👋</h1>
            <h2>あなたはU7374の第2責任者です。</h2>
            <Heading1 emoji={"📄"}>
              企画情報
            </Heading1>
            <p>
              このページは工大祭実行委員会のメンバー専用のページです。<br/>
              メンバー以外の方は<a href="/">トップページ</a>からご覧ください。
            </p>
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