import Image from "next/image";

type IconType = "edit" | "delete" | "download";

type ButtonProps = {
  iconType: IconType;
  onClick: () => void;
  isClicked?: boolean;
};

export const ButtonIcon = ({iconType, onClick}: ButtonProps) => {
  const iconSrc = `/generic/${iconType}.svg`

  return (
    <button className="border-none bg-transparent h-6 w-6 cursor-pointer hover:brightness-[3]" onClick={onClick}>
      <Image src={iconSrc} alt={iconType} width={24} height={24}/>
    </button>
  );
};