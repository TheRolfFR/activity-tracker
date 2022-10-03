import adapter from '@sveltejs/adapter-static'; // This was changed from adapter-auto
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),
        alias: {
            '$bindings/*': './src-tauri/bindings/*',
            '$components/*': './src/lib/components/*'
          }
	}
};

export default config;
