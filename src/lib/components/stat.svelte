<script lang="ts">
	import { TextBlock } from "fluent-svelte";
	import Title from "./title.svelte";

    export let title: string = 'Stats';
    export let value: [number, number] = [0,0];
    export let large: boolean = false;

    $: time = value.map((v) => v < 10 ? String('0' + v) : String(v)).join('h')+'m';
</script>

<div id="day-stats">
    {#if large }
        <div id="title">
            <Title>{title}</Title>
        </div>
    {:else}
        <Title>{title}</Title>
    {/if}
    <TextBlock variant="title">{time}</TextBlock>
</div>

<style>
    #title {
        margin: 8px 0;
    }
</style>