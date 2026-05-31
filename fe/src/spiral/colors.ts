const RANGE_PALETTE = [
  "#e63946",
  "#f4a261",
  "#e9c46a",
  "#2a9d8f",
  "#264653",
  "#9d4edd",
  "#48cae4",
  "#e76f51",
  "#588157",
  "#5e548e",
  "#bc4749",
];

export const rangeColor = (index: number): string => {
  return RANGE_PALETTE[index % RANGE_PALETTE.length] ?? "#888";
};
