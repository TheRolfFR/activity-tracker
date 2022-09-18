<script lang="ts">
    import Graph from '../lib/graph.svelte';

	import { onMount } from 'svelte';

    import { TextBlock } from "fluent-svelte";
    let value = "Default value";

    let payload: any = {};

    onMount(async () => {
        const { listen } = await import("@tauri-apps/api/event")

        listen('activity', (event) => {
            payload = event.payload;
        })
    })
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="twice">
    <div>
        <div class="caption"><TextBlock variant="caption">Clicks/min</TextBlock></div>
        <TextBlock variant="title">{(payload.clicks_per_minute|| 0).toFixed(3)}</TextBlock>
    </div>
    <div>
        <div class="caption"><TextBlock variant="caption">Inputs/min</TextBlock></div>
        <TextBlock variant="title">{(payload.inputs_per_minute|| 0).toFixed(3)}</TextBlock>
    </div>
</div>

<!-- <p>{JSON.stringify(payload, null, 2)}</p> -->

<Graph title={"Click activity"} values={payload?.click_series?.points||[]} labels={payload?.click_series?.points||[]} />
<Graph
    title={"Input activity"}
    values={payload?.input_series?.points||[]}
    labels={payload?.input_series?.points||[]}
    color={"#8B12AE"}
/>

<style>
    p {
        margin: 0;
    }
    .caption {
        opacity: 0.7;
    }
    .twice {
        display: flex;
        flex: 1 1 auto;
    }
    .twice > * {
        flex-grow: 1;
        margin: 0 0 5px;
    }
</style>