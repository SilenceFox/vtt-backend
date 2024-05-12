use super::*;
fn get_headers(headers: &HeaderMap, header: &str) -> Option<i32> {
    headers
        .get(header)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<i32>().ok())
}

pub(crate) async fn faced_roll(headers: HeaderMap) -> impl IR {
    // How many times to roll
    let times = get_headers(&headers, "times").or(Some(1));
    // Dice roll range, D20, D12...
    let range = get_headers(&headers, "range").or(Some(20));

    // Dice rolling
    let roll = Roll::new().roll(DiceKind::Faced, range, times);

    Json(roll)
}

pub(crate) async fn fate_roll(headers: HeaderMap) -> impl IR {
    // parse headers and handle defaults
    let times = get_headers(&headers, "times").unwrap_or(1);
    // handle rolls
    let roll: Vec<RollResult> = (1..=times)
        .map(|_| {
            info!("Rolling fate dice {} times.", times);
            Roll::new().roll(DiceKind::Fate, None, None)
        })
        .collect();

    Json(roll)
}
