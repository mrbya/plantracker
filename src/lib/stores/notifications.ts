import { writable } from "svelte/store";

type NotificationType = "success" | "error" | "warning";

interface Notification {
  id: number;
  type: NotificationType;
  message: string;
}

export const notifications = writable<Notification[]>([]);

let nextId = 0;

function add(type: NotificationType, message: string): void {
  const id = ++nextId;
  notifications.update((n) => [...n, { id, type, message }]);
  setTimeout(
    () => notifications.update((n) => n.filter((x) => x.id !== id)),
    4000,
  );
}

export const addSuccess = (msg: string): void => add("success", msg);
export const addError = (msg: string): void => add("error", msg);
const addWarning = (msg: string): void => add("warning", msg);
