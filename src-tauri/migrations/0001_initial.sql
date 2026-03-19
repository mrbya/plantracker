CREATE TABLE IF NOT EXISTS plans (
    id        TEXT PRIMARY KEY,
    graph_id  TEXT NOT NULL UNIQUE,
    title     TEXT NOT NULL,
    synced_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id        TEXT PRIMARY KEY,
    graph_id  TEXT NOT NULL UNIQUE,
    plan_id   TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    title     TEXT NOT NULL,
    synced_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS time_entries (
    id         TEXT PRIMARY KEY,
    task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    start_time TEXT NOT NULL,
    end_time   TEXT,
    notes      TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_time_entries_task_id  ON time_entries(task_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_start_time ON time_entries(start_time);
CREATE INDEX IF NOT EXISTS idx_tasks_plan_id          ON tasks(plan_id);
