<script>
	import { TextBlock } from "fluent-svelte";
</script>

<div class="title">
    <TextBlock variant="bodyLarge"><slot></slot></TextBlock>
</div>

<style scoped>
    .title {
        opacity: 0.7;
    }
</style>