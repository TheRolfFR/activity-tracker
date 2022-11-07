import { writable, type Writable } from 'svelte/store';

function isBrowser() {
    return typeof(localStorage) != 'undefined' && typeof(window) != 'undefined'
}

function createStore<T, V>(key: string, default_value: T, parse: (v: string) => T, obj: (v: Writable<T>) => V): V {
    let initial: T;
    if(isBrowser()) {
        const val = localStorage.getItem(key);
        initial = val ? parse(val) : default_value;
    } else {
        initial = default_value;
    }

    const my_writable = writable(initial);
    my_writable.subscribe(v => {
        if (isBrowser()) {
            localStorage.setItem(key, String(v));
        }
    });
    return obj(my_writable);
}

// 2250 = 37h30 in minutes
export const total_store = createStore('TOTAL_HOURS', 2250, v => {
    return Number.parseInt(v, 10)
}, writable => {
    const { subscribe, set } = writable;

    return {
        subscribe,
        set
    }
});