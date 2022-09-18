<script lang="ts">
	import 'fluent-svelte/theme.css';
	import '../app.css';

	import type { WebviewWindow } from '@tauri-apps/api/window';
	import { TextBlock } from 'fluent-svelte';

	export const prerender = true;
	export const ssr = false;

	import { onMount } from 'svelte';

	let appWindow: WebviewWindow;
	onMount(async () => {
		appWindow = (await import('@tauri-apps/api/window')).appWindow;
	});
</script>

<div id="title-bar" data-tauri-drag-region>
	<div id="title" data-tauri-drag-region>
		<TextBlock data-tauri-drag-region variant="bodyStrong">Activity tracker</TextBlock>
	</div>
	<div>
		<div class="titlebar-button" id="titlebar-minimize" on:click={() => appWindow.minimize()}>
			<img src="https://api.iconify.design/mdi:window-minimize.svg?color=white" alt="minimize" />
		</div>
		<div class="titlebar-button" id="titlebar-maximize" on:click={() => appWindow.toggleMaximize()}>
			<img src="https://api.iconify.design/mdi:window-maximize.svg?color=white" alt="maximize" />
		</div>
		<div class="titlebar-button" id="titlebar-close" on:click={() => appWindow.close()}>
			<img src="https://api.iconify.design/mdi:close.svg?color=white" alt="close" />
		</div>
	</div>
</div>

<main>
	<slot />
</main>
<div id="bg"></div>

<style>
	/* Some base styles to get things looking right. */
	:global(main) {
		/* background-color: var(--fds-solid-background-base); */
        font-family: var(--fds-font-family-small);
	}
	:global(body) {
		color: var(--fds-text-primary);
	}

	main {
		padding: 0 8px 8px;
		border-radius: 0.5em;
		box-shadow: 0px 1px 2px 0px rgba(0, 0, 0, 0.3);
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
        border-radius: 0.5em;
        position: fixed;
        top: 5px;
        left: 5px;
        right: 5px;
        bottom: 5px;
        opacity: .5;
        z-index: -1;
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
		top: 5px;
		left: 5px;
		right: 5px;
		z-index: 10;
	}

	#title {
		flex-grow: 1;
		padding-left: 10px;
	}

	.titlebar-button {
		display: inline-flex;
		justify-content: center;
		align-items: center;
		width: 30px;
		height: 30px;
	}
	.titlebar-button:hover {
		background: #5bbec3;
	}
</style>
