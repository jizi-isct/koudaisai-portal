'use client';

import {Heading1, Question} from "@koudaisai-portal/ui-generic";
import {useState} from "react";
import {questionDataNoLogin} from "@/lib/lib";
import { get } from "http";

export default function Page() {
    
    return (
        <>
            <Heading1 emoji={"❓"}>
                よくある質問
            </Heading1>
            {
            questionDataNoLogin["研究室公開企画"]
            .map((data, i) => (
                <Question key={i} content={data} />
            ))
            }
        </>
    );
}