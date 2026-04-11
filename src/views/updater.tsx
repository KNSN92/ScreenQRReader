import { ReactNode, useEffect, useState } from "react";
import { Button } from "../components/ui/Button";

import { check, Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

import ArrowDownOnSquare from "@heroicons/react/24/solid/ArrowDownOnSquareIcon";
import ArrowPathIcon from "@heroicons/react/24/solid/ArrowPathIcon";
import { ProgressBar } from "../components/ui/ProgressBar";
import { cn } from "../libs/cn";
import { Loading } from "../components/Loading";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Updater, useImmer } from "use-immer";
import { useI18n } from "../hooks/i18n";

type UpdaterStatus = "available" | "installing" | "installed" | "failed";

interface UpdatingInfo {
  total: number;
  downloaded: number;
}

const DEBUG = false;

export function UpdaterView() {
  const [update, setUpdate] = useState<Update | null>(null);

  useEffect(() => {
    check().then(setUpdate);
  }, []);

  return (
    <div className="flex size-full p-2 bg-bg">
      <div className="w-24 h-full flex justify-between items-center">
        <img src="/icon.png" className="w-full aspect-square" />
      </div>
      {!(update != null || DEBUG) ? <Fetching /> : <Content update={update} />}
    </div>
  );
}

function Fetching() {
  return (
    <div className="grow p-2">
      <div className="flex justify-center items-center w-auto h-full mx-auto aspect-square">
        <Loading />
      </div>
    </div>
  );
}

function Content({ update }: { update: Update | null }) {
  const [updateStatus, setUpdateStatus] = useState<UpdaterStatus>("available");
  const [updatingInfo, setUpdatingInfo] = useImmer<UpdatingInfo | null>(null);
  const t = useI18n();

  let title: string;
  let message: string | null = null;
  switch (updateStatus) {
    case "available":
      title = t("updater.available.title");
      message = t(
        "updater.available.message",
        update
          ? t("updater.available.message.version", update.version)
          : t("updater.available.message.no_version"),
      );
      break;
    case "installing":
      title = t("updater.installing.title");
      message = null;
      break;
    case "installed":
      title = t("updater.installed.title");
      message = t("updater.installed.message");
      break;
    case "failed":
      title = t("updater.failed.title");
      message = t("updater.failed.message");
  }
  return (
    <div className="grow flex flex-col p-2 gap-2">
      <div
        className={cn(
          "grow h-19.5 py-3 flex flex-col justify-start items-start",
          updateStatus === "installing" && "justify-between",
        )}
      >
        <h1 className="text-xl text-text-primary font-bold text-nowrap">
          {title}
        </h1>
        {message && (
          <span className="pl-1 text-xs text-text-primary">
            {message
              .split("\n")
              .reduce(
                (acc, value, i, src) => [
                  ...acc,
                  value,
                  ...(src.length - 1 !== i ? [<br />] : []),
                ],
                [] as ReactNode[],
              )}
          </span>
        )}
        {updateStatus === "installing" && updatingInfo && (
          <Progress max={updatingInfo.total} value={updatingInfo.downloaded} />
        )}
      </div>
      <div className="h-8 flex gap-2 justify-end items-center">
        <Choices
          update={update}
          updateStatus={updateStatus}
          setUpdateStatus={setUpdateStatus}
          setUpdatingInfo={setUpdatingInfo}
        />
      </div>
    </div>
  );
}

function Progress({ max, value }: { max: number; value: number }) {
  return (
    <div className="w-full flex gap-2 items-center h-fit">
      <span className="w-9 text-[16px] text-text-primary">
        {Math.round((value / max) * 1000) / 10}%
      </span>
      <ProgressBar max={max} value={value} bgClassName="" />
    </div>
  );
}

function Choices({
  update,
  updateStatus,
  setUpdateStatus,
  setUpdatingInfo,
}: {
  update: Update | null;
  updateStatus: UpdaterStatus;
  setUpdateStatus: (status: UpdaterStatus) => void;
  setUpdatingInfo: Updater<UpdatingInfo | null>;
}) {
  const t = useI18n();
  switch (updateStatus) {
    case "available":
      return (
        <>
          <Button
            variant="secondary"
            className="w-24 h-full text-[16px] gap-1 rounded-lg"
            onClick={() => {
              getCurrentWindow().close();
            }}
          >
            {t("updater.choice.skip")}
          </Button>
          <Button
            variant="primary"
            className="w-24 h-full text-[16px] gap-1 rounded-lg"
            onClick={() => {
              if (DEBUG) {
                __debug__simulateInstallUpdate(
                  setUpdateStatus,
                  setUpdatingInfo,
                );
              } else if (update) {
                installUpdate(update, setUpdateStatus, setUpdatingInfo);
              }
            }}
            disabled={!(update || DEBUG)}
          >
            <ArrowDownOnSquare className="relative -top-0.5 fill-white h-6" />
            {t("updater.choice.install")}
          </Button>
        </>
      );
    case "installing":
      return (
        <Button
          variant="secondary"
          className="w-24 h-full text-[16px] gap-1 rounded-lg"
          disabled={true}
        >
          {t("updater.choice.cancel")}
        </Button>
      );
    case "installed":
      return (
        <>
          <Button
            variant="secondary"
            className="w-24 h-full text-[16px] gap-1 rounded-lg"
            onClick={() => getCurrentWindow().close()}
          >
            {t("updater.choice.skip")}
          </Button>
          <Button
            variant="primary"
            className="w-24 h-full text-[16px] gap-1 rounded-lg"
            onClick={() => relaunch()}
          >
            <ArrowPathIcon className="relative -top-0.5 fill-white h-6" />
            {t("updater.choice.restart")}
          </Button>
        </>
      );
    case "failed":
      return (
        <Button
          variant="secondary"
          className="w-24 h-full text-[16px] gap-1 rounded-lg"
        >
          {t("updater.choice.close")}
        </Button>
      );
  }
}

function installUpdate(
  update: Update,
  setUpdateStatus: (status: UpdaterStatus) => void,
  setUpdatingInfo: Updater<UpdatingInfo | null>,
) {
  let downloaded = 0;
  update.downloadAndInstall((e) => {
    switch (e.event) {
      case "Started":
        setUpdateStatus("installing");
        setUpdatingInfo((draft) => {
          if (!draft) draft = { total: 0, downloaded: 0 };
          draft.total = e.data.contentLength ?? 0;
        });
        break;
      case "Progress":
        downloaded += e.data.chunkLength;
        setUpdatingInfo((draft) => {
          if (!draft) draft = { total: 0, downloaded: 0 };
          draft.downloaded = downloaded;
        });
        break;
      case "Finished":
        setUpdateStatus("installed");
        break;
    }
  });
}

// @ts-ignore - for debug
function __debug__simulateInstallUpdate(
  setUpdateStatus: (status: UpdaterStatus) => void,
  setUpdatingInfo: Updater<UpdatingInfo | null>,
) {
  const total = 10_000_000;
  const chunkSize = 200_000;
  const intervalMs = 500;

  setUpdateStatus("installing");
  setUpdatingInfo(() => ({ total, downloaded: 0 }));

  let downloaded = 0;
  const timer = setInterval(() => {
    downloaded += chunkSize;
    if (downloaded >= total) {
      downloaded = total;
      clearInterval(timer);
      setUpdatingInfo((draft) => {
        if (draft) draft.downloaded = downloaded;
      });
      setTimeout(() => setUpdateStatus("installed"), 200);
    } else {
      setUpdatingInfo((draft) => {
        if (draft) draft.downloaded = downloaded;
      });
    }
  }, intervalMs);
}
