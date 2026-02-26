import { Suspense, useEffect, useState, useTransition } from "react";
import { useImmer } from "use-immer";
import { genQRCode, GenQRCodeOptions } from "./libs/qrcode";
import { useDebouncedCallback } from "use-debounce";
import { Loading } from "./components/Loading";
import { QRCode } from "./components/QRCode";

const BOM = "\uFEFF";

export function App() {
  const [qrcodeOption, setQRCodeOption] = useImmer<GenQRCodeOptions>({});
  const [text, setText] = useState("");
  const [isQRCodeGenerating, startGenerateQRCode] = useTransition();
  const handleTextChange = useDebouncedCallback((text: string) => {
    setText(BOM + text);
  }, 500);
  return (
    <div className="size-full py-4 grid grid-cols-2 grid-rows-1 bg-bg">
      <div className="px-4">
        <h1 className="text-4xl font-bold text-text">QRCode Maker</h1>
        <div>
          <label className="text-text">Text:</label>
          <textarea
            className="cursor-auto w-full h-24 border border-text text-text text-xl"
            onChange={(e) => handleTextChange(e.target.value)}
          />
        </div>
      </div>
      <div className="flex flex-col">
        <div className="aspect-square">
          <Suspense fallback={<Loading />}>
            <QRCode text={text} options={qrcodeOption} />
          </Suspense>
        </div>
        <div>
          <button>Save</button>
          <button>Copy</button>
          <select>
            <option value="png">PNG</option>
            <option value="jpeg">JPEG</option>
            <option value="webp">WEBP</option>
            <option value="svg">SVG</option>
          </select>
        </div>
      </div>
    </div>
  );
}
