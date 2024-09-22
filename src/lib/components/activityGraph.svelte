<script lang="ts">
	import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { DayActivityStat } from "$bindings/DayActivityStat";
	import type { Measure } from "$bindings/Measure";
    import { colorRamp } from "$lib/helpers/color";
    import { getHour, padTo2Digits } from "$lib/helpers/date";
	import { onMount } from "svelte";

    export let activities: ActivitySeries<Measure>;
    export let activity_stats: DayActivityStat[];

    interface DateActivity {
        from: Date,
        to: Date,
        count: number
    }

    interface ActivityShort {
        from: number,
        to: number,
        title: string
    }

    let date_activity_stats: DateActivity[] = [];
    let activity_shorts: Record<number, ActivityShort[]> = {};

    $: {
        activity_shorts = {};
        let min_date = Infinity;
        let max_date = 0;
        date_activity_stats = (activity_stats || []).map((e)=>{
            min_date = Math.min(min_date, e.from);
            max_date = Math.max(max_date, e.to);

            const date_from = new Date(e.from * 1000);
            const date_to   = new Date(e.to   * 1000);

            const df_h = date_from.getHours();
            const df_min = date_from.getMinutes();
            const dt_h = date_to.getHours();
            const dt_min = date_to.getMinutes();

            const title = `${getHour(e.from)} - ${getHour(e.to)}`;

            let h = df_h;
            while(h <= dt_h) {
                // calcul des minutes d'un short
                let st_min = h == df_h ? df_min : 0;
                let en_min = h == dt_h ? dt_min : 59;

                // ajout de l'heure dans les shorts
                if(!(h in activity_shorts)) activity_shorts[h] = [];

                // ajout du short
                activity_shorts[h].push({
                    from: st_min,
                    to: en_min,
                    title
                });

                h++;
            }
            

            return {
                from: date_from,
                to: date_to,
                count: e.count
            };
        });

        onMount(() => {
            // @ts-ignore
            window.date_activity_stats = date_activity_stats;
            // @ts-ignore
            window.activity_shorts = activity_shorts;
        });
    }

    const ramp = colorRamp('#2d333b', '#39d353', 6);

    const TOTAL = 100;
    const N = 60;

    let activity_props: Record<number, // hour
        Record<number, // minute
            { style: string, title: string }
        >
    > = {};
    $: {
        let min = Infinity;
        let max = 0;
        let activities_per_hour: Record<number, Measure[]> = {};
        
        activities.points.forEach(p => {
            let d_h = new Date(p.date *1000).getHours();
            if(!(d_h in activities_per_hour)) activities_per_hour[d_h] = [];

            activities_per_hour[d_h].push(p);

            min = Math.min(min, p.count);
            max = Math.max(max, p.count);
        });

        function mapIndex(val: number) {
            const amp = max - min;
            const adjusted = val - min;
            const percent = adjusted / amp;

            return Math.floor(percent*ramp.length);
        }

        activity_props = {};
        activities.points.forEach(p => {
            let d =  new Date(p.date *1000);
            let d_h = d.getHours();
            let d_m = d.getMinutes();
            if(!(d_h in activity_props)) activity_props[d_h] = {};

            let color = ramp[mapIndex(p.count)];
            activity_props[d_h][d_m] = ({
                style: cssStringify({ 'background-color': color }),
                title: getHour(p.date) + ' - ' + p.count
            });
        });
    }
    $: hours = Object.keys(activity_props);

    function cssStringify(obj: {[key: string]: string}) {
        return Object.keys(obj).map(k => `${k}: ${obj[k]}`).join(';')
    }

    function getShorts(hour: string): ActivityShort[] {
        return activity_shorts[Number.parseInt(hour, 10)] || []
    }

    function getActivityProps(hour: string, minute: number) {
        return activity_props[Number.parseInt(hour,10)][minute] || {}
    }
</script>

<div class="activities">
    {#each hours as hour}
        <div class="hour">
            <div class="hour-text">{padTo2Digits(hour)}</div>
            <div class="activity-group">
                {#each getShorts(hour) as short}
                    <div
                        class="short"
                        style:left={(6*short.from) +'px'}
                        style:width={(6*(short.to+1-short.from)-2)+'px'}
                        title={short.title}
                    ></div>
                {/each}
                {#each Array(60) as _ , i}
                    <div class="activity" {...getActivityProps(hour, i)}></div>
                {/each}
            </div>
        </div>
    {/each}
    <div class="group"></div>
</div>

<style scoped>
    .activities {
        flex-direction: column;
        height: 100%;
        width: 100%;
        gap: 2px;
    }
    .activities, .hour, .activity-group {
        display: flex;
        flex: 1 1 auto;
    }
    .hour, .activity-group {
        height: 100%;
    }
    .hour {
        align-items: flex-start;
        position: relative;
        width: 100%;
        text-align: center;
    }
    .hour-text {
        position: relative;
        display: inline-block;
        z-index: 2;
        font-size: 16px;
        line-height: 16px;
        height: 16px;
        width: 17px;
        padding-right: 3px;
        text-align: right;
    }
    .activity-group {
        position: relative;
        gap: 2px;
        justify-content: flex-start;
    }
    .activity {
        width: 4px;
        height: 100%;
        border-radius: 2px;
    }
    .hour:hover .short {
        opacity: 0.3;
    }
    .short {
        position: absolute;
        height: 4px;
        top: 4px;
        background: #ccc;
        border-radius: 2px;
        opacity: 0;
        z-index: 3;
        transition: opacity 0.2s ease;
    }
</style>