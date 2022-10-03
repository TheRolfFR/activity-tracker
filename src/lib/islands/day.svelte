<script>
	import Graph from "$components/graph.svelte";
	import Stat from "$components/stat.svelte";
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
    </div><div>
        <Stat title='Stats' value={[9,5]} />
        <Stat title='Adjusted' value={[1,5]} />
    </div>
</div>

<style scoped>
    #day {
        display: flex;
        flex: 1 1 auto;
        justify-content: top;
    }
    #day > div:first-child {
        flex-grow: 1;
        margin-right: var(--spacing);
    }
</style>