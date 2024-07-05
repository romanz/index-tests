use log::*;

fn main() -> Result<(), ureq::Error> {
    let mut builder = env_logger::Builder::from_default_env();
    builder.default_format().format_timestamp_micros();
    builder.init();

    let mut args = std::env::args().skip(1);
    let url = args.next().expect("no URL");
    let count: usize = args.next().expect("no count").parse().expect("bad count");
    let mut blob = vec![];
    let t0 = std::time::Instant::now();
    let mut bytes = 0;
    let agent = ureq::AgentBuilder::new().build();
    for i in 0..count {
        blob.clear();
        info!("sending request {}", i);
        let res = agent.get(&url).call()?;
        assert_eq!(res.status(), 200);
        res.into_reader().read_to_end(&mut blob)?;
        bytes += blob.len();
        let dt = t0.elapsed().as_secs_f32();
        info!("{:.3}MB, {:.3}s, {}", bytes as f32 / 1e6, dt, i);
    }
    Ok(())
}
