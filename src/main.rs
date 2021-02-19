use std::collections::HashMap;

fn main() -> Result<(), anyhow::Error> {

    let mut relevant_channels = vec![];
    let mut lock_file_name = "./flake.lock".to_string();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "-f" {
            lock_file_name = args.next().expect("Expected a path to the flake.lock");
        } else {
            relevant_channels.push(arg.to_string());
        }
    }


    let body = reqwest::blocking::get(
        "https://monitoring.nixos.org/prometheus/api/v1/query?query=channel_update_time",
    )?
    .json::<serde_json::Value>()?;
    // println!("{:#?}", body);

    let channels = &body["data"]["result"];

    let mut dates = HashMap::new();

    for (obj, channel) in channels
        .as_array()
        .expect("This should be an array")
        .iter()
        .filter_map(|o| {
            let ch_n = o["metric"]["channel"]
                .as_str()
                .expect("This should be a string");
            if relevant_channels.iter().any(|ch| ch == &ch_n) {
                Some((o, ch_n))
            } else {
                None
            }
        })
    {
        dates.insert(
            channel.to_string(),
            obj["value"][1]
                .as_str()
                .expect("This should always be a string").parse::<u64>()?,
        );
    }

    let lock_file = std::fs::File::open(lock_file_name)?;

    let lock_obj = serde_json::from_reader::<_, serde_json::Value>(&lock_file)?;

    let mut locked_dates = HashMap::new();

    for obj in &mut lock_obj["nodes"]
        .as_object()
        .expect("Always an object")
        .values()
    {
        let name = &obj["original"]["ref"];
        if relevant_channels.iter().any(|r_ch| r_ch == name) {
            let name = name.as_str().expect("This should always be a string");
            locked_dates.insert(
                name.to_string(),
                obj["locked"]["lastModified"]
                    .as_u64()
                    .expect("This should always be a timestamp as an integer"),
            );
        }
    }

    for (ch_name, last_rel) in dates {
        if let Some(locked_rel) = locked_dates.get(&ch_name) {
            if last_rel > *locked_rel {
                println!("{} got an update!", ch_name);
            }
        }
    }
    Ok(())
}
