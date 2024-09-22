<script lang="ts">
	import { TextBlock } from 'fluent-svelte';
    // @ts-ignore
	import { LinkedChart, LinkedLabel, LinkedValue } from 'svelte-tiny-linked-charts';
    // @ts-ignore
    import { v4 as uuidv4 } from "uuid";
	import Caption from '$components/caption.svelte';
	import type { ActivitySeries } from '$bindings/ActivitySeries';
	import type { Measure } from '$bindings/Measure';
	import Title from './title.svelte';
	import { getHour } from '$lib/helpers/date';

    export let title: string;
    export let data: ActivitySeries<Measure>;
    export let color: string = '#117DBB';
    export let avg: number | undefined = undefined;
    export let width: number = 212; // default 150
    export let type: string = "bar";

    const uid = uuidv4();

    $: hasData = data.points.length > 0;
    $: labels = data.points.map(e => getHour(e.date));
    $: values = data.points.map(e => e.count);

    $: lastPoint = data.points.length ? {
        count: values[values.length - 1],
        date: labels[labels.length - 1]
    } : {
        count: 0,
        date: '00:00 AM'
    };

    let hovered: boolean = false;
</script>

<div id="title">
    <Title>{title}</Title>
    {#if avg }
        <div class="spacer"></div>
        <TextBlock variant="bodyStrong">{avg.toFixed(2)}/min</TextBlock>
    {/if}
</div>
<div id="graph-container">
    <div id="graph-part">
        <LinkedChart
            dispatchEvents
            on:hover={ () => hovered = true }
            on:blur={ () => hovered = false }
            {labels}
            {values}
            {type}
            {width}
            fill={color}
            linked={uid}
            uid={uid}
        />
    </div>
    <div id="label-part">
            <Caption>
                {#if hasData}
                    {#if hovered}
                        Selected: <LinkedLabel linked={uid} />: <LinkedValue uid={uid} />
                    {:else}
                        Last minute: {lastPoint.count} ({data.points.length})
                    {/if}
                {/if}
            </Caption>
    </div>
</div>

<style scoped>
    #title {
        padding: 0 0 8px;
        opacity: 0.7;
        display: flex;
        align-items: center;
    }

    .spacer {
        flex-grow: 1;
    }

    #graph-part {
        border-radius: 4px;
        background: rgba(0,0,0,0.2);
        padding: 10px 10px 7px;
    }
</style>