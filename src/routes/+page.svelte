<script lang="ts">
	import { onMount } from 'svelte';
    import Graph from '$lib/components/graph.svelte';
	import type { Payload } from '$lib/data';
	import Day from '$lib/islands/day.svelte';
	import Week from '$lib/islands/week.svelte';

    let payload: Payload = {
        clicks_per_minute: 0.0,
        click_series: {
            points: [],
            labels: {
                x: '',
                y: '',
            },
        },
        inputs_per_minute: 0.0,
        input_series: {
            points: [],
            labels: {
                x: '',
                y: '',
            },
        },
    };

    onMount(async () => {
        const { listen } = await import("@tauri-apps/api/event")

        listen('activity', (event) => {
            // @ts-ignore
            payload = event.payload;
        })
    })
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<Day />
<Week />

<div class="twice">
    <div>
        <Graph
            title={"Click activity"}
            data={payload.click_series} />
    </div>
    <div>
        <Graph
            title={"Input activity"}
            data={payload.input_series}
            color={"#8B12AE"}
        />
    </div>
</div>

<style>
    .twice {
        display: flex;
        flex: 1 1 auto;
        margin: -5px;
    }
    .twice > * {
        flex-grow: 1;
        margin: 5px;
    }
</style>