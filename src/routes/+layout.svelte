<script lang="ts">
	import 'fluent-svelte/theme.css';
	import '../app.css';

	import type { WebviewWindow } from '@tauri-apps/api/window';
	import type { Event as TauriEvent } from "@tauri-apps/api/event";
	import { TextBlock } from 'fluent-svelte';
    import Favicon from '$lib/assets/favicon.png';
	import Close from '$lib/assets/close.svg';

	export const prerender = true;
	export const ssr = false;

	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Payload } from '$src/lib/data';

	let appWindow: WebviewWindow;
	let version: string = "";
	onMount(async () => {
		appWindow = (await import('@tauri-apps/api/window')).appWindow;

		const { listen } = await import("@tauri-apps/api/event")

        listen('activity', (event: TauriEvent<Payload>) => {
            version = event.payload.version;
        })
    });

    let closeFocus: boolean = false;

    const handleClose = (e: Event) => {
        e.preventDefault()
        closeFocus = false;
        appWindow.hide()
    }

	const openAlt = async () => {
		invoke('open_menu_handle')
		.catch(err => {
			console.error(err)
		})
	}
</script>

<div id="title-bar" data-tauri-drag-region>
	<div id="title" data-tauri-drag-region>
		<button id="icon" on:click={openAlt} on:keypress={() => {}}>
			<img height="14" src={Favicon} alt="A" />
		</button>
		<span>
			<TextBlock data-tauri-drag-region variant="bodyStrong" id="window-title">Activity tracker</TextBlock>
			<TextBlock variant="caption" id="version" data-tauri-drag-region style="opacity: 0.7">{version}</TextBlock>
		</span>
	</div>
	<div>
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<button class="titlebar-button" id="titlebar-close" on:click={handleClose} on:mouseenter={() => closeFocus = true} on:mouseleave={() => closeFocus = false} class:hover={closeFocus}>
			<img src={Close} alt="close" />
		</button>
	</div>
</div>

<main>
	<slot />
</main>

<style>
	button {
		border: 0px none;
		background: transparent;
		padding: 0;
	}
	:global(*) {
		-webkit-box-sizing: border-box;
		-moz-box-sizing: border-box;
		box-sizing: border-box;
		outline: none;
		--title-bar-height: 30px;
	}
	/* Some base styles to get things looking right. */
	:global(main) {
		--window-offset: 0;
		--window-border-radius: 0;
        --spacing: 8px;
        font-family: var(--fds-font-family-small);
	}
	:global(body) {
		color: var(--fds-text-primary);
		display: flex;
		flex-direction: column;
	}
	:global(::selection) {
		background-color: transparent !important;
	}

    :global(.content-dialog-smoke) {
        border-radius: var(--window-border-radius);
        left: var(--window-offset);
        right: var(--window-offset);
        bottom: var(--window-offset);
        top: var(--window-offset);
        block-size: auto !important;
        inline-size: auto !important;
        inset-inline-start: var(--window-offset) !important;
        inset-block-start: var(--window-offset) !important;
    }

	main {
		padding: 0 var(--spacing) var(--spacing);
		border-radius: var(--window-border-radius);
		overflow: auto;
        z-index: 2;
		flex-grow: 1;
	}

	#title-bar {
		-webkit-padding-start: 0;
		padding-inline-start: 0;
		border-top-left-radius: 0;
		border-top-right-radius: 0;
		height: 30px;

		user-select: none;
		display: flex;
		align-items: center;
		justify-content: flex-end;

		position: relative;
	}
	#title-bar :global(img) {
		user-select: none;
	}

	#title {
		flex-grow: 1;
		padding-left: 8px;
	}

    #icon {
		height: 14px;
		width: 12.31px;
        margin-right: 8px;
    }

    #title > *, span :global( > *) {
        display: inline-block;
        vertical-align: middle;
    }

	:global(#version) {
		opacity: 0 !important;
		transition: opacity 0.2s ease;
	}
	#title #icon:hover :global(+ span #version) {
		opacity: inherit !important;
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
		transform: translateY(-1px);
        border-top-right-radius: var(--window-border-radius);
    }
    .titlebar-button:last-child.hover {
        background: #D41325;
    }

	:global(#version, #version *) {
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
</style>
