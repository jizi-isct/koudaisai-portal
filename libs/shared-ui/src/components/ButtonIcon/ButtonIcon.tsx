import styles from './ButtonIcon.module.css';

type IconType = 'edit' | 'delete' | 'download';

type ButtonProps = {
  // ①
  iconType: IconType;
  onClick: () => void;
  isClicked?: boolean;
};

export const ButtonIcon = ({ iconType, onClick }: ButtonProps) => {
  const iconSrc = `/generic/${iconType}.svg`;

  return (
    <button className={styles.root} onClick={onClick}>
      <img src={iconSrc} alt={iconType} width={24} height={24} />
    </button>
  );
};
