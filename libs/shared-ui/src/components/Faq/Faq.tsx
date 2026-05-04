type Props = {
  number: number
  content:
  {
    question: string
    answer: string
  }
}

export function Faq({number, content}: Props) {
  return (
    <div className="w-full border border-darkblue rounded-[10px] py-[0.5em] px-[15px] shadow-[4px_4px_0_0_var(--darkblue)] my-[2em] bg-white">
      <p className="w-[90%] mx-auto my-5 relative text-[15px]">
        <span className="inline-block text-darkgray font-bold text-[20px] mr-2">Q{number}. </span>
        {content.question}
      </p>
      <p className="w-[90%] mx-auto my-5 relative text-[15px]">
        <span className="inline-block text-darkgray font-bold text-[20px] mr-2">A{number}. </span>
        {content.answer}
      </p>
    </div>
  )
}
