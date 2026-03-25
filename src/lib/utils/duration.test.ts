import { describe, it, expect } from "vitest";
import { formatDuration, formatDurationCSV } from "./duration";

describe("formatDuration", () => {
  it("formats sub-hour durations as minutes only", () => {
    expect(formatDuration(90)).toBe("1m");
    expect(formatDuration(3540)).toBe("59m");
  });

  it("formats hour + minutes", () => {
    expect(formatDuration(3600)).toBe("1h 0m");
    expect(formatDuration(9240)).toBe("2h 34m");
  });

  it("handles zero seconds", () => {
    expect(formatDuration(0)).toBe("0m");
  });
});

describe("formatDurationCSV", () => {
  it("formats as H:MM:SS with zero-padded minutes and seconds", () => {
    expect(formatDurationCSV(0)).toBe("0:00:00");
    expect(formatDurationCSV(3661)).toBe("1:01:01");
    expect(formatDurationCSV(9240)).toBe("2:34:00");
  });
});
