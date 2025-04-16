'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useEffect, useState} from 'react';
import {getTokensMembers} from "@koudaisai-portal/util";
import {Heading, Steps, Tab} from "@koudaisai-portal/ui-generic";
import Link from "next/link";
import {topPageData} from "@/lib/lib";

export default function Page() {
  const [authenticated, setAuthenticated] = useState(false);

  useEffect(() => {
    (async () => {
      const tokens = await getTokensMembers()
      if (tokens) {
        setAuthenticated(true);
      }
    })();
  }, []);

  return (
    <main>
      {authenticated ? (
        <h1>ログイン済みです</h1>
      ) : (
        <>
          <Heading emoji={"ℹ️"}>
            このサイトについて
          </Heading>
          <p>
            このサイトは工大祭実行委員会公式の参加団体向けポータルサイトです。<br/>
            このサイトを通じて工大祭への参加に関する各種手続きを行うことができます。<br/>
            一緒に工大祭を創りあげましょう！
          </p>
          <Tab tabs={
            new Map([
              [
                "模擬企画",
                <>
                  <Heading emoji={"🕺"}>
                    参加申請までの流れ
                  </Heading>
                  <Steps
                    steps={
                      topPageData.booth[0]
                    }
                  />
                  <Heading emoji={"📅"}>
                    工大祭までの流れ
                  </Heading>
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
                  <Heading emoji={"🕺"}>
                    参加申請までの流れ
                  </Heading>
                  <Steps
                    steps={
                      topPageData.general[0]
                    }
                  />
                  <Heading emoji={"📅"}>
                    工大祭までの流れ
                  </Heading>
                  <Steps
                    steps={
                      topPageData.general[1]
                    }
                  />
                </>
              ],
              [
                "研究室企画",
                <>
                  <Heading emoji={"🕺"}>
                    参加申請までの流れ
                  </Heading>
                  <Steps
                    steps={
                      topPageData.labo[0]
                    }
                  />
                </>
              ],
              [
                "ステージ企画",
                <>
                  <Heading emoji={"🕺"}>
                    参加申請までの流れ
                  </Heading>
                  <Steps
                    steps={
                      topPageData.stage[0]
                    }
                  />
                  <Heading emoji={"📅"}>
                    工大祭までの流れ
                  </Heading>
                  <Steps
                    steps={
                      topPageData.stage[1]
                    }
                  />
                </>
              ],
            ])
          } />

        </>
      )}
    </main>
  );
}