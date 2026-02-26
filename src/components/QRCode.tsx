import { use, useEffect, useRef } from "react";
import { genQRCode, GenQRCodeOptions } from "../libs/qrcode";

interface Props {
  text: string;
  options?: GenQRCodeOptions;
}

export async function QRCode({ text, options }: Props) {
  const previewRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!previewRef.current) return;
    if (text === "") {
      previewRef.current.innerHTML = "";
      return;
    }
    generateQRCode(previewRef.current, text, options);
  }, [text, options]);
  return (
    <div
      ref={previewRef}
      id="qrcode-preview"
      className="*:max-w-full *:max-h-full"
    />
  );
}

async function generateQRCode(
  preview: HTMLElement,
  text: string,
  options?: GenQRCodeOptions,
) {
  const qrcode = await genQRCode(text, "M", {
    type: "canvas",
    width: 400,
    height: 400,
    margin: 20,
    dotsOptions: {
      color: "blue",
      type: "rounded",
    },
    ...options,
  });
  preview.innerHTML = "";
  qrcode.append(preview);
}
