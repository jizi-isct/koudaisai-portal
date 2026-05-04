import {Content} from "../lib/types";

type Props = {
  content: Content
}

export function ContentRow({content}: Props) {
  return (
    <div className="flex flex-row items-center h-10 px-5 cursor-pointer gap-5 text-[15px]" onClick={content.onClick}>
      <span className="font-light">{content.date}</span>
      <span className="grow">{content.title}</span>
      <span>{content.author}</span>
    </div>
  )
}
