'use client';

import styles from "../documents/page.module.css";
import {Faq, Heading1, Selector} from "@koudaisai/shared-ui";
import {useState} from "react";
import {questionDataNoLogin} from "./QuestionDataNoLogin"

export default function Page() {
  const first = Object.keys(questionDataNoLogin)[0];
  const [questionType, setQuestionType] = useState(first);
  return (
    <>
    <main className={styles.main}>
      <Heading1 emoji="❓">よくある質問</Heading1>
      <div className={styles.container}>
      <Selector
        options={Object.keys(questionDataNoLogin)}
        selectedOption={questionType}
        setOption={setQuestionType}
      />
      {questionDataNoLogin[questionType].map((data, i) => (
        <Faq key={i} number={i+1} content={data}/>
      ))}
      <p>その他何かご不明などございましたら、当委員会までお気軽にお問い合わせください！</p>
    </div>
    </main>
    </>
  );
}