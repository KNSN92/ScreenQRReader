import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

import TrashIcon from "@heroicons/react/24/solid/TrashIcon";
import DocumentMagnifyingGlassIcon from "@heroicons/react/24/solid/DocumentMagnifyingGlassIcon";
import { Button } from "./Button";
import { Icon } from "../Icon";
import { cn } from "../../libs/cn";

interface Props {
  imgUrl?: string;
  onChange?: (value: string | null) => void;
  disabled?: boolean;
  className?: string;
}

export function ImageSelector({
  imgUrl,
  onChange,
  disabled,
  className,
}: Props) {
  function handleSelectImage() {
    open({
      title: "Select an image",
      filters: [
        {
          name: "Image Files",
          extensions: ["jpg", "jpeg", "png", "svg"],
        },
      ],
      multiple: false,
      directory: false,
    }).then((path) => {
      const url = path && convertFileSrc(path);
      onChange?.(url);
    });
  }
  return (
    <div
      className={cn(
        "group relative w-64 h-44 rounded-lg bg-black shadow-md shadow-black/20 overflow-hidden",
        className,
      )}
      aria-disabled={disabled}
    >
      {imgUrl == null ? (
        <button
          className="flex flex-col items-center justify-center gap-2 size-full enabled:cursor-pointer"
          onClick={handleSelectImage}
          disabled={disabled}
        >
          {disabled ? (
            <Icon
              className="w-16 aspect-square"
              shapeClassName="fill-text-secondary"
            />
          ) : (
            <>
              <DocumentMagnifyingGlassIcon className="h-10.5 fill-text-primary! aria-disabled:hidden" />
              <span className="w-full px-2 text-center text-2xl text-text-primary aria-disabled:hidden whitespace-nowrap overflow-hidden text-ellipsis">
                Select an image
              </span>
            </>
          )}
        </button>
      ) : (
        <>
          <img
            src={imgUrl}
            alt="selected image"
            className="size-full object-contain"
          />
          <div className="absolute inset-0 opacity-0 group-hover:opacity-100 flex flex-col items-center justify-center gap-3 size-full bg-black/50 transition-all">
            <span className="w-full px-2 text-center text-xl text-text-primary whitespace-nowrap overflow-hidden text-ellipsis">
              {decodeURIComponent(imgUrl).split("/").pop()}
            </span>
            <div className="flex gap-3">
              <Button
                variant="danger"
                onClick={() => onChange?.(null)}
                disabled={disabled}
                className="size-10 rounded-lg"
              >
                <TrashIcon className="w-4.5" />
              </Button>
              <Button
                variant="secondary"
                onClick={handleSelectImage}
                disabled={disabled}
                className="size-10 rounded-lg"
              >
                <DocumentMagnifyingGlassIcon className="h-6" />
              </Button>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
