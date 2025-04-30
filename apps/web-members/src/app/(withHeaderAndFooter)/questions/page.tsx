'use client';

import {Heading1, Faq, Selector} from "@koudaisai-portal/ui-generic";
import {useState} from "react";
import {questionDataNoLogin} from "@/lib/lib";

import "@koudaisai-portal/ui-generic/css"

export default function Page() {
    const firstQuestionType = Object.keys(questionDataNoLogin)[0];
    const [questionType, setQuestionType] = useState(firstQuestionType);
    return (
        <>
            <Heading1 emoji={"❓"}>
                よくある質問
            </Heading1>
            <Selector
                options={Object.keys(questionDataNoLogin)}
                selectedOption={questionType}
                setOption={setQuestionType} />
            {
            questionDataNoLogin[questionType]
            .map((data, i) => (
                <Faq key={i} content={data} />
            ))
            }
            <p>その他何かご不明などございましたら、当委員会までお気軽にお問い合わせください！</p>
        </>
    );
}