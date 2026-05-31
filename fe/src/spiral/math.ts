export type Point = { x: number; y: number };

export type SpiralConfig = {
  turns: number;
  rStart: number;
  rEnd: number;
};

export const DEFAULT_CONFIG: SpiralConfig = {
  turns: 2,
  rStart: 120,
  rEnd: 1600,
};

export const thetaRange = (cfg: SpiralConfig = DEFAULT_CONFIG): [number, number] => {
  return [0, cfg.turns * 2 * Math.PI];
};

export const thetaFromUnit = (u: number, cfg: SpiralConfig = DEFAULT_CONFIG): number => {
  const [a, b] = thetaRange(cfg);
  return a + u * (b - a);
};

export const radiusAtTheta = (theta: number, cfg: SpiralConfig = DEFAULT_CONFIG): number => {
  const [a, b] = thetaRange(cfg);
  const slope = (cfg.rEnd - cfg.rStart) / (b - a);
  return cfg.rStart + slope * (theta - a);
};

export const pointAtTheta = (theta: number, cfg: SpiralConfig = DEFAULT_CONFIG): Point => {
  const r = radiusAtTheta(theta, cfg);
  return { x: r * Math.cos(theta), y: r * Math.sin(theta) };
};

export const pointAtThetaOffset = (
  theta: number,
  radialOffset: number,
  cfg: SpiralConfig = DEFAULT_CONFIG,
): Point => {
  const r = radiusAtTheta(theta, cfg) + radialOffset;
  return { x: r * Math.cos(theta), y: r * Math.sin(theta) };
};

const polylinePath = (steps: number, pt: (i: number) => Point): string => {
  let d = "";
  for (let i = 0; i <= steps; i++) {
    const { x, y } = pt(i);
    d += i === 0 ? `M ${x.toFixed(2)} ${y.toFixed(2)}` : ` L ${x.toFixed(2)} ${y.toFixed(2)}`;
  }
  return d;
};

export const spiralPath = (cfg: SpiralConfig = DEFAULT_CONFIG, stepsPerTurn = 80): string => {
  const totalSteps = cfg.turns * stepsPerTurn;
  return polylinePath(totalSteps, (i) => pointAtTheta(thetaFromUnit(i / totalSteps, cfg), cfg));
};

export const arcPath = (
  thetaA: number,
  thetaB: number,
  cfg: SpiralConfig = DEFAULT_CONFIG,
  stepsPerTurn = 80,
): string => {
  const span = Math.abs(thetaB - thetaA);
  const turns = span / (2 * Math.PI);
  const steps = Math.max(8, Math.ceil(turns * stepsPerTurn));
  return polylinePath(steps, (i) => {
    const theta = thetaA + (i / steps) * (thetaB - thetaA);
    return pointAtTheta(theta, cfg);
  });
};

export const parseDateTime = (input: string): number => {
  const hasTz = /[zZ]$|[+-]\d\d:?\d\d$/.test(input);
  const t = Date.parse(hasTz ? input : input + "Z");
  if (Number.isNaN(t)) {
    throw new Error(`invalid datetime: ${input}`);
  }
  return t;
};
