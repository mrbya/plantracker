<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import Select from "$lib/components/ui/Select.svelte";

  const planOptions = [
    { value: "p1", label: "Sprint 12" },
    { value: "p2", label: "Backlog" },
    { value: "p3", label: "Q2 Roadmap" },
  ];

  const manyOptions = [
    { value: "p1", label: "Sprint 12" },
    { value: "p2", label: "Backlog" },
    { value: "p3", label: "Q2 Roadmap" },
    { value: "p4", label: "Infrastructure" },
    { value: "p5", label: "Security Hardening" },
    { value: "p6", label: "Design System" },
    { value: "p7", label: "Mobile Release" },
    { value: "p8", label: "API v2 Migration" },
    { value: "p9", label: "Performance" },
    { value: "p10", label: "Accessibility Audit" },
    { value: "p11", label: "Onboarding Flow" },
    { value: "p12", label: "Analytics Dashboard" },
  ];

  const { Story } = defineMeta({
    title: "PlanTracker/UI/Select",
    component: Select,
    tags: ["autodocs"],
  });
</script>

<Story
  name="WithPlaceholder"
  args={{ options: planOptions, placeholder: "Select a plan…", value: "" }}
/>

<Story name="PreSelected" args={{ options: planOptions, value: "p2" }} />

<Story
  name="ManyOptions"
  args={{ options: manyOptions, placeholder: "Select a plan…", value: "" }}
/>
