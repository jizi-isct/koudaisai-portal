"use client";

import "./globals.css";
import {getTokensAdmin, Tokens} from "@/lib";
import {ReactNode, useEffect, useState} from "react";

type Props = {
  children: ReactNode
}

export function Inner({children}: Props) {
  const [tokens, setTokens] = useState<Tokens | null | undefined>();
  useEffect(() => {
    (async () => {
      const tokens = await getTokensAdmin()
      if (tokens) {
        setTokens(tokens)
      } else {
        window.location.assign(process.env.NEXT_PUBLIC_AUTH_BASE_URL + "/admin/login")
      }
    })()
  }, [])
  if (tokens) {
    return (
      <>
        {children}
      </>
    );
  } else {
    return <>Loading...</>
  }
}