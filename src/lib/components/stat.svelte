<script lang="ts">
	import { TextBlock } from "fluent-svelte";
	import Title from "./title.svelte";

    export let title: string = 'Stats';
    export let value: [number, number] = [0,0];
    export let detail: string | undefined = undefined;

    $: time = value.map((v) => v < 10 ? String('0' + v) : String(v)).join('h')+'m';
</script>

<div id="day-stats">
    <div id="title">
        <Title>{title}</Title>
        {#if typeof(detail) === "string" }
            <div class="spacer"></div>
            <TextBlock variant="bodyStrong">{detail}</TextBlock>
        {/if}
    </div>
    <TextBlock variant="title">{time}</TextBlock>
</div>

<style scoped>
    #title {
        display: flex;
    }

    .spacer {
        flex-grow: 1;
    }
</style>