<script lang="ts">
	import Stat from "$lib/components/stat.svelte";
</script>

<div class="flex">
    <Stat title='Week activity' value={[28,52]}></Stat>
    <Stat title='Time left' value={[8,25]}></Stat>
    <Stat title='Total' value={[37,30]}></Stat>
</div>