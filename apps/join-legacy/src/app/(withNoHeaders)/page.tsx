'use client';

import {useEffect, useState} from 'react';
import {Footer, Header, Heading2, MobileNavigator, Steps, Tab} from '@koudaisai/shared-ui';
import {Hero} from '../../components/Hero';
import {topPageData} from '../../lib/StaticData';
import '../globals.css';
import '../members.css';

export default function Page() {

  const [scrollY, setScrollY] = useState(0);
  const [innerHeight, setInnerHeight] = useState(100);

  useEffect(() => {
    const handleScroll = () => {
      setScrollY(window.scrollY)
      setInnerHeight(window.innerHeight)
    }

    window.addEventListener("scroll", handleScroll)
    handleScroll()
    return () => {
      window.removeEventListener("scroll", handleScroll)
    }
  }, [])

  return (
    <html>
    <body>
    <main className="content">
      <Header
        titleColor={scrollY > innerHeight - 80 ? "black" : "white"}
      />
      <Hero/>
      <Heading2 emoji={"ℹ️"}>このサイトについて</Heading2>
      <p>
        このサイトは工大祭実行委員会公式の参加団体向けポータルサイトです。
        <br/>
        このサイトを通じて工大祭への参加に関する各種手続きを行うことができます。
        <br/>
        一緒に工大祭を創りあげましょう！
      </p>
      <Tab
        tabs={
          new Map([
            [
              "模擬店企画",
              <>
                <Heading2 emoji={"🕺"}>参加申請までの流れ</Heading2>
                <Steps steps={topPageData.booth[0]}/>
                <Heading2 emoji={"📅"}>工大祭までの流れ</Heading2>
                <Steps steps={topPageData.booth[1]}/>
              </>,
            ],
            [
              "一般企画",
              <>
                <Heading2 emoji={"🕺"}>参加申請までの流れ</Heading2>
                <Steps steps={topPageData.general[0]}/>
                <Heading2 emoji={"📅"}>工大祭までの流れ</Heading2>
                <Steps steps={topPageData.general[1]}/>
              </>,
            ],
            [
              "ステージ企画",
              <>
                <Heading2 emoji={"🕺"}>参加申請までの流れ</Heading2>
                <Steps steps={topPageData.stage[0]}/>
                <Heading2 emoji={"📅"}>工大祭までの流れ</Heading2>
                <Steps steps={topPageData.stage[1]}/>
              </>,
            ],
            [
              "研究室企画",
              <>
                <Heading2 emoji={"📅"}>工大祭までの流れ</Heading2>
                <Steps steps={topPageData.labo[0]}/>
              </>,
            ],
          ])
        }
      />
      <MobileNavigator
        header_type="members"
        logout={async () => Promise.resolve()}
      />
    </main>
    <Footer isLoggedIn={false}/>
    </body>
    </html>
  );
}