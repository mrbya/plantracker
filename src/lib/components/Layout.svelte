<script lang="ts">
    import TimeTracking from "../../views/TimeTracking.svelte";
    import ManualEntry from "../../views/ManualEntry.svelte";
    import Reports from "../../views/Reports.svelte";
    import Settings from "../../views/Settings.svelte";
    import { userDisplayName, logout } from "$lib/stores/auth";

    type View = "time-tracking" | "manual-entry" | "reports" | "settings";

    let activeView = $state<View>("time-tracking");

    const navItems: { id: View; icon: string; label: string }[] = [
        { id: "time-tracking", icon: "", label: "Time Tracking" },
        { id: "manual-entry", icon: "", label: "Manual Entry" },
        { id: "reports", icon: "󱛣", label: "Reports" },
        { id: "settings", icon: "", label: "Settings" },
    ];

    const initials = $derived(
        $userDisplayName
            ? $userDisplayName
                  .split(" ")
                  .map((w) => w[0])
                  .join("")
                  .slice(0, 2)
                  .toUpperCase()
            : "?",
    );
</script>

<div class="shell">
    <nav class="sidebar">
        <ul class="nav-list">
            {#each navItems as item (item.id)}
                <li>
                    <button
                        class="nav-btn"
                        class:active={activeView === item.id}
                        title={item.label}
                        onclick={() => (activeView = item.id)}
                    >
                        <span class="icon">{item.icon}</span>
                    </button>
                </li>
            {/each}
        </ul>

        <div class="sidebar-bottom">
            <div class="avatar" title={$userDisplayName ?? "User"}>
                {initials}
            </div>
            <button class="nav-btn sign-out" title="Sign out" onclick={logout}>
                <span class="icon">󰍃</span>
            </button>
        </div>
    </nav>

    <main class="content">
        {#if activeView === "time-tracking"}
            <TimeTracking />
        {:else if activeView === "manual-entry"}
            <ManualEntry />
        {:else if activeView === "reports"}
            <Reports />
        {:else if activeView === "settings"}
            <Settings />
        {/if}
    </main>
</div>

<style>
    .shell {
        display: flex;
        height: 100vh;
        overflow: hidden;
    }

    .sidebar {
        width: 64px;
        flex-shrink: 0;
        background: var(--bg-raised);
        border-right: 1px solid var(--border);
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        padding: 0.5rem 0;
    }

    .nav-list {
        list-style: none;
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        padding: 0 0.5rem;
    }

    .nav-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        aspect-ratio: 1;
        background: transparent;
        border: none;
        border-radius: var(--radius);
        color: var(--text-muted);
        cursor: pointer;
        transition:
            background 0.15s,
            color 0.15s;
    }

    .nav-btn:hover {
        background: var(--bg-input);
        color: var(--text);
    }

    .nav-btn.active {
        background: color-mix(in srgb, var(--accent) 15%, transparent);
        color: var(--accent);
    }

    .icon {
        font-size: 1.25rem;
        line-height: 1;
    }

    .sidebar-bottom {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;
        padding: 0 0.5rem;
    }

    .avatar {
        width: 36px;
        height: 36px;
        border-radius: 50%;
        background: var(--bg-input);
        border: 1px solid var(--border);
        color: var(--text-muted);
        font-size: var(--font-size-sm);
        font-weight: 600;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: default;
        user-select: none;
    }

    .sign-out {
        color: var(--text-muted);
    }

    .sign-out:hover {
        color: var(--danger);
        background: color-mix(in srgb, var(--danger) 10%, transparent);
    }

    .content {
        flex: 1;
        overflow-y: auto;
        background: var(--bg);
    }
</style>
