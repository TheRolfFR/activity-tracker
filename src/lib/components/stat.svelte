<script lang="ts">
	import { TextBlock } from "fluent-svelte";


    export let title: string = 'Stats';
    export let value: [number, number] = [0,0];

    $: time = value.map((v) => v < 10 ? String('0' + v) : String(v)).join('h')+'m';
</script>

<div id="day-stats">
    <div id="title">
        <TextBlock variant="bodyLarge">{title}</TextBlock>
    </div>
    <TextBlock variant="title">{time}</TextBlock>
</div>

<style scoped>
    #title {
        margin: 5px 0 0;
        opacity: 0.7;
    }
</style>