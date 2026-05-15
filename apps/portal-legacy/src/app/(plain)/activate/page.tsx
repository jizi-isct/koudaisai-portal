"use client";

import {Suspense, useState} from "react";
import {useSearchParams} from "next/navigation";
import {ActivationPhaseFirst} from "@/components/activation/ActivationPhaseFirst";
import "../../globals.css";
import "../../members.css"
import {ActivationPhaseMAddress} from "@/components/activation/ActivationPhaseMAddress";
import {ActivationPhasePassword} from "@/components/activation/ActivationPhasePassword";
import {ActivationPhaseActivating} from "@/components/activation/ActivationPhaseActivating";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {Result} from "antd";
import {ActivationPhaseDone} from "@/components/activation/ActivationPhaseDone";

type ActivationPhase = "first" | "m_address" | "password" | "activating" | "done";

export default function Page() {
  return (
    <Suspense>
      <Inner1/>
    </Suspense>
  )
}

function Inner1() {
  const params = useSearchParams()
  const token = params.get("token")
  if (token === null) {
    return (
      <main>
        <Result
          status="error"
          title="クエリパラメータに不足があります"
          subTitle="tokenが指定されていません。URLに?token=xxxxのように指定してください。"
        />
      </main>
    )
  }
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner2 token={token}/>
    </QueryClientProvider>
  )
}

export function Inner2({token}: { token: string }) {
  const [phase, setPhase] = useState<ActivationPhase>("first")
  const [mAddress, setMAddress] = useState<string>("")
  const [password, setPassword] = useState<string>("")

  const handleMAddressChange = (address: string) => {
    setMAddress(address);
    setPhase("password")
  }

  const handlePasswordChange = (password: string) => {
    setPassword(password);
    setPhase("activating")
  }

  const handleActivationDone = () => {
    setPhase("done");
  }

  switch (phase) {
    case "first":
      return <ActivationPhaseFirst next={() => setPhase("m_address")}/>
    case "m_address":
      return <ActivationPhaseMAddress next={handleMAddressChange}/>
    case "password":
      return <ActivationPhasePassword next={handlePasswordChange} mAddress={mAddress}/>
    case "activating":
      return <ActivationPhaseActivating next={handleActivationDone} mAddress={mAddress} password={password}
                                        token={token}/>
    case "done":
      return <ActivationPhaseDone m_address={mAddress} password={password}/>
  }
}