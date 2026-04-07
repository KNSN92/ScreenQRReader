import { useI18n } from "../hooks/i18n";
import { cn } from "../libs/cn";

interface Props {
  status: "empty" | "generating" | "valid" | "invalid" | "too-long" | "error";
}

export function QRCodeStatus({ status }: Props) {
  const t = useI18n();
  let message;
  switch (status) {
    case "empty":
      message = t("qr_maker.status.empty");
      break;
    case "generating":
      message = t("qr_maker.status.generating");
      break;
    case "valid":
      message = t("qr_maker.status.valid");
      break;
    case "invalid":
      message = t("qr_maker.status.invalid");
      break;
    case "too-long":
      message = t("qr_maker.status.too_long");
      break;
    case "error":
      message = t("qr_maker.status.error");
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
