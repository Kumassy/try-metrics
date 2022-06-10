use std::{thread::sleep, time::Duration};
use metrics::{gauge, counter, describe_counter, describe_gauge, increment_gauge, decrement_gauge};
use metrics_exporter_prometheus::PrometheusBuilder;


fn main() {
    let builder = PrometheusBuilder::new();
    builder.install().expect("failed to install recorder/exporter");

    describe_counter!("some_metric_name", "The iterations of the TCP server event loop so far.");
    describe_gauge!("some_gauge_name", "The iterations of the TCP server event loop so far.");

    loop {
        sleep(Duration::from_secs(1));

        // increment_gauge!("some_gauge_name", 42.2222);
        decrement_gauge!("some_gauge_name", 42.2222);
        counter!("some_metric_name", 12);
    }
}
