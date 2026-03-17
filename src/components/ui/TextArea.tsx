import { cn } from "../../libs/cn";

interface Props {
  placeholder?: string;
  value?: string;
  onChange?: (value: string) => void;
  className?: string;
}

export function TextArea({ placeholder, value, onChange, className }: Props) {
  return (
    <textarea
      placeholder={placeholder}
      value={value}
      onChange={(e) => onChange?.(e.target.value)}
      className={cn(
        "w-full min-h-34 px-4 py-2.5 overflow-y-scroll rounded-lg bg-fg text-2xl text-text-primary placeholder:text-text-secondary resize-none shadow-md shadow-black/20",
        className,
      )}
    />
  );
}
