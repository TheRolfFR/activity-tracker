<script lang="ts">
    import type { Event } from "@tauri-apps/api/event";
    import { invoke } from '@tauri-apps/api/tauri';

    import { onMount } from 'svelte';

	import AdjustDialog from "$lib/islands/adjustDialog.svelte";
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
            done: 0
        },
        today: 0,
        today_stats: {
            activities: [],
            duration: 0
        },
        version: '',
        week_payload: []
    };
    
    let activity: Activity;
    $: today_adjusted = activity ? activity.adjusted : 0;
    onMount(async () => {
        const { listen } = await import("@tauri-apps/api/event")

        listen('activity', (event: Event<Payload>) => {
            payload = event.payload;
            // @ts-ignore
            window.payload = payload;
            activity = payload.activity;
            
            let versionElement = document.getElementById('version');
            if(!versionElement || versionElement.innerText.trim() !== '') return;
            versionElement.innerText = payload.version;
        })
    })

    let openadjust = false;
    let adjusttime = [0,0] as [number,number];
    function openAdjustDialog() {
        adjusttime = [Math.floor(activity.adjusted/60), activity.adjusted%60]
        openadjust = true;
    }

    function openWeekWindow() {
        invoke('open_week_window')
        .then(() => {
            console.warn('Opened window')
        }).catch((err) => {
            console.error(err)
            console.error('Failed to open window')
        });
    }
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<AdjustDialog bind:openadjust bind:adjusttime />

<div id="container">
    <div id="top">
        <Day activity={activity} adjusted={today_adjusted} stats={payload.today} today_stats={payload.today_stats} on:adjust={openAdjustDialog} />
    </div>
    
    <div id="bottom">
        <Week data={payload.week_stats} on:openweek={openWeekWindow} />
        
        <div class="twice">
            {#if activity}
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
            {/if}
        </div>
    </div>
</div>

<style>
    #container {
        display: flex;
        flex-direction: column;
        height: 100%;
    }
    #top {
        flex-grow: 1;
    }
    #top :global(> *) {
        height: 100%;
    }
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