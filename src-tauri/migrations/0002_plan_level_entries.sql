-- Add plan_id column (nullable for backfill)
ALTER TABLE time_entries
    ADD COLUMN plan_id TEXT REFERENCES plans(id) ON DELETE CASCADE;

-- Backfill plan_id from the task's plan
UPDATE time_entries
SET plan_id = (SELECT plan_id FROM tasks WHERE tasks.id = time_entries.task_id)
WHERE task_id IS NOT NULL;

-- Recreate table: plan_id NOT NULL, task_id nullable with ON DELETE SET NULL
CREATE TABLE time_entries_new (
    id         TEXT PRIMARY KEY,
    plan_id    TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    task_id    TEXT            REFERENCES tasks(id) ON DELETE SET NULL,
    start_time TEXT NOT NULL,
    end_time   TEXT,
    notes      TEXT,
    created_at TEXT NOT NULL
);

INSERT INTO time_entries_new (id, plan_id, task_id, start_time, end_time, notes, created_at)
SELECT id, plan_id, task_id, start_time, end_time, notes, created_at
FROM time_entries;

DROP TABLE time_entries;
ALTER TABLE time_entries_new RENAME TO time_entries;

CREATE INDEX IF NOT EXISTS idx_time_entries_plan_id    ON time_entries(plan_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_task_id    ON time_entries(task_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_start_time ON time_entries(start_time);
