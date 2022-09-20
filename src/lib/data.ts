export interface DataLabels {
    x: string,
    y: string,
}

export interface Measure<T> {
    count: T,
    date: number,
}

export interface DataSeries<T> {
    points: Measure<T>[],
    labels: DataLabels,
}

type ClickSeries = DataSeries<number>;
type InputSeries = DataSeries<number>;

export interface Payload {
    click_series: ClickSeries,
    clicks_per_minute: number,
    input_series: InputSeries,
    inputs_per_minute: number
}