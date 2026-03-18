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
import ChevronUpIcon from "@heroicons/react/24/solid/ChevronUpIcon";
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

export function QRMakerView() {
  const [qrcodeOption, setQRCodeOption] = useImmer<QRCodeMakerOptions>({
    type: "canvas",
    width: 400,
    height: 400,
    margin: 40,
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
  const [margin, setMargin] = useState(10);
  const [isAdvancedOptionsOpen, setIsAdvancedOptionsOpen] = useState(false);
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
    <div className="flex size-full pt-4 bg-bg">
      <div className="w-full flex flex-col gap-4 pl-4">
        <header className="flex gap-4">
          <Icon className="w-12 aspect-square" shapeClassName="fill-white" />
          <h1 className="text-text-primary text-5xl font-bold">QRCode Maker</h1>
        </header>
        <div className="w-full h-fit pr-4">
        <TextArea
          placeholder="https://example.com"
          onChange={handleTextChange}
            className="mr-4"
        />
        </div>
        <div className="h-full pr-4 flex flex-col gap-4 overflow-y-scroll">
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
              value={margin + "%"}
              onInputFinish={(value) =>
                setQRCodeOption((options) => {
                  const merginPercent = Math.min(
                    50,
                    Math.max(0, parseInt(value)),
                  );
                  const size = (qrcodeOption.width * merginPercent) / 100;
                  console.log(size);

                  if (isNaN(size)) return;
                  setMargin(merginPercent);
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
                  const dotType = value as DotType;
                  options.dotsOptions.type = dotType;
                  if (dotType === "dots") {
                    if (options.cornersDotOptions.type == null)
                      options.cornersDotOptions.type = "rounded";
                    if (options.cornersSquareOptions.type == null)
                      options.cornersSquareOptions.type = "rounded";
                  } else if (qrcodeOption.dotsOptions.type === "dots") {
                    if (options.cornersDotOptions.type == "rounded")
                      options.cornersDotOptions.type = undefined;
                    if (options.cornersSquareOptions.type == "rounded")
                      options.cornersSquareOptions.type = undefined;
                  }
                })
              }
              className="w-64"
            />
          </div>
          <button
            onClick={() => setIsAdvancedOptionsOpen(!isAdvancedOptionsOpen)}
            className="w-fit flex items-center justify-start gap-4 py-4 text-2xl text-primary cursor-pointer"
          >
            Advanced Options
            {isAdvancedOptionsOpen ? (
              <ChevronUpIcon className="w-6 h-6 scale-125" />
            ) : (
              <ChevronDownIcon className="w-6 h-6 scale-125" />
            )}
          </button>
          {isAdvancedOptionsOpen && <div className="relative w-full"></div>}
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
        <Button variant="primary">
          <ArrowDownTrayIcon className="w-10" />
          Save
        </Button>
      </div>
    </div>
  );
}
