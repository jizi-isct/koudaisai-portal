'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useEffect, useState} from 'react';
import {Footer, Header, Heading2, MobileNavigator, Steps, Tab} from "@/components/generic";
import {topPageData} from "@/lib/lib";
import {Hero} from "@/components/Hero/Hero";

export const TopPageUnauthenticated = () => {
    const [scrollY, setScrollY] = useState(0);
    const [innerHeight, setInnerHeight] = useState(100);
    
    useEffect(() => {
        const handleScroll = () => {
          setScrollY(window.scrollY)
          setInnerHeight(window.innerHeight)
        }
    
        window.addEventListener("scroll", handleScroll)
      }, [])

    return (
    <>
      <main className="content">
        <Header
          header_type="members"
          titleColor={scrollY > innerHeight - 80 ? "black" : "white"}
        ></Header>
        <Hero />
        <Heading2 emoji={"ℹ️"}>このサイトについて</Heading2>
        <p>
          このサイトは工大祭実行委員会公式の参加団体向けポータルサイトです。
          <br />
          このサイトを通じて工大祭への参加に関する各種手続きを行うことができます。
          <br />
          一緒に工大祭を創りあげましょう！
        </p>
        <Tab
          tabs={
            new Map([
              [
                "模擬店企画",
                <>
                  <Heading2 emoji={"🕺"}>参加申請までの流れ</Heading2>
                  <Steps steps={topPageData.booth[0]} />
                  <Heading2 emoji={"📅"}>工大祭までの流れ</Heading2>
                  <Steps steps={topPageData.booth[1]} />
                </>,
              ],
              [
                "一般企画",
                <>
                  <Heading2 emoji={"🕺"}>参加申請までの流れ</Heading2>
                  <Steps steps={topPageData.general[0]} />
                  <Heading2 emoji={"📅"}>工大祭までの流れ</Heading2>
                  <Steps steps={topPageData.general[1]} />
                </>,
              ],
              [
                "ステージ企画",
                <>
                  <Heading2 emoji={"🕺"}>参加申請までの流れ</Heading2>
                  <Steps steps={topPageData.stage[0]} />
                  <Heading2 emoji={"📅"}>工大祭までの流れ</Heading2>
                  <Steps steps={topPageData.stage[1]} />
                </>,
              ],
              [
                "研究室企画",
                <>
                  <Heading2 emoji={"📅"}>工大祭までの流れ</Heading2>
                  <Steps steps={topPageData.labo[0]} />
                </>,
              ],
            ])
          }
        />
        <MobileNavigator header_type="members" />
      </main>
      <Footer />
    </>
  );
};

export default TopPageUnauthenticated;