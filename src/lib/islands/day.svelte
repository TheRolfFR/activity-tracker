<script lang="ts">
    import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { Measure } from "$bindings/Measure";

	import Graph from "$components/graph.svelte";
	import Stat from "$components/stat.svelte";

    export let data: ActivitySeries<Measure<number>>;
    export let adjusted: number;
    export let stats: number;

    $: adjusted_time = [Math.floor(adjusted/60), Math.round(adjusted%60)] as [number, number];
    $: stats_time = [Math.floor(stats/60), Math.round(stats%60)] as [number, number];
</script>

<div id="day">
    <div id="day-graph">
        <Graph title="Day activity" data={data} width={343} type="line" />
    </div><div>
        <Stat title='Stats' value={stats_time} />
        <Stat title='Adjusted' value={adjusted_time} />
    </div>
</div>

<style scoped>
    #day {
        display: flex;
        flex: 1 1 auto;
        justify-content: top;
    }
    #day > div:first-child {
        flex-grow: 1;
        margin-right: var(--spacing);
    }
</style>