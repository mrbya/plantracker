<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import Card from "$lib/components/ui/Card.svelte";

  const { Story } = defineMeta({
    title: "PlanTracker/UI/Card",
    component: Card,
    tags: ["autodocs"],
    argTypes: {
      title: { control: "text" },
    },
  });
</script>

<Story name="WithTitle">
  <Card title="Active Timer">
    <p>Timer has been running for 1h 23m on "Implement auth flow".</p>
  </Card>
</Story>

<Story name="WithoutTitle">
  <Card>
    <p>Card body without a header section.</p>
  </Card>
</Story>

<Story name="WithComplexContent">
  <Card title="Recent Entries">
    <table class="entry-table">
        <thead>
          <tr>
            <th>Task</th>
            <th>Duration</th>
            <th>Date</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>Implement auth flow</td>
            <td>2h 30m</td>
            <td>2026-03-24</td>
          </tr>
          <tr>
            <td>Write migration</td>
            <td>1h 15m</td>
            <td>2026-03-23</td>
          </tr>
          <tr>
            <td>Graph API client</td>
            <td>3h 00m</td>
            <td>2026-03-22</td>
          </tr>
        </tbody>
      </table>
  </Card>
</Story>

<style>
  .entry-table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-sm);
    color: var(--text);
  }

  .entry-table th {
    text-align: left;
    padding: 0.3rem 0.5rem;
    color: var(--text-muted);
    font-weight: 500;
    border-bottom: 1px solid var(--border);
  }

  .entry-table td {
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--border);
  }

  .entry-table tr:last-child td {
    border-bottom: none;
  }
</style>
