<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
	import { Button, ContentDialog, TextBox } from "fluent-svelte";

    export let adjusttime: [number, number] = [0,0];

    export let openadjust: boolean;

    function submit() {
        if(adjusttime[0] < 0 || adjusttime[0] > 23 || 
           adjusttime[1] < 0 || adjusttime[1] > 59) return;
        
        const val = adjusttime[0]*60 + adjusttime[1];
        console.log(val);
        
        invoke('adjust', {
            val
        }).finally(() => {
            openadjust = false;
        });
    }
</script>

<ContentDialog bind:open={openadjust} title="Adjusted time" size="min">
    <form id="form" on:submit|preventDefault={submit}>
        <TextBox placeholder="h" type="number" min="0" max="23" bind:value={adjusttime[0]} clearButton={false} />
        <TextBox placeholder="m" type="number" min="0" max="59" bind:value={adjusttime[1]} clearButton={false} />
    </form>
    <div slot="footer" id="footer">
        <Button on:click={() => openadjust = false}>Close Dialog</Button>
        <Button on:click={submit} variant="accent">Submit</Button>
    </div>
</ContentDialog>

<style>
    #form {
        display: flex;
        gap: 5px;
    }
    #form :global( *) {
        text-align: center;
    }
    #footer {
        text-align: right;
    }
</style>