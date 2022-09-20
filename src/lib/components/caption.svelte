<script lang="ts">
    import { TextBlock } from "fluent-svelte";
</script>


<div class="caption">
    <TextBlock variant="caption"><slot></slot></TextBlock>
</div>

<style scoped>
    .caption {
        opacity: 0.7;
    }
</style>