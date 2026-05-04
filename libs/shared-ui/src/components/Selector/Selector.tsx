type Props = {
  options: string[],
  selectedOption: string,
  setOption: (value: string) => void
}

export function Selector({options, selectedOption, setOption}: Props) {
  return (
    <div className="w-full flex justify-center">
      <div className="border border-darkblue rounded-[10px] px-[14px] py-[10px] bg-white inline-block">
        {
          options.map((option, i) => (
            <div
              key={i}
              className={`cursor-pointer text-sm inline-block px-3 ${selectedOption === option ? "font-bold" : ""}`}
              onClick={() => setOption(option)}
            >
              {option}
            </div>
          ))
        }
      </div>
    </div>
  )
}