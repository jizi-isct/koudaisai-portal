import { ReactNode, useCallback, useEffect, useRef, useState } from "react";

export type PulldownItem = {
  label: ReactNode;
  onClick: () => void;
};

type LargePulldownProps = {
  children: ReactNode;
  type: "primary" | "secondary";
  items: PulldownItem[];
};

export const LargePulldown = ({ children, type, items }: LargePulldownProps) => {
  const [open, setOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const handleItemClick = useCallback((item: PulldownItem) => {
    item.onClick();
    setOpen(false);
  }, []);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const isPrimary = type === "primary";
  const baseBtn = `inline-flex items-center justify-center gap-[10px] px-[30px] h-[55px] rounded-[30px] border border-darkblue text-sm font-medium cursor-pointer [transition:box-shadow_300ms_ease-out,scale_300ms_ease-out,transform_300ms_ease-out] shadow-[0_0_12px_0_rgba(37,54,97,0.25)] hover:shadow-[0_0_12px_0_var(--darkblue)] hover:scale-105`;

  return (
    <div className="relative w-fit" ref={containerRef}>
      <button
        className={`${baseBtn} ${isPrimary ? "bg-darkblue text-white" : "bg-white text-black"}`}
        onClick={() => setOpen(prev => !prev)}
        aria-expanded={open}
      >
        <span>{children}</span>
        <span className={`inline-block w-2 h-2 border-r-2 border-b-2 border-current transition-transform duration-300 ease-out shrink-0 ${open ? "-rotate-[135deg] -mb-1" : "rotate-45 mb-0.5"}`} />
      </button>

      <div className={`absolute top-full left-0 mt-2 w-full rounded-[16px] border border-darkblue overflow-hidden shadow-[0_0_16px_0_rgba(37,54,97,0.3)] transition-opacity duration-200 ${open ? "opacity-100 pointer-events-auto" : "opacity-0 pointer-events-none"}`}>
        {items.map((item, i) => (
          <button
            key={i}
            className={`block w-full px-4 py-3 text-sm font-medium text-left cursor-pointer whitespace-nowrap border-0 ${i > 0 ? "border-t border-[rgba(37,54,97,0.2)]" : ""} ${isPrimary ? "bg-darkblue text-white hover:brightness-[1.2]" : "bg-white text-black hover:bg-[rgba(37,54,97,0.06)]"} transition-[filter,background-color] duration-150`}
            onClick={() => handleItemClick(item)}
          >
            {item.label}
          </button>
        ))}
      </div>
    </div>
  );
};
