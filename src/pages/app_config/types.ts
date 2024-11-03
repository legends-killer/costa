export interface RuntimeMap {
  [key: string]: Runtime;
}

export interface Runtime {
  build: string;
  version: string;
  state: string;
  deletable: boolean;
  kind: string;
  lastUsedAt: string;
  mountPath: string;
  identifier: string;
  path: string;
  platformIdentifier: string;
  runtimeBundlePath: string;
  runtimeIdentifier: string;
  signatureState: string;
  sizeBytes: number;
}
