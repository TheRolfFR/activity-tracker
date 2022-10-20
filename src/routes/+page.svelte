<script lang="ts">
    import type { Event } from "@tauri-apps/api/event";
    import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { Measure } from "$bindings/Measure";

	import { onMount } from 'svelte';
    import Graph from '$components/graph.svelte';
	import Day from '$lib/islands/day.svelte';
	import Week from '$lib/islands/week.svelte';
	import type { Activity, Payload } from "$lib/data";

    let payload: Payload = {
        activity: {
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
            adjusted: 0
        },
        week_stats: {
            total: 0,
            done: 0
        }
    };
    
    let activity: Activity;
    let day_activity: ActivitySeries<Measure<number>> = {
        labels: {
            x: "act",
            y: "amount"
        },
        points: []
    };

    $: {
        activity = payload.activity;
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

    onMount(async () => {
        const { listen } = await import("@tauri-apps/api/event")

        listen('activity', (event: Event<Payload>) => {
            payload = event.payload;
            console.log('payload', payload);
            // @ts-ignore
            window.payload = payload;
        })
    })
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<Day data={day_activity} adjusted={activity.adjusted} stats={payload.today} />
<Week data={payload.week_stats} />

<div class="twice">
    <div>
        <Graph
            title={"Click activity"}
            data={activity.click_series}
            avg={activity.clicks_per_minute}
        />
    </div>
    <div>
        <Graph
            title={"Input activity"}
            data={activity.input_series}
            avg={activity.inputs_per_minute}
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