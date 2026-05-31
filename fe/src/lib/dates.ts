import {parseDateTime} from "../spiral/math";

const MONTH_NAMES = [
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
];

export const formatDate = (ms: number): string => {
  const d = new Date(ms);
  return `${d.getUTCDate()} ${MONTH_NAMES[d.getUTCMonth()]} ${d.getUTCFullYear()}`;
};

export const formatYear = (ms: number): string => {
  return new Date(ms).getUTCFullYear().toString();
};

// Try to coerce user input into Jiff's serde DateTime format (YYYY-MM-DDTHH:MM:SS).
// Returns null if the input cannot be parsed.
export const normalizeDateTime = (input: string): string | null => {
  const s = input.trim();
  if (!s) return null;
  const isoDate = /^(-?\d{4,6})-(\d{2})-(\d{2})$/;
  const isoDateTime = /^-?\d{4,6}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?$/;
  const yearMonth = /^(-?\d{4,6})-(\d{2})$/;
  const bareYear = /^(-?\d{1,6})$/;
  if (isoDateTime.test(s)) return s;
  if (isoDate.test(s)) return `${s}T00:00:00`;
  const ym = s.match(yearMonth);
  if (ym) return `${s}-01T00:00:00`;
  const by = s.match(bareYear);
  if (by) {
    const year = parseInt(by[1], 10);
    const padded = padSignedYear(year);
    return `${padded}-01-01T00:00:00`;
  }
  return null;
};

const padSignedYear = (year: number): string => {
  if (year < 0) {
    return `-${String(-year).padStart(6, "0")}`;
  }
  return String(year).padStart(4, "0");
};

// Strip the trailing T00:00:00 if present, for cleaner display in inputs.
export const displayDateTime = (s: string): string => {
  return s.endsWith("T00:00:00") ? s.slice(0, -"T00:00:00".length) : s;
};

export const tryParseToMs = (s: string): number | null => {
  try {
    return parseDateTime(s);
  } catch {
    return null;
  }
};
