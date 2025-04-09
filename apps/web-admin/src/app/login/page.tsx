'use client';
import {useSearchParams} from "next/navigation";
import {$auth} from '@koudaisai-portal/util';
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Suspense} from "react";

export default function Login() {
    return (
        <Suspense>
            <QueryClientProvider client={new QueryClient()}>
                <Inner/>
            </QueryClientProvider>
        </Suspense>
    );
}

function Inner() {
  const search = useSearchParams()
    const code = search.get("code")!
    const state = search.get("state")!
    const {data, error} = $auth.useQuery(
        "post",
        "/admin/redirect",
        {
            body: {
                code: code,
                state: state
            }
        }
    )

    if (data) {
        localStorage.setItem("admin_refresh_token", data!.refresh_token)
        localStorage.setItem("admin_access_token", data!.access_token)
        window.location.assign("/admin")
    }

    return (
        <div>
            <h1>ログイン</h1>
            {error && <p style={{color: 'red'}}>{error}</p>}
            {!data && !error && <p>ログイン中...</p>}
        </div>
    );
}