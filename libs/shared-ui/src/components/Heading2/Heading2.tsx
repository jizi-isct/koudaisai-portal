import {ReactNode} from "react";

type Props = {
  children: ReactNode,
  emoji: string
}

export function Heading2({children, emoji}: Props) {
  return (
    <h1 className="flex relative items-center max-[700px]:justify-center">
      <span className="relative text-[32px] pr-2">
        <span className="-z-[5] absolute text-[3em] opacity-5 -top-16 -left-12">{emoji}</span>
        {emoji}
      </span>
      <div className="drop-shadow-[0_0_4px_white] text-2xl flex items-center gap-[0.3em]">{children}</div>
    </h1>
  )
}
