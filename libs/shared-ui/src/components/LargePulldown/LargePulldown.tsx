import {ReactNode, useEffect, useRef, useState} from "react";
import styles from "./LargePulldown.module.css";

export type LargePulldownItem = {
  label: ReactNode;
  onClick?: () => void;
  href?: string;
  disabled?: boolean;
  danger?: boolean;
};

type LargePulldownProps = {
  children: ReactNode;
  type: "primary" | "secondary";
  items: LargePulldownItem[];
  align?: "left" | "right";
};

export const LargePulldown = ({
  children,
  type,
  items,
  align = "left",
}: LargePulldownProps) => {
  const [isOpen, setIsOpen] = useState(false);
  const rootRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isOpen) return;

    const handlePointerDown = (event: PointerEvent) => {
      if (!rootRef.current?.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setIsOpen(false);
      }
    };

    document.addEventListener("pointerdown", handlePointerDown);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("pointerdown", handlePointerDown);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [isOpen]);

  return (
    <div className={styles.root} ref={rootRef}>
      <button
        type="button"
        className={`${styles.button} ${type === "primary" ? styles.primary : styles.secondary}`}
        aria-haspopup="menu"
        aria-expanded={isOpen}
        onClick={() => setIsOpen((current) => !current)}
      >
        <span className={styles.buttonContent}>{children}</span>
        <span className={styles.caret} aria-hidden="true" />
      </button>

      {isOpen && (
        <div
          className={`${styles.menu} ${align === "right" ? styles.menuRight : styles.menuLeft}`}
          role="menu"
        >
          {items.map((item, index) => {
            const className = `${styles.menuItem} ${item.danger ? styles.danger : ""}`;

            if (item.href && !item.disabled) {
              return (
                <a
                  key={index}
                  className={className}
                  href={item.href}
                  role="menuitem"
                  onClick={() => {
                    item.onClick?.();
                    setIsOpen(false);
                  }}
                >
                  {item.label}
                </a>
              );
            }

            return (
              <button
                key={index}
                type="button"
                className={className}
                disabled={item.disabled}
                role="menuitem"
                onClick={() => {
                  item.onClick?.();
                  setIsOpen(false);
                }}
              >
                {item.label}
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
};
