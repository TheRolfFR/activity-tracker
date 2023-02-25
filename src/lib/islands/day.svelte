<script lang="ts">
    import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { Measure } from "$bindings/Measure";
    import type { DayStats } from "$bindings/DayStats";

    import { createEventDispatcher } from 'svelte';

	import Graph from "$components/graph.svelte";
	import Stat from "$components/stat.svelte";
	import ActivityGraph from "$components/activityGraph.svelte";
	import type { Activity } from "$bindings/Activity";

    export let activity: Activity;
    export let stats: number;
    export let today_stats: DayStats;

    export let adjusted: number = 0;
    export let notadjusted: boolean = false;
	const dispatch = createEventDispatcher();
    function openDialog() {
        if(!notadjusted)
            dispatch('adjust');
    }

    let day_activity: ActivitySeries<Measure<number>> = {
        labels: {
            x: "act",
            y: "amount"
        },
        points: []
    };

    $: {
        if(activity) {
            activity.click_series.points.sort((a, b) => a.date - b.date);
            activity.input_series.points.sort((a, b) => a.date - b.date);

            let series_obj: Record<string, Measure<number>> = {};
            const acts: ActivitySeries<Measure<number>>[] = [activity.click_series, activity.input_series];

            acts.forEach(act => {
                act.points.forEach(measure => {
                    if(! (measure.date in series_obj) ) {
                        series_obj[measure.date] = {
                            count: 0,
                            date: measure.date
                        };
                    }

                    series_obj[measure.date].count+=measure.count;
                });
            });

            // sort by increasing date
            day_activity.points = Object.values(series_obj).sort((a, b) => a.date - b.date);
        }
    }
    $: data = day_activity;

    let visible: boolean = false;

    $: adjusted_time = [Math.floor(adjusted/60), Math.round(adjusted%60)] as [number, number];
    $: stats_time = [Math.floor(stats/60), Math.round(stats%60)] as [number, number];
</script>

<div id="day">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div id="day-graph" on:click={() => visible = !visible}>
        <div style="display: {visible ? 'block' : 'none'};">
            <Graph title="Day activity" data={data} width={343} type="line" color="#39d353" />
        </div>
        <div style="display: {visible ? 'none' : 'block'};">
            <ActivityGraph activity_stats={today_stats.activities} activities={data} />
        </div>
    </div><div>
        <Stat title='Stats' value={stats_time} />
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div on:click={openDialog}>
            <Stat title='Adjusted' value={adjusted_time} />
        </div>
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
        overflow: auto;
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