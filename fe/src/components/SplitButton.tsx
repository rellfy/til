import {useEffect, useRef, useState, type ReactNode} from "react";
import "./SplitButton.css";

export type SplitButtonItem = {
  label: string;
  onClick: () => void;
};

type Props = {
  label: ReactNode;
  onClick: () => void;
  items: SplitButtonItem[];
};

const SplitButton = ({label, onClick, items}: Props) => {
  const [open, setOpen] = useState(false);
  const rootRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!open) return;
    const onDoc = (e: MouseEvent) => {
      if (rootRef.current && !rootRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", onDoc);
    return () => document.removeEventListener("mousedown", onDoc);
  }, [open]);
  return (
    <div className="split-button" ref={rootRef}>
      <button type="button" className="split-button-main" onClick={onClick}>
        {label}
      </button>
      <button
        type="button"
        className="split-button-chevron"
        onClick={() => setOpen((o) => !o)}
        aria-label="more options"
        aria-expanded={open}
      >
        ▾
      </button>
      {open && (
        <div className="split-button-menu" role="menu">
          {items.map((item) => (
            <button
              key={item.label}
              type="button"
              className="split-button-menu-item"
              onClick={() => {
                setOpen(false);
                item.onClick();
              }}
            >
              {item.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
};

export default SplitButton;
