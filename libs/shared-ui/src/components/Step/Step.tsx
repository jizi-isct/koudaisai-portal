import {ReactNode} from "react";
import "./Step.css";

type Props = {
  title: string,
  date?: string,
  children: ReactNode,
  isFirst: boolean
}

export function Step({title, date, children, isFirst}: Props) {
  const cardClass = `step-card ${isFirst ? "step-card--first" : "step-card--after"} relative bg-no-repeat drop-shadow-[4px_4px_0_var(--darkblue)]`;

  return (
    <div className={cardClass}>
      {date && (
        <div className="step-date absolute bg-darkblue text-white text-[10px] font-bold py-[0.1em] px-[0.5em] rounded-[0.5em]">
          🕓 {date}
        </div>
      )}
      <h2 className="step-heading font-bold text-[22px] text-[#253661]">
        {title}
      </h2>
      <p className="text-[13px]">
        {children}
      </p>
    </div>
  )
}