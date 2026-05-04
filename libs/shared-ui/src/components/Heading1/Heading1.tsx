import {ReactNode} from "react";

type Props = {
  children: ReactNode,
  emoji: string
}

export function Heading1({children, emoji}: Props) {
  return (
    <h1 className="text-center my-[1em]">
      <span className="text-[32px] pr-2">{emoji}</span>
      <span className="drop-shadow-[0_0_4px_white] text-[28px]">{children}</span>
    </h1>
  )
}
