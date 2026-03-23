export function clickOutside(
  node: HTMLElement,
  handler: () => void,
): { destroy(): void } {
  const onClick = (e: MouseEvent) => {
    if (!node.contains(e.target as Node)) handler();
  };
  document.addEventListener("mousedown", onClick, true);
  return {
    destroy: () => document.removeEventListener("mousedown", onClick, true),
  };
}
