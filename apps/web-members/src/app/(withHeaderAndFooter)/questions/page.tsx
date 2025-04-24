'use client';

import {useEffect, useState} from 'react';
import {getTokensMembers} from "@koudaisai-portal/util";
import {Heading1} from "@koudaisai-portal/ui-generic";

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
        <>
            <Heading1 emoji={"❓"}>
                よくある質問
            </Heading1>
        </>
    );
}