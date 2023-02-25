<script lang="ts">
    import type { Event } from "@tauri-apps/api/event";

	import { onMount } from "svelte";
    import days from "days-of-week";

	import Title from "$components/title.svelte";
    import type { Payload } from "$lib/data";
	import Day from "$lib/islands/day.svelte";
	import type { DayPayload } from "$bindings/DayPayload";

    let days_data = [] as DayPayload[];

    onMount(async () => {
        const { listen } = await import("@tauri-apps/api/event")

        listen('activity', (event: Event<Payload>) => {
            days_data = event.payload.week_payload;
            console.log(days_data)
        });

        const window_title = document.getElementById('window-title');
        if(!window_title) return;
        window_title.innerText = "Week data";
    })
</script>

<div id="container">
    {#each Object.entries(days_data) as [day_number, day]}
        <div class="item" class:nocontent={day.time == 0}>
            <Title>
                {days.english[(Number.parseInt(day_number,10)+1)%7]}
            </Title>
            <div class="content">
                {#if day.time == 0}
                    <Title>
                        <i class="nodata">No data provided for this day</i>
                    </Title>
                {:else}
                    <Day notadjusted activity={day.activity} stats={day.time} today_stats={day.stats} adjusted={day.adjusted} />
                {/if}
            </div>
        </div>
    {/each}
</div>

<style>
    #container > .item {
        display: inline-block;
        max-width:50%;
        width: 100%;
    }
    .content {
        min-height: 120px;
    }
    
    .item.nocontent .content {
        padding-top: 10px;
    }
</style>