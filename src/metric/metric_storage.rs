use super::MetricType;

pub trait MetricStorage {
     fn list_metric(&self) -> Vec<String>;

     fn put_metric(&mut self, series_name: &str, bit_amount: u32, timestamp: u32, metric_type: MetricType);
}
