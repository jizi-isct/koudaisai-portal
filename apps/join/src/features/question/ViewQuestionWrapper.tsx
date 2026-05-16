'use client'

import {questionDataNoLogin} from "../../features/question/questionDataNoLogIn";
import {Faq, Selector} from "@koudaisai/shared-ui";
import {useState} from "react";

export default function ViewQuestionWrapper() {
  const first = Object.keys(questionDataNoLogin)[0];
  const [questionType, setQuestionType] = useState(first);

  return (
    <>
      <Selector
        options={Object.keys(questionDataNoLogin)}
        selectedOption={questionType}
        setOption={setQuestionType}
      />
      {questionDataNoLogin[questionType].map((data, i) => (
        <Faq key={i+1} number={i+1} content={data}/>
      ))}
      <p>その他何かご不明などございましたら、当委員会までお気軽にお問い合わせください！</p>
    </>
  );
}