"use client";
import {getTokensMembers, logout} from "@koudaisai/shared-auth-members";
import {Footer, LoadingScreen} from "@koudaisai/shared-ui";
import {Header} from "@/components/Header";
import {MobileNavigator} from "@/components/MobileNavigator";
import {ReactNode, useEffect, useState} from "react";
import {authFetchClient} from "@/lib/api";
import type {Tokens} from "@koudaisai/shared-auth";

export default function MemberLayout({children}: {children: ReactNode}) {
  const [tokens, setTokens] = useState<Tokens | null | undefined>();

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

  return (
    <>
      <Header
        logout = {async () => {logout();}}
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