export interface Plan {
  id: string;
  graphId: string;
  title: string;
  syncedAt: string;
}

export interface Task {
  id: string;
  graphId: string;
  planId: string;
  title: string;
  syncedAt: string;
}

export interface TimeEntry {
  id: string;
  planId: string;
  taskId: string | null;
  startTime: string;
  endTime: string | null;
  notes: string | null;
  createdAt: string;
}

export interface AuthStatus {
  isAuthenticated: boolean;
  userDisplayName: string | null;
}

export interface SyncResult {
  plansCount: number;
  tasksCount: number;
  syncedAt: string;
}

export interface ActiveTimerInfo {
  entryId: string;
  planId: string;
  taskId: string | null;
  startTime: string;
  elapsedSeconds: number;
}

export interface ReportEntry {
  taskTitle: string;
  planTitle: string;
  startTime: string;
  endTime: string;
  durationSeconds: number;
  notes: string | null;
}

export interface ReportResult {
  entries: ReportEntry[];
  grandTotalSeconds: number;
  subjectLabel: string;
}
