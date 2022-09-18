<script lang="ts">
	import { TextBlock } from 'fluent-svelte';
    // @ts-ignore
	import { LinkedChart, LinkedLabel, LinkedValue } from 'svelte-tiny-linked-charts';
    // @ts-ignore
    import { v4 as uuidv4 } from "uuid";

	let data = {
		'2005-01-01': 25,
		'2005-01-02': 20,
		'2005-01-03': 18,
		'2005-01-04': 17,
		'2005-01-05': 21
	};

    const uid = uuidv4();

    export let title: string;
    export let values: Array<number>;
    export let labels: Array<string>;
    export let color: string = '#117DBB';
</script>

<div id="title">
    <TextBlock variant="bodyStrong">{title}</TextBlock>
</div>
<div id="graph-container">
    <div id="graph-part">
        <LinkedChart {labels} {values} fill={color} linked={uid} uid={uid} />
    </div>
    <div id="label-part">
        <TextBlock variant="caption">
            <LinkedLabel linked={uid} />
        </TextBlock><br>
        <TextBlock variant="title">
            <LinkedValue uid={uid} />
        </TextBlock>
    </div>
</div>

<style scoped>
    #title {
        margin: 5px 0 8px;
    }
    #graph-container {
        display: flex;
        flex: 1 1 auto;
    }
    #graph-part {
        border-radius: 4px;
        background: rgba(0,0,0,0.2);
        padding: 10px 10px 7px;
        flex-grow: 1;
    }
    #label-part {
        width: 50px;
        margin-left: 20px;
    }
</style>