'use client';

import {Faq, Heading1, LoadingScreen, Selector} from "@koudaisai/shared-ui";
import {Suspense, useEffect, useMemo, useState} from "react";
import { questionDataNoLogin,questionDataPlanBooth,questionDataPlanGeneral,questionDataPlanStage } from "@/lib/questionData";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {$apiMembers} from "@/lib/api";
import { UserRead } from "@koudaisai/shared-types";

export default function Page() {
  const [qc] = useState(() => new QueryClient());
  return (
    <Suspense fallback={<LoadingScreen/>}>
      <QueryClientProvider client={qc}>
        <Inner/>
      </QueryClientProvider>
    </Suspense>
  );
}

function Inner() {
  // ログイン時だけユーザ情報を取得（Suspenseで待つ）
  return (
    <Suspense fallback={<LoadingScreen/>}>
      <LoggedInGate/>
    </Suspense>
  );
}

function LoggedInGate() {
  const {data: user} = $apiMembers.useQuery("get", "/users/{user_id}", {
    params: {path: {user_id: "me"}},
    suspense: true,
  });

  // Suspense中は上のfallbackが出るのでここに来る時は基本 user があるはず
  if (!user) return <LoadingScreen/>;
  return <LoggedIn user={user}/>;
}

function LoggedIn({user}: { user: UserRead }) {
  const {data: group} = $apiMembers.useQuery("get", "/groups/{id}", {
    params: {path: {id: user.group_id}},
  });

  const questionData = useMemo(() => {
    if (group?.type_plan?.type_booth) return questionDataPlanBooth;
    if (group?.type_plan?.type_general) return questionDataPlanGeneral;
    if (group?.type_plan?.type_stage) return questionDataPlanStage;
    return questionDataNoLogin; // 念のため
  }, [group]);

  // 初期値は現在の questionData に同期
  const [questionType, setQuestionType] = useState(
    () => Object.keys(questionData)[0]
  );

  // questionData が切り替わったら先頭キーに合わせてリセット
  useEffect(() => {
    const first = Object.keys(questionData)[0];
    setQuestionType(first);
  }, [questionData]);

  // questionType がまだ空の瞬間を安全にやり過ごす
  const items = questionType ? questionData[questionType] : [];

  return (
    <>
      <Heading1 emoji="❓">よくある質問</Heading1>
      <Selector
        options={Object.keys(questionData)}
        selectedOption={questionType}
        setOption={setQuestionType}
      />
      {items?.map((data, i) => (
        <Faq key={i} number={i} content={data}/>
      ))}
      <p>その他何かご不明などございましたら、当委員会までお気軽にお問い合わせください！</p>
    </>
  );
}