'use client';

import {useEffect, useState} from 'react';
import {getTokensMembers} from "@koudaisai-portal/util";

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
                <h1>ログインしていません</h1>
            )}
        </main>
    );
}