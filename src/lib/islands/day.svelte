<script lang="ts">
    import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { Measure } from "$bindings/Measure";

	import Graph from "$components/graph.svelte";
	import Stat from "$components/stat.svelte";

    export let data: ActivitySeries<Measure<number>>;
</script>

<div id="day">
    <div id="day-graph">
        <Graph title="Day activity" data={data} />
    </div><div>
        <Stat title='Stats' value={[9,5]} />
        <Stat title='Adjusted' value={[1,5]} />
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