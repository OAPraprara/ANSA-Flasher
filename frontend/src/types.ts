export interface FirmwareVersion {
  version: string;
  channel: string;
  url: string;
  sha256: string;
  signature_url: string;
  size_bytes: number;
}

export interface Board {
  id: string;
  name: string;
  mcu: string;
  transport: "swd" | "dfu" | string;
  versions: FirmwareVersion[];
}

export interface Manifest {
  boards: Board[];
}
