<script lang="ts">
	import type { ActivitySeries } from "$bindings/ActivitySeries";
    import type { DayActivityStat } from "$bindings/DayActivityStat";
	import type { Measure } from "$bindings/Measure";
    import { colorRamp } from "$lib/helpers/color";
    import { getHour } from "$lib/helpers/date";

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

    let activities_percented: Record<number,[number, number, string][]> = {};
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
            activities_percented[d_h].push([top, height, color]);
        });

        console.log(activities_percented)
    }
</script>

<div class="activities">
    <!-- {#each groups as group}
        <div class="activity-group">
            {#each group as activity}
                <div class="activity" {...activity}></div>
            {/each}
        </div>
    {/each} -->
    <div class="group"></div>
</div>

<style scoped>
    .activities, .activity-group {
        display: flex;
        flex: 1 1 auto;
        gap: 1px;
    }
    .activity-group {
        flex-direction: column;
        align-items: flex-end;
    }
    .activity {
        border-radius: 2px;
    }
</style>