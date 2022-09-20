<script lang="ts">
	import { TextBlock } from 'fluent-svelte';
    // @ts-ignore
	import { LinkedChart, LinkedLabel, LinkedValue } from 'svelte-tiny-linked-charts';
    // @ts-ignore
    import { v4 as uuidv4 } from "uuid";
	import Caption from '$lib/components/caption.svelte';
	import type { DataSeries } from '$lib/data';

    export let title: string;
    export let data: DataSeries<number>;
    export let color: string = '#117DBB';

    const uid = uuidv4();


    function padTo2Digits(num: number) {
        return String(num).padStart(2, '0');
    }

    const getHour = (s: number) => {
        let now = new Date(s*1000);
        
        return padTo2Digits(now.getHours())
         + ':' + padTo2Digits(now.getMinutes());
    };
    
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
    <TextBlock variant="bodyLarge">{title}</TextBlock>
</div>
<div id="graph-container">
    <div id="graph-part">
        <LinkedChart
            dispatchEvents
            on:hover={ () => hovered = true }
            on:blur={ () => hovered = false } 
            {labels}
            {values}
            fill={color}
            linked={uid}
            uid={uid}
        />
    </div>
    <div id="label-part">
        <Caption>
            {#if hovered}
                Selected: <LinkedLabel linked={uid} />: <LinkedValue uid={uid} />
            {/if}
        </Caption>
    </div>
</div>

<style scoped>
    #title {
        margin: 8px 0;
        opacity: 0.7;
    }
    
    #graph-part {
        border-radius: 4px;
        background: rgba(0,0,0,0.2);
        padding: 10px 10px 7px;
    }
</style>