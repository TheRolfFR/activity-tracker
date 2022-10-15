<script lang="ts">
	import type { WeekStats } from "$bindings/WeekStats";
	import Stat from "$components/stat.svelte";

    export let data: WeekStats;

    $: week_hours = Object.entries(data)
    .map(
        ([key,secs]: [string,number]): [string, [number, number]] => {
            return [key, [Math.floor(secs/60), secs%60]]
        })
    .reduce((acc: Record<string, [number, number]>, cur) => {
        acc[cur[0]] = cur[1];
        return acc
    }, {})
</script>

<div class="flex">
    <Stat title='Week activity' value={week_hours['done']}></Stat>
    <Stat title='Time left' value={week_hours['left']}></Stat>
    <Stat title='Total' value={week_hours['total']}></Stat>
</div>