export function mountMolecule(
  canvas: HTMLCanvasElement,
  pdb: string,
  onError?: (error: unknown) => void,
): Promise<() => void>;
