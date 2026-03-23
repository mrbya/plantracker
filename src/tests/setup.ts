import { randomFillSync } from "crypto";
import { clearMocks } from "@tauri-apps/api/mocks";
import "@testing-library/jest-dom";

// jsdom does not ship WebCrypto — polyfill it so @tauri-apps/api/mocks works
Object.defineProperty(window, "crypto", {
  value: {
    getRandomValues: (buffer: BufferSource) => randomFillSync(buffer as Buffer),
  },
  writable: true,
});

// Reset all Tauri IPC mocks between tests — prevents state leaking across files
afterEach(() => {
  clearMocks();
});
