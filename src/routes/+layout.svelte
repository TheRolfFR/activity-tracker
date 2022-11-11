<script lang="ts">
	import 'fluent-svelte/theme.css';
	import '../app.css';

	import type { WebviewWindow } from '@tauri-apps/api/window';
	import { TextBlock } from 'fluent-svelte';
    import Favicon from '$lib/assets/favicon.png';
	import Close from '$lib/assets/close.svg';

	export const prerender = true;
	export const ssr = false;

	import { onMount } from 'svelte';

	let appWindow: WebviewWindow;
	onMount(async () => {
		appWindow = (await import('@tauri-apps/api/window')).appWindow;
	});

    let closeFocus: boolean = false;

    const handleClose = (e: Event) => {
        e.preventDefault()
        closeFocus = false;
        appWindow.hide()
    } 
</script>

<div id="title-bar" data-tauri-drag-region>
	<div id="title" data-tauri-drag-region>
        <img height="14" src={Favicon} alt="A"/><span>
            <TextBlock data-tauri-drag-region variant="bodyStrong" data:title="Activity tracker" title="" id="window-title">Activity tracker</TextBlock> <TextBlock variant="caption" id="version" style="opacity: 0.7"></TextBlock>
        </span>
	</div>
	<div>
		<div class="titlebar-button" id="titlebar-close" on:click={handleClose} on:mouseenter={() => closeFocus = true} on:mouseleave={() => closeFocus = false} class:hover={closeFocus}>
			<img src={Close} alt="close" />
		</div>
	</div>
</div>

<main>
	<slot />
</main>
<div class="bg" id="fg"></div>
<div class="bg" id="bg"></div>

<style>
	/* Some base styles to get things looking right. */
	:global(main) {
        --spacing: 8px;
        font-family: var(--fds-font-family-small);
	}
	:global(body) {
		color: var(--fds-text-primary);
	}
	:global(::selection) {
		background-color: transparent !important;
	}

	main {
		padding: 0 var(--spacing) var(--spacing);
		border-radius: 0.5em;
		position: fixed;
		overflow: auto;
        top: 37px;
        left: 5px;
        right: 5px;
        bottom: 5px;
        z-index: 2;
	}

    /* https://fluent-svelte.vercel.app/bloom-mica-light.png */
    #bg {
        background: url("https://fluent-svelte.vercel.app/bloom-mica-dark.png") 50%/cover no-repeat fixed;
        opacity: .7;
        z-index: -3;
    }

    #fg {
		background-color: var(--fds-solid-background-base);
        opacity: .7;
        z-index: -2;
		box-shadow: 0px 1px 2px 0px rgba(0, 0, 0, 0.3);
    }

    .bg {
        border-radius: 0.5em;
        position: fixed;
        top: 5px;
        left: 5px;
        right: 5px;
        bottom: 5px;
    }

	#title-bar {
		-webkit-padding-start: 0;
		padding-inline-start: 0;
		border-top-left-radius: 0;
		border-top-right-radius: 0;
		min-block-size: 32px;

		user-select: none;
		display: flex;
		align-items: center;
		justify-content: flex-end;
		position: fixed;
		top: 4px;
		left: 5px;
		right: 5px;
		z-index: 10;
	}

	#title {
		flex-grow: 1;
		padding-left: 8px;
	}

    #title img {
        margin-right: 8px;
    }

    #title > * {
        display: inline-block;
        vertical-align: middle;
    }

	.titlebar-button {
		display: inline-flex;
		justify-content: center;
		align-items: center;
		width: 30px;
		height: 30px;
        text-align: center;
	}
    .titlebar-button:last-child {
        border-top-right-radius: 0.5em;
    }
    .titlebar-button:last-child.hover {
        background: #D41325;
    }
</style>
