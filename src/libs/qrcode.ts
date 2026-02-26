import { invoke } from "@tauri-apps/api/core";
import QRCodeStyling, { Options } from "qr-code-styling";

export interface QRPayload {
  text: string;
  eclevel: "L" | "M" | "Q" | "H";
}

export interface QRCode {
  code: string;
  width: number;
}

export type QRError = "DataTooLong" |
  "InvalidVersion" |
  "UnsupportedCharacterSet" |
  "InvalidEciDesignator" |
  "InvalidCharacter";

export type QRResult = { success: QRCode } | { error: QRError };

async function genRawQrcode(text: QRPayload["text"], eclevel: QRPayload["eclevel"]): Promise<QRResult> {
  return await invoke("generate_qrcode", { payload: { text, eclevel } });
}

class WrappedQRCode {

  qrcode: QRCode;

  constructor(qrcode: QRCode) {
    this.qrcode = qrcode;
  }

  getModuleCount(): number {
    return this.qrcode.width;
  }

  isDark(row: number, col: number): boolean {
    const index = row * this.qrcode.width + col;
    return this.qrcode.code.charAt(index) === "1";
  }
}

const drawTypes = {
  canvas: "canvas",
  svg: "svg"
};

class HackedQRCodeStyling extends QRCodeStyling {

  private _qrcode: QRCode;

  constructor(qrcode: QRCode, options?: Partial<Options>) {
    super(options);
    this._qrcode = qrcode;
    this.update();
  }

  update(): void {
    QRCodeStyling._clearContainer(this._container);

    // anyだけど**多分**大丈夫。QRCodeStyling内で使われる関数はQRCodeWrapperにも実装してるからね〜 エラーが出たら直す。
    this._qr = this._qrcode && new WrappedQRCode(this._qrcode) as any;

    if (this._options.type === drawTypes.canvas) {
      this._setupCanvas();
    } else {
      this._setupSvg();
    }

    this.append(this._container);
  }
}

export type GenQRCodeOptions = Omit<Options, "data" | "qrOptions">;

export async function genQRCode(text: QRPayload["text"], eclevel: QRPayload["eclevel"], options: GenQRCodeOptions): Promise<HackedQRCodeStyling> {
  const qr_result = await genRawQrcode(text, eclevel);
  if (!("success" in qr_result)) {
    throw new Error(`Failed to generate QR code: ${qr_result.error}`);
  }
  const qrcode = new HackedQRCodeStyling(qr_result.success, options);
  return qrcode;
}