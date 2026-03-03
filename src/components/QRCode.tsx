import { useEffect, useRef, useTransition } from "react";
import {
  genQRCode,
  GenQRCodeOptions,
  validateQRCode,
  ValidateQRCodeResponse,
} from "../libs/qrcode";
import { Loading } from "./Loading";

interface Props {
  text: string;
  options?: GenQRCodeOptions;
  setValidateResult?: (
    result: ValidateQRCodeResponse | { error: "InvalidData" },
  ) => void;
}

export function QRCode({ text, options, setValidateResult }: Props) {
  const previewRef = useRef<HTMLDivElement>(null);
  const [isGenerating, startGenerate] = useTransition();

  async function generate() {
    // せっかく作ったローディング表示が一瞬で消えるのは寂しいので、偽のロードを表示。許してちょ ;-)
    await wait(Math.random() * 500 + 500);
    if (!previewRef.current) return;
    const preview = previewRef.current;
    if (text === "") {
      previewRef.current.innerHTML = "";
      return;
    }

    const qrcode = await genQRCode(text, "M", {
      type: "canvas",
      width: 400,
      height: 400,
      margin: 20,
      dotsOptions: {
        color: "#000",
        type: "extra-rounded",
      },
      cornersDotOptions: {
        color: "#f00",
        type: "extra-rounded",
      },
      cornersSquareOptions: {
        color: "#00f",
      },
      ...options,
    });
    if (typeof qrcode === "string") {
      console.error(qrcode);
    } else {
      preview.innerHTML = "";
      qrcode.append(preview);

      const data = await qrcode.getRawData("png");
      if (data == null) {
        setValidateResult?.({ error: "InvalidData" });
        return;
      }
      const validate = await validateQRCode(
        new TextEncoder().encode(text),
        new Uint8Array(await data.arrayBuffer()),
      );
      if (setValidateResult) {
        setValidateResult(validate);
      }
    }
  }

  useEffect(() => {
    startGenerate(generate);
  }, [text, options]);
  return (
    <>
      {isGenerating && (
        <div className="size-full flex justify-center items-center">
          <Loading />
        </div>
      )}
      <div
        ref={previewRef}
        id="qrcode-preview"
        className="*:w-full *:h-full"
        hidden={isGenerating}
      />
    </>
  );
}

async function wait(milliseconds: number): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, milliseconds));
}
