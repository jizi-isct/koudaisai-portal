import {ReactNode} from "react";

type ButtonProps = {
  children: ReactNode
  type: "primary" | "secondary";
  onClick?: () => void;
  href?: string;
}

const baseClass = "inline-flex items-center justify-center rounded-[30px] cursor-pointer [transition:box-shadow_300ms_ease-out,background-color_300ms_ease-out,color_300ms_ease-out,scale_300ms_ease-out,transform_300ms_ease-out] px-[30px] border border-darkblue text-sm font-medium mx-[10px] h-[55px] shadow-[0_0_12px_0_rgba(37,54,97,0.25)] hover:shadow-[0_0_12px_0_var(--darkblue)] hover:scale-105";

export const LargeButton = ({children, type, onClick, href}: ButtonProps) => {
  const className = `${baseClass} ${type === "primary" ? "bg-darkblue text-white" : "bg-white text-black"}`;

  if (href) {
    return (
      <a href={href} className={className} onClick={onClick}>
        {children}
      </a>
    );
  }

  return (
    <button className={className} onClick={onClick}>
      {children}
    </button>
  );
};
