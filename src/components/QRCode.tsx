import { useEffect, useRef, useState, useTransition } from "react";
import {
  EcLevel,
  genQRCode,
  GenQRCodeOptions,
  QRError,
  validateQRCode,
  ValidateQRCodeResponse,
} from "../libs/qrcode";
import { Loading } from "./Loading";
import { Icon } from "./Icon";

interface Props {
  text: string;
  eclevel: EcLevel;
  options: GenQRCodeOptions;
  setQRCodeStatus?: (
    status:
      | ValidateQRCodeResponse
      | QRError
      | "InvalidData"
      | "Generating"
      | "Empty",
  ) => void;
}

export function QRCode({ text, eclevel, options, setQRCodeStatus }: Props) {
  const previewRef = useRef<HTMLDivElement>(null);
  const [isGenerating, startGenerate] = useTransition();
  const [isEmpty, setIsEmpty] = useState(true);

  async function generate() {
    // せっかく作ったローディング表示が一瞬で消えるのは寂しいので、偽のロードを表示。許してちょ ;-)
    if (!previewRef.current) return;
    const preview = previewRef.current;
    if (text === "") {
      previewRef.current.innerHTML = "";
      setIsEmpty(true);
      setQRCodeStatus?.("Empty");
      return;
    }
    setQRCodeStatus?.("Generating");

    await wait(Math.random() * 500 + 500);

    const qrcode = await genQRCode(text, eclevel, options);
    if (typeof qrcode === "string") {
      console.error(qrcode);
      setQRCodeStatus?.(qrcode);
    } else {
      preview.innerHTML = "";
      qrcode.append(preview);
      setIsEmpty(false);

      const data = await qrcode.getRawData("png");
      if (data == null) {
        setQRCodeStatus?.("InvalidData");
        return;
      }
      const validate = await validateQRCode(
        new TextEncoder().encode(text),
        new Uint8Array(await data.arrayBuffer()),
      );
      setQRCodeStatus?.(validate);
    }
  }

  useEffect(() => {
    startGenerate(generate);
  }, [text, eclevel, options]);
  return (
    <>
      {(isGenerating || isEmpty) && (
        <div className="size-full flex justify-center items-center">
          {isGenerating && <Loading />}
          {isEmpty && !isGenerating && (
            <Icon
              className="size-1/2 aspect-square"
              shapeClassName="fill-stone-400"
            />
          )}
        </div>
      )}
      <div
        ref={previewRef}
        id="qrcode-preview"
        className="*:w-full *:h-full"
        hidden={isGenerating || isEmpty}
      />
    </>
  );
}

async function wait(milliseconds: number): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, milliseconds));
}
