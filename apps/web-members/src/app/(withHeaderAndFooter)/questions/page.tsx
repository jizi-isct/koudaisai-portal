'use client';

import {Heading1, Question, Selector} from "@koudaisai-portal/ui-generic";
import {useState} from "react";
import {questionDataNoLogin} from "@/lib/lib";

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
                slectedOption={questionType}
                setOption={setQuestionType} />
            {
            questionDataNoLogin[questionType]
            .map((data, i) => (
                <Question key={i} content={data} />
            ))
            }
        </>
    );
}