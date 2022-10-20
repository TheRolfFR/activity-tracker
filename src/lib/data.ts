import type { WeekStats } from '$bindings/WeekStats';
import type { ActivitySeries } from "$bindings/ActivitySeries";
import type { Measure } from "$bindings/Measure";

export interface Activity {
    clicks_per_minute: number,
    click_series: ActivitySeries<Measure<number>>,
    inputs_per_minute: number,
    input_series: ActivitySeries<Measure<number>>,
    adjusted: number
}

export interface Payload {
    activity: Activity,
    today: number,
    week_stats: WeekStats
}

export type WeekActivity = Array<Activity>;
