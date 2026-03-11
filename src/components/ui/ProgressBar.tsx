import { cn } from "../../libs/cn";

interface Props {
  max: number;
  value: number;
  bgClassName?: string;
  fgClassName?: string;
}

export function ProgressBar({ max, value, bgClassName, fgClassName }: Props) {
  const progress = Math.min(1, value / max) * 100;
  return (
    <div className={cn("w-full h-3 bg-fg rounded-full", bgClassName)}>
      <div
        className={cn(
          "h-full bg-blue-500 rounded-full transition-all",
          fgClassName,
        )}
        style={{ width: `${progress}%` }}
      />
    </div>
  );
}
