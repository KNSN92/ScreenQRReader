import { cn } from "../libs/cn";

interface Props {
  status: "empty" | "generating" | "valid" | "invalid" | "too-long" | "error";
}

export function QRCodeStatus({ status }: Props) {
  let message;
  switch (status) {
    case "empty":
      message = "QRCode is empty";
      break;
    case "generating":
      message = "Generating...";
      break;
    case "valid":
      message = "Scannable";
      break;
    case "invalid":
      message = "UnScannable";
      break;
    case "too-long":
      message = "TextTooLong";
      break;
    case "error":
      message = "Error";
      break;
  }
  const className = cn(
    "w-full h-20 px-8 py-4 rounded-xl border-2 text-text-primary text-4xl shadow-md shadow-black/20",
    {
      "bg-[#8f8]/50 border-[#8f8]": status === "valid",
      "bg-[#f88]/50 border-[#f88]":
        status === "invalid" || status === "too-long",
      "bg-[#888]/50 border-[#888]":
        status === "empty" || status === "generating" || status === "error",
    },
  );
  return <div className={className}>{message}</div>;
}
