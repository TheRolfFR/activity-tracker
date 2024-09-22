<script>
	import { TextBlock } from "fluent-svelte";
    export let italic = false;
</script>

<div class="title" class:italic={italic}>
    <TextBlock variant="bodyLarge"><slot></slot></TextBlock>
</div>

<style scoped>
    .title {
        opacity: 0.7;
    }
    .title :global(*) {
        line-height: 1 !important;
    }
    .italic {
        font-style: italic;
    }
</style>