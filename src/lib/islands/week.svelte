<script lang="ts">
	import type { DayPayload } from "$bindings/DayPayload";
	import type { WeekStats } from "$bindings/WeekStats";
	import Stat from "$components/stat.svelte";
    import { total_store } from "$lib/stores";
	import { createEventDispatcher, onDestroy } from "svelte";

    let total_week: number;
    const unsubscribe = total_store.subscribe(value => {
        total_week = value;
    });
    onDestroy(unsubscribe);

	const dispatch = createEventDispatcher();
    function openWeek() {
        dispatch('openweek');
    }

    export let data: WeekStats;
    export let week_payload: DayPayload[];

    let done:  [number, number] = [0,0];
    let left:  [number, number] = [0,0];
    let total: [number, number] = [0,0];

    $: {
        total = [Math.floor(total_week/60), Math.round(total_week%60)]

        let done_min = data.done/60;
        done = [Math.floor(done_min/60), Math.round(done_min%60)]

        let secs = total_week - done_min;
        left = [Math.floor(secs/60), Math.round(secs%60)];
    }

    let week_diff = 0;
    $: {
        week_diff = 0;
        for (let day of Object.values(week_payload)) {
            let day_diff = 0;
            let stats = day.stats;
            if(stats.activities.length > 0) {
                const day_first = stats.activities[0].from;
                const day_last = stats.activities[stats.activities.length - 1].to;
                day_diff = day_last - day_first;
            }
            week_diff += day_diff;
        }
        week_diff /= 60;
    };
    $: week_diff_time = [Math.floor(week_diff/60), Math.round(week_diff%60)] as [number, number];
</script>

<div id="week-stats" class="flex">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div  class="pointer" on:click={openWeek}><Stat title='Week activity' value={done}></Stat></div>
    <Stat title='Week duration' value={week_diff_time}></Stat>
    <Stat title='Time left' value={left}></Stat>
    <Stat title='Total' value={total}></Stat>
</div>

<style>
    #week-stats {
        margin-top: var(--spacing);
    }
</style>