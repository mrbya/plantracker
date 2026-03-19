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
  taskId: string;
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

export interface MonthlyTotal {
  year: number;
  month: number;
  totalSeconds: number;
}

export interface ReportResult {
  monthlyTotals: MonthlyTotal[];
  grandTotalSeconds: number;
  subjectLabel: string;
}
