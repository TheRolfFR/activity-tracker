<script lang="ts">
    import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { Measure } from "$bindings/Measure";
    import type { DayStats } from "$bindings/DayStats";

	import Graph from "$components/graph.svelte";
	import Stat from "$components/stat.svelte";
	import ActivityGraph from "$components/activityGraph.svelte";

    export let data: ActivitySeries<Measure<number>>;
    export let adjusted: number;
    export let stats: number;
    export let today_stats: DayStats;

    let visible: boolean = false;

    $: adjusted_time = [Math.floor(adjusted/60), Math.round(adjusted%60)] as [number, number];
    $: stats_time = [Math.floor(stats/60), Math.round(stats%60)] as [number, number];
</script>

<div id="day">
    <div id="day-graph" on:click={() => visible = !visible}>
        <div style="display: {visible ? 'block' : 'none'};">
            <Graph title="Day activity" data={data} width={343} type="line" />
        </div>
        <div style="display: {visible ? 'none' : 'block'};">
            <ActivityGraph activity_stats={today_stats.activities} activities={data} />
        </div>
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
    #day #day-graph {
        align-self: stretch;
        flex-shrink: 1;
        height: 132px;
        /* background: rgba(255,255,255,0.3); */
    }
    #day-graph > div {
        height: 100%;
    }
    #day > div:first-child {
        flex-grow: 1;
        margin-right: var(--spacing);
    }
</style>