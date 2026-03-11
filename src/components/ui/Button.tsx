import { ReactNode } from "react";
import { cn } from "../../libs/cn";

interface Props {
  variant: "primary" | "secondary";
  disabled?: boolean;
  className?: string;
  onClick?: () => void;
  children: ReactNode;
}

export function Button({
  variant,
  disabled,
  className,
  onClick,
  children,
}: Props) {
  return (
    <button
      className={cn(
        "flex gap-4 items-center justify-center w-full h-16 rounded-xl text-4xl text-text-primary enabled:cursor-pointer transition shadow-md shadow-black/20",
        {
          "bg-primary hover:bg-primary-dark disabled:bg-primary-dark":
            variant === "primary",
          "bg-fg hover:bg-fg-dark disabled:bg-fg-dark": variant === "secondary",
        },
        className,
      )}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
}
