import { cn } from "../../libs/cn";

import CheckIcon from "@heroicons/react/24/solid/CheckIcon";

interface Props {
  value?: boolean;
  onChange?: (value: boolean) => void;
  id?: string;
  disabled?: boolean;
  className?: string;
}

export function Checkbox({ value, onChange, id, disabled, className }: Props) {
  return (
    <button
      onClick={() => onChange?.(!value)}
      className={cn(
        "size-10 flex items-center justify-center rounded-lg shadow-md shadow-black/20 enabled:cursor-pointer transition-all",
        {
          "bg-bg border-4 border-fg hover:bg-bg-dark hover:border-fg-dark disabled:border-fg-dark":
            !value,
          "bg-primary hover:bg-primary-dark disabled:bg-primary-dark": value,
        },
      )}
    >
      {value && <CheckIcon className="size-full p-1 fill-text-primary" />}
    </button>
  );
}
