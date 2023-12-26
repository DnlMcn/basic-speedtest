extern crate speedtest_rs;

use std::io;
use std::io::prelude::*;
use speedtest_rs::speedtest::{
    get_best_server_based_on_latency, get_configuration, get_server_list_with_config,
    test_download_with_progress_and_config, test_upload_with_progress_and_config,
};

fn main() {
    println!(
        "Welcome to haomakk's speedtester!\nNow testing your internet speed... please wait.\n"
    );

    let mut config = get_configuration().unwrap();

    let server_list = get_server_list_with_config(&config).unwrap();

    let best_server = get_best_server_based_on_latency(&server_list.servers).unwrap();

    let download_measurement =
        test_download_with_progress_and_config(best_server.server, || {}, &mut config).unwrap();

    let upload_measurement =
        test_upload_with_progress_and_config(best_server.server, || {}, &mut config).unwrap();

    println!(
        "Download speed: {:.2} Megabytes per second",
        download_measurement.kbps() as f64 / 8000.0
    );
    println!(
        "Upload speed: {:.2} Megabytes per second",
        upload_measurement.kbps() as f64 / 8000.0
    );

    println!(
        "\nThe program has finished. Thank you for using haomakk's speedtester!\n"
    );

    let mut stdin = io::stdin();

    println!("Press Enter to close this terminal instance.");

    let _ = stdin.read(&mut [0u8]).unwrap();
}
