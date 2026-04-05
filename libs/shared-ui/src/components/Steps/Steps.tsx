import {Step} from "../Step";
import styles from "./Steps.module.css";
import {chunk} from "@koudaisai/shared-utils";
import {JSX} from "react";

type Props = {
  steps: Array<{
    title: string,
    date?: string,
    children: JSX.Element
  }>
}

export function Steps({steps}: Props) {
  const stepChunks = chunk(steps, 3)
  return (
    <div className={styles.root}>
      {
        stepChunks.map((stepChunk, i) =>
          <div key={`steps-${i}`} className={styles.steps}>
            <svg className={styles.arrow} width="102" height="24" viewBox="0 0 102 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <line y1="12" x2="50" y2="12" stroke={i === 0 ? "transparent" : "#253661"} strokeWidth="3" strokeDasharray="6 6"/>
              <path d="M101.061 13.0607C101.646 12.4749 101.646 11.5251 101.061 10.9393L91.5147 1.3934C90.9289 0.807611 89.9792 0.807611 89.3934 1.3934C88.8076 1.97919 88.8076 2.92893 89.3934 3.51472L97.8787 12L89.3934 20.4853C88.8076 21.0711 88.8076 22.0208 89.3934 22.6066C89.9792 23.1924 90.9289 23.1924 91.5147 22.6066L101.061 13.0607ZM50 13.5H100V10.5H50V13.5Z" fill={i === 0 ? "transparent" : "#253661"}/>
            </svg>
            {
              stepChunk.map((step, j) =>
                <Step key={`step-${i}-${j}`} title={step.title} date={step.date} isFirst={i === 0 && j === 0}>
                  {step.children}
                </Step>
              )
            }
            <svg className={styles.arrow} width="100" height="3" viewBox="0 0 100 3" fill="none" xmlns="http://www.w3.org/2000/svg">
              <line y1="1.5" x2="50" y2="1.5" stroke={i === stepChunks.length - 1  ? "transparent" : "#253661"} strokeWidth="3"/>
              <line x1="50" y1="1.5" x2="100" y2="1.5" stroke={i === stepChunks.length - 1  ? "transparent" : "#253661"}  strokeWidth="3" strokeDasharray="6 6"/>
            </svg>
          </div>
        )
      }
    </div>
  )
}