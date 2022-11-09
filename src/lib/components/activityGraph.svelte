<script lang="ts">
	import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { DayActivityStat } from "$bindings/DayActivityStat";
	import type { Measure } from "$bindings/Measure";
    import { colorRamp } from "$lib/helpers/color";
    import { getHour, padTo2Digits } from "$lib/helpers/date";

    export let activities: ActivitySeries<Measure<number>>;
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
        date_activity_stats = activity_stats.map((e)=>{
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
                    from: st_min/59,
                    to: en_min/59,
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
    }

    const ramp = colorRamp('#2d333b', '#39d353', 6);

    let activities_percented: Record<number, {
        top: string,
        height: string,
        'background-color': string,
        title: string
    }[]> = {};
    $: {
        let min = Infinity;
        let max = 0;
        let activities_per_hour: Record<number, Measure<number>[]> = {};
        
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

        activities_percented = {};
        activities.points.forEach(p => {
            let d =  new Date(p.date *1000);
            let d_h = d.getHours();
            let d_m = d.getMinutes();
            if(!(d_h in activities_percented)) activities_percented[d_h] = [];

            let top = d_m/60;
            let height = 1/60;
            let color = ramp[mapIndex(p.count)];
            activities_percented[d_h].push({
                top: top*100 + '%',
                height: height*100 + '%',
                'background-color': color,
                title: getHour(p.date) + ' - ' + p.count
            });
        });
        
        console.log(activity_shorts)
        console.log(date_activity_stats)
    }

    function cssStringify(obj: {[key: string]: string}) {
        return Object.keys(obj).map(k => `${k}: ${obj[k]}`).join(';')
    }

    function getShorts(hour: string): ActivityShort[] {
        return activity_shorts[Number.parseInt(hour, 10)] || []
    }
</script>

<div class="activities">
    {#each Object.entries(activities_percented) as [hour,group]}
        <div class="hour">
            <div class="hour-text">{padTo2Digits(hour)}</div>
            {#each getShorts(hour) as short}
                <div
                    class="short"
                    style:top={short.from*100+'%'}
                    style:height={(short.to-short.from)*100+'%'}
                    title={short.title}
                ></div>
            {/each}
            <div class="activity-group">
                {#each group as activity}
                    <div class="activity" style={cssStringify(activity)} title={activity.title}></div>
                {/each}
            </div>
        </div>
    {/each}
    <div class="group"></div>
</div>

<style scoped>
    .activities {
        height: 100%;
        width: 100%;
    }
    .activities, .hour, .activity-group {
        display: flex;
        flex: 1 1 auto;
    }
    .hour, .activity-group {
        position: relative;
        height: 100%;
    }
    .hour {
        align-items: flex-start;
        width: 12px;
        text-align: center;
    }
    .hour-text {
        position: relative;
        display: inline-block;
        z-index: 2;
    }
    .activity-group {
        margin: 0 3px;
    }
    .activity {
        position: absolute;
        left: 0;
        right: 0;
        border-radius: 2px;
    }
    .short {
        position: absolute;
        width: 4px;
        left: 4px;
        background: #39d353;
    }
</style>