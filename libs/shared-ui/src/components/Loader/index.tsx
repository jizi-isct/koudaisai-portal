import type React from "react";

const dotBase = "absolute top-0 left-1/2 w-[5px] h-[5px] rounded-full";
const dotStyle = (n: number): React.CSSProperties => ({
  transform: "translate(-15px, 15px)",
  transformOrigin: "15px center",
  animation: `loader-rotate${n} 1.2s cubic-bezier(0.4, 0, 0.2, 1) infinite`,
});
const trailStyle = (n: number): React.CSSProperties => ({
  ...dotStyle(n),
  animationDelay: "0.03s",
});

export function Loader() {
  return (
    <div className="relative w-[30px] h-[30px]">
      <div>
        {([1, 2, 3] as const).map(n => (
          <div key={n} className={`${dotBase} bg-lightblue`} style={trailStyle(n)} />
        ))}
      </div>
      <div>
        {([1, 2, 3] as const).map(n => (
          <div key={n} className={`${dotBase} bg-darkblue`} style={dotStyle(n)} />
        ))}
      </div>
    </div>
  )
}
