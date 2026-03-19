import Select from "react-select";
import { cn } from "../../libs/cn";

interface Props {
  items: { label: string; value: string }[];
  value?: string;
  onChange?: (value: string | null) => void;
  placeholder?: string;
  id?: string;
  className?: string;
}

// function o2f<K extends keyof any, V>(obj: Record<K, V>): Record<K, () => V> {
//   const result = {} as Record<K, () => V>;
//   for (const key in obj) {
//     result[key] = () => obj[key];
//   }
//   return result;
// }

export function Selector({
  items,
  value,
  onChange,
  placeholder,
  id,
  className,
}: Props) {
  const label = items.find((item) => item.value === value)?.label;
  return (
    <Select
      id={id}
      options={items}
      value={
        label == null
          ? undefined
          : {
              label,
              value,
            }
      }
      onChange={(v) => onChange?.(v?.value || null)}
      className={cn("w-40", className)}
      classNames={{
        container: () => "rounded-lg text-2xl shadow-md shadow-black/20",
        control: () =>
          "h-10 px-3 bg-fg hover:bg-fg-dark rounded-lg text-white cursor-pointer!",
        menu: () => "rounded-lg py-2 bg-fg text-white",
        option: (props) =>
          cn(
            "flex justify-center items-center h-10 px-3 transition-all",
            props.isSelected
              ? "bg-fg-dark"
              : "hover:bg-primary cursor-pointer!",
          ),
      }}
      unstyled
      placeholder={placeholder}
    />
  );
}
