<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { Measure } from "$bindings/Measure";
    import type { DayStats } from "$bindings/DayStats";

	import Graph from "$components/graph.svelte";
	import Stat from "$components/stat.svelte";
	import ActivityGraph from "$components/activityGraph.svelte";
	import type { Activity } from "$bindings/Activity";

    export let activity: Activity;
    export let stats: number;
    export let today_stats: DayStats;

    export let adjusted: number = 0;
    export let notadjusted: boolean = false;

    let day_activity: ActivitySeries<Measure> = {
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

            let series_obj: Record<string, Measure> = {};
            const acts: ActivitySeries<Measure>[] = [activity.click_series, activity.input_series];

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

    $: stats_time = [Math.floor(stats/60), Math.round(stats%60)] as [number, number];

    let day_diff = 0;
    $: {
        day_diff = 0;
        if(today_stats.activities.length > 0) {
            const day_first = today_stats.activities[0].from;
            const day_last = today_stats.activities[today_stats.activities.length - 1].to;
            day_diff = day_last - day_first;
        }
        day_diff = day_diff/60;
    };
    $: day_diff_time = [Math.floor(day_diff/60), Math.round(day_diff%60)] as [number, number];

	const dispatch = createEventDispatcher();
    function openDialog() {
        if(!notadjusted)
            dispatch('adjust');
    }

    const adjusted_absolute = Math.abs(adjusted);
    const adjusted_time = [Math.floor(adjusted_absolute/60), Math.round(adjusted_absolute%60)] as [number, number];
    const adjusted_sign = adjusted >= 0 ? "+" : "-";
    const adjusted_text = `${adjusted_sign}${adjusted_time.map((v) => v < 10 ? String('0' + v) : String(v)).join('h')+'m'}`;
</script>

<div id="day">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div id="day-graph" role="button" tabindex="0" aria-roledescription="Switch to between graphs" on:click={() => visible = !visible}>
        <div style="display: {visible ? 'block' : 'none'};">
            <Graph title="Day activity" data={data} width={343} type="line" color="#39d353" />
        </div>
        <div style="display: {visible ? 'none' : 'block'};">
            <ActivityGraph activity_stats={today_stats.activities} activities={data} />
        </div>
    </div><div>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div class="pointer" on:click={openDialog}>
            <Stat title='Stats' value={stats_time} detail={adjusted_text} />
        </div>
        <hr>
        <Stat title="Day duration" value={day_diff_time} />
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