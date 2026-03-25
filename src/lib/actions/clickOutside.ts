/**
 * Svelte use-action that fires a callback when the user clicks outside a node.
 *
 * Attaches a `mousedown` listener (rather than `click`) in the **capture
 * phase** so that the handler runs before any child `click` handlers.  This
 * prevents the common race condition where a child button dismisses a dropdown
 * and the subsequent bubbled click immediately re-opens it.
 *
 * Usage:
 * ```svelte
 * <script>
 *   import { clickOutside } from '$lib/actions/clickOutside';
 *   let open = false;
 * </script>
 *
 * <div use:clickOutside={() => (open = false)}>
 *   <!-- dropdown content -->
 * </div>
 * ```
 *
 * The returned `destroy` method is called automatically by Svelte when the
 * element is removed from the DOM.
 *
 * @param node - The element to watch; clicks outside this element fire `handler`.
 * @param handler - Callback invoked when a `mousedown` event occurs outside `node`.
 * @returns An object with a `destroy` method that removes the event listener.
 */
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
