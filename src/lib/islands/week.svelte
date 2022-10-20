<script lang="ts">
	import type { WeekStats } from "$bindings/WeekStats";
	import Stat from "$components/stat.svelte";

    export let data: WeekStats;

    let done:  [number, number] = [0,0];
    let left:  [number, number] = [0,0];
    let total: [number, number] = [0,0];

    $: {
        total = [Math.floor(data.total/60), Math.round(data.total%60)]

        let done_min = data.done/60;
        done = [Math.floor(done_min/60), Math.round(done_min%60)]

        let secs = data.total - done_min;
        left = [Math.floor(secs/60), Math.round(secs%60)];
    }
</script>

<div class="flex">
    <Stat title='Week activity' value={done}></Stat>
    <Stat title='Time left' value={left}></Stat>
    <Stat title='Total' value={total}></Stat>
</div>