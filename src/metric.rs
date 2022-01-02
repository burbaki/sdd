pub mod metric_storage;

struct TimeSeries {
    time: u32,
    value: u32,
    metric_type: MetricType
}

pub enum MetricType {
    Write,
    Read
}