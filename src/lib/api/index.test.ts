import { describe, it, expect } from 'vitest';
import { mockIPC } from '@tauri-apps/api/mocks';
import { startTimer, stopTimer, createManualEntry, generateReport } from './index';

// Minimal TimeEntry fixture — matches the Rust struct shape (camelCase)
function makeEntry(overrides: Record<string, unknown> = {}) {
    return {
        id: '1',
        planId: 'p1',
        taskId: null,
        startTime: '2024-03-15T10:00:00Z',
        endTime: null,
        notes: null,
        createdAt: '2024-03-15T10:00:00Z',
        ...overrides,
    };
}

describe('startTimer', () => {
    it('passes planId and null taskId when no taskId provided', async () => {
        let payload: Record<string, unknown> = {};
        mockIPC((cmd, args) => {
            if (cmd === 'start_timer') {
                payload = args as Record<string, unknown>;
                return makeEntry();
            }
        });
        await startTimer('p1');
        expect(payload.planId).toBe('p1');
        expect(payload.taskId).toBeNull();
    });

    it('passes taskId when provided', async () => {
        let payload: Record<string, unknown> = {};
        mockIPC((cmd, args) => {
            if (cmd === 'start_timer') {
                payload = args as Record<string, unknown>;
                return makeEntry({ taskId: 'tid' });
            }
        });
        await startTimer('p1', 'tid');
        expect(payload.planId).toBe('p1');
        expect(payload.taskId).toBe('tid');
    });
});

describe('stopTimer', () => {
    it('returns completed entry with non-null endTime', async () => {
        mockIPC((cmd) => {
            if (cmd === 'stop_timer') {
                return makeEntry({ endTime: '2024-03-15T11:00:00Z' });
            }
        });
        const entry = await stopTimer();
        expect(entry.endTime).not.toBeNull();
    });
});

describe('createManualEntry', () => {
    it('maps undefined taskId to null', async () => {
        let payload: Record<string, unknown> = {};
        mockIPC((cmd, args) => {
            if (cmd === 'create_manual_entry') {
                payload = args as Record<string, unknown>;
                return makeEntry({ endTime: '2024-03-15T11:00:00Z' });
            }
        });
        await createManualEntry({
            planId: 'p1',
            startTime: '2024-03-15T10:00:00Z',
            endTime: '2024-03-15T11:00:00Z',
        });
        expect(payload.taskId).toBeNull();
    });

    it('passes taskId through when provided', async () => {
        let payload: Record<string, unknown> = {};
        mockIPC((cmd, args) => {
            if (cmd === 'create_manual_entry') {
                payload = args as Record<string, unknown>;
                return makeEntry({ taskId: 't1', endTime: '2024-03-15T11:00:00Z' });
            }
        });
        await createManualEntry({
            planId: 'p1',
            taskId: 't1',
            startTime: '2024-03-15T10:00:00Z',
            endTime: '2024-03-15T11:00:00Z',
        });
        expect(payload.taskId).toBe('t1');
    });
});

describe('generateReport', () => {
    it('sends null planId and taskId when not provided', async () => {
        let payload: Record<string, unknown> = {};
        mockIPC((cmd, args) => {
            if (cmd === 'generate_report') {
                payload = args as Record<string, unknown>;
                return { entries: [], grandTotalSeconds: 0, subjectLabel: 'All' };
            }
        });
        await generateReport({ fromYear: 2024, fromMonth: 1, toYear: 2024, toMonth: 3 });
        expect(payload.planId).toBeNull();
        expect(payload.taskId).toBeNull();
    });
});
