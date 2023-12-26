extern crate speedtest_rs;

use speedtest_rs::speedtest::{get_configuration, get_server_list_with_config, get_best_server_based_on_latency, test_download_with_progress_and_config, test_upload_with_progress_and_config};

fn main() {
    let mut config = get_configuration().unwrap();

    let server_list = get_server_list_with_config(&config).unwrap();

    let best_server = get_best_server_based_on_latency(
        &server_list.servers
    ).unwrap();
    
    let download_measurement = test_download_with_progress_and_config(
        best_server.server, || {}, &mut config
    ).unwrap();

    let upload_measurement = test_upload_with_progress_and_config(
        best_server.server, || {}, &mut config
    ).unwrap();

    println!(
        "Download speed: {:.2} Megabytes per second",
        download_measurement.kbps() as f64 / 8000.0 
    );
    println!(
        "Upload speed: {:.2} Megabytes per second",
        upload_measurement.kbps() as f64 / 8000.0 
    );
}
