import { useState } from "react";
import { useImmer } from "use-immer";
import {
  GenQRCodeOptions,
  QRError,
  ValidateQRCodeResponse,
} from "../libs/qrcode";
import { cssColorToHex } from "../libs/color";
import { useDebouncedCallback } from "use-debounce";
import { QRCode } from "../components/QRCode";
import { TextField } from "../components/ui/TextField";
import { Selector } from "../components/ui/Selector";
import { TextArea } from "../components/ui/TextArea";
import { QRCodeStatus } from "../components/QRCodeStatus";

import ChevronDownIcon from "@heroicons/react/24/solid/ChevronDownIcon";
import ArrowDownTrayIcon from "@heroicons/react/24/solid/ArrowDownTrayIcon";

import {
  CornerDotType,
  CornerSquareType,
  DotType,
  DrawType,
  Gradient,
} from "qr-code-styling";
import { Icon } from "../components/Icon";
import { Button } from "../components/ui/Button";

type QRCodeMakerOptions = GenQRCodeOptions & {
  type: DrawType;
  width: number;
  height: number;
  margin: number;
  imageOptions: {
    saveAsBlob?: boolean;
    hideBackgroundDots?: boolean;
    imageSize?: number;
    crossOrigin?: string;
    margin?: number;
  };
  dotsOptions: {
    type: DotType;
    color?: string;
    gradient?: Gradient;
    roundSize?: boolean;
  };
  cornersSquareOptions: {
    type?: CornerSquareType;
    color?: string;
    gradient?: Gradient;
  };
  cornersDotOptions: {
    type?: CornerDotType;
    color?: string;
    gradient?: Gradient;
  };
  backgroundOptions: {
    round?: number;
    color?: string;
    gradient?: Gradient;
  };
};

export function App() {
  const [qrcodeOption, setQRCodeOption] = useImmer<QRCodeMakerOptions>({
    type: "canvas",
    width: 400,
    height: 400,
    margin: 20,
    backgroundOptions: { color: "#FFFFFF" },
    dotsOptions: {
      type: "square",
      color: "#000000",
    },
    cornersDotOptions: {},
    cornersSquareOptions: {},
    imageOptions: {},
  });
  const [eccLevel, setEccLevel] = useState<"L" | "M" | "Q" | "H">("M");
  const [text, setText] = useState("");
  const handleTextChange = useDebouncedCallback((text: string) => {
    setText(text);
  }, 500);
  const [qrcodeValidateResult, setQRCodeValidateResult] = useState<
    ValidateQRCodeResponse | QRError | "InvalidData" | "Generating" | "Empty"
  >("Empty");
  let qrcodeStatus:
    | "empty"
    | "generating"
    | "valid"
    | "invalid"
    | "too-long"
    | "error";
  switch (qrcodeValidateResult) {
    case "Empty":
      qrcodeStatus = "empty";
      break;
    case "Generating":
      qrcodeStatus = "generating";
      break;
    case "Valid":
      qrcodeStatus = "valid";
      break;
    case "Invalid":
      qrcodeStatus = "invalid";
      break;
    case "DataTooLong":
      qrcodeStatus = "too-long";
      break;
    default:
      qrcodeStatus = "error";
      break;
  }
  return (
    <div className="flex size-full py-4 bg-bg">
      <div className="w-full flex flex-col gap-4 px-4">
        <header className="flex gap-4">
          <Icon className="w-12 aspect-square" shapeClassName="fill-white" />
          <h1 className="text-text-primary text-5xl font-bold">QRCode Maker</h1>
        </header>
        <TextArea
          placeholder="https://example.com"
          onChange={handleTextChange}
        />
        <div className="flex flex-col gap-4">
          <div className="flex justify-between">
            <label className="text-3xl text-text-primary" htmlFor="ecc-level">
              Ecc Level
            </label>
            <Selector
              id="ecc-level"
              items={[
                { label: "Low", value: "L" },
                { label: "Medium", value: "M" },
                { label: "Quartile", value: "Q" },
                { label: "High", value: "H" },
              ]}
              value={eccLevel}
              onChange={(value) => {
                const isValid = value?.length === 1 && "LMQH".includes(value);
                if (!isValid) return;
                setEccLevel(value as "L" | "M" | "Q" | "H");
              }}
            />
          </div>
          <div className="flex justify-between">
            <label className="text-3xl text-text-primary" htmlFor="margin">
              Margin
            </label>
            <TextField
              id="margin"
              value={qrcodeOption.margin.toString() + "px"}
              onInputFinish={(value) =>
                setQRCodeOption((options) => {
                  const size = parseInt(value);
                  if (isNaN(size)) return;
                  options.margin = size;
                })
              }
            />
          </div>
          <div className="flex justify-between">
            <label className="text-3xl text-text-primary" htmlFor="back-color">
              Back Color
            </label>
            <div className="flex gap-4">
              <div
                className="w-20 h-10 rounded-lg shadow-md shadow-black/20"
                style={{
                  backgroundColor: qrcodeOption.backgroundOptions?.color,
                }}
              />
              <TextField
                id="back-color"
                value={qrcodeOption.backgroundOptions?.color}
                onInputFinish={(value) =>
                  setQRCodeOption((options) => {
                    const hexColor = cssColorToHex(value);
                    if (!hexColor) return;
                    options.backgroundOptions = options.backgroundOptions || {};
                    options.backgroundOptions.color = hexColor;
                  })
                }
              />
            </div>
          </div>
          <div className="flex justify-between">
            <label className="text-3xl text-text-primary" htmlFor="dots-color">
              Dots Color
            </label>
            <div className="flex gap-4">
              <div
                className="w-20 h-10 rounded-lg shadow-md shadow-black/20"
                style={{
                  backgroundColor: qrcodeOption.dotsOptions?.color,
                }}
              />
              <TextField
                id="dots-color"
                value={qrcodeOption.dotsOptions?.color}
                onInputFinish={(value) =>
                  setQRCodeOption((options) => {
                    const hexColor = cssColorToHex(value);
                    if (!hexColor) return;
                    options.dotsOptions = options.dotsOptions || {};
                    options.dotsOptions.color = hexColor;
                  })
                }
              />
            </div>
          </div>
        </div>
        <div className="flex justify-between">
          <label className="text-3xl text-text-primary" htmlFor="dots-style">
            Dots Style
          </label>
          <Selector
            id="dots-style"
            items={[
              { label: "Square", value: "square" },
              { label: "Dots", value: "dots" },
              { label: "Rounded", value: "rounded" },
              { label: "Extra Rounded", value: "extra-rounded" },
              { label: "Classy", value: "classy" },
              { label: "Classy Rounded", value: "classy-rounded" },
            ]}
            value={qrcodeOption.dotsOptions.type}
            onChange={(value) =>
              setQRCodeOption((options) => {
                options.dotsOptions = options.dotsOptions || {};
                options.dotsOptions.type = value as DotType;
              })
            }
            className="w-64"
          />
        </div>
        <div className="flex items-center justify-start gap-4 py-4 text-2xl text-primary cursor-pointer">
          Advanced Options
          <ChevronDownIcon className="w-6 h-6 scale-125" />
        </div>
      </div>
      <div className="w-1 h-full rounded-full bg-text-secondary" />
      <div className="flex flex-col justify-between w-full px-4">
        <div className="w-full aspect-square">
          <QRCode
            text={text}
            ecclevel={eccLevel}
            options={qrcodeOption}
            setQRCodeStatus={setQRCodeValidateResult}
          />
        </div>
        <QRCodeStatus status={qrcodeStatus} />
        <div className="flex gap-4 px-2">
          <div className="flex justify-between gap-2">
            <label className="text-3xl text-text-primary" htmlFor="size">
              size
            </label>
            <TextField
              id="size"
              value={qrcodeOption.width.toString() + "px"}
              onInputFinish={(value) =>
                setQRCodeOption((options) => {
                  const size = parseInt(value);
                  if (isNaN(size)) return;
                  options.width = size;
                  options.height = size;
                })
              }
            />
          </div>
          <div className="flex justify-between gap-2">
            <label className="text-3xl text-text-primary" htmlFor="format">
              Format
            </label>
            <Selector
              id="format"
              items={[
                { label: "png", value: "png" },
                { label: "jpeg", value: "jpeg" },
                { label: "svg", value: "svg" },
              ]}
            />
          </div>
        </div>
        {/* TODO: ダウンロード出来るようにする */}
        <Button variant="secondary">
          <ArrowDownTrayIcon className="w-10" />
          Save
        </Button>
      </div>
    </div>
  );
}
