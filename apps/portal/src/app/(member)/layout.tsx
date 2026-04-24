"use client";
import {getTokensMembers, logout} from "@koudaisai/shared-auth-members";
import {Footer, LoadingScreen} from "@koudaisai/shared-ui";
import {Header} from "@/components/Header";
import {ReactNode, useCallback, useEffect, useState} from "react";
import {authFetchClient} from "@/lib/api";
import type {Tokens} from "@koudaisai/shared-auth";
import {useRouter} from "next/navigation";

export default function MemberLayout({children}: {children: ReactNode}) {
  const [tokens, setTokens] = useState<Tokens | null | undefined>();
  const router = useRouter();

  useEffect(() => {
    (async () => {
      const t = await getTokensMembers(authFetchClient);
      if (t) {
        setTokens(t);
      } else {
        window.location.assign("/login");
      }
    })();
  }, []);

  const handleLogout = useCallback(() => {
    logout();
    router.push("/login");
  }, [logout, router])

  return (
    <>
      <Header
        logout = {async () => {handleLogout();}}
      />
      <main className="content">
        {!tokens ? <LoadingScreen/> : children}
      </main>
      {/*<MobileNavigator*/}
      {/*  header_type="members"*/}
      {/*  logout = {async () => {logout();}}*/}
      {/*  isLoggedIn={!!tokens}*/}
      {/*/>*/}
      <Footer/>
    </>
  );
}