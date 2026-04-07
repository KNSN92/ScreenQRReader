import { useEffect, useState } from "react";
import { cn } from "../../libs/cn";

interface Props {
  id?: string;
  className?: string;
  value?: string;
  onChange?: (value: string) => void;
  onInputFinish?: (value: string) => void;
}

export function TextField({
  id,
  className,
  value,
  onChange,
  onInputFinish,
}: Props) {
  const [inputValue, setInputValue] = useState<string | null>("");
  useEffect(() => {
    setInputValue(value ?? "");
  }, [value]);
  return (
    <input
      id={id}
      type="text"
      value={inputValue == null ? value : inputValue}
      onChange={(e) => {
        const value = e.target.value;
        setInputValue(value);
        onChange?.(value);
      }}
      onBlur={(e) => {
        const value = e.target.value;
        onInputFinish?.(value);
        setInputValue(null);
      }}
      onKeyDown={(e) => {
        if (e.key === "Enter") {
          const value = e.currentTarget.value;
          onInputFinish?.(value);
          setInputValue(null);
        }
      }}
      className={cn(
        "block w-40 h-10 px-3 text-2xl rounded-lg bg-fg text-text-primary shadow-md shadow-black/20",
        className,
      )}
    />
  );
}
