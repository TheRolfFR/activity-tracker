<script>
	import Graph from "$lib/components/graph.svelte";
	import Stat from "$lib/components/stat.svelte";
</script>

<div id="day">
    <div id="day-graph">
        <Graph title="Day activity" data={{
            points: [],
            labels: {
                x: "",
                y: "",
            },
        }} />
    </div><Stat title='Stats' value={[9,5]} />
</div>

<style scoped>
    #day {
        display: flex;
        flex: 1 1 auto;
        justify-content: top;
    }
    #day > div:first-child {
        flex-grow: 1;
        margin-right: 20px;
    }
</style>