use std::io::BufReader;

use bitcoin::consensus::Decodable;

use bitcoin_slices::{bsl, Visit, Visitor};

struct CountTxs(pub usize);
impl Visitor for CountTxs {
    fn visit_block_begin(&mut self, total_transactions: usize) {
        self.0 = total_transactions;
    }
}

// let hash = "00000000000000000002a0b5db2a7f8d9087464c2586b546be7bce8eb53b8187";
// let url = format!("http://localhost:8332/rest/block/{}.bin?offset=0&length=10000", hash);


fn main() -> Result<(), ureq::Error> {
    let t0 = std::time::Instant::now();
    let mut blob = vec![];
    for i in 0..850000 {
        let res = ureq::get(&format!("http://localhost:8332/rest/blockhashbyheight/{}.hex", i))
            .call()?
            .into_string()?;
        let hash = res.trim_end();
        let url = format!("http://localhost:8332/rest/block/{}.bin", hash);
        let req = ureq::get(&url).call()?;
        let size = req.header("Content-Length").expect("no size").to_owned();
        // let mut reader = BufReader::new();
        // let block = bitcoin::Block::consensus_decode(&mut buf).expect("bad block");
        blob.clear();
        req.into_reader().read_to_end(&mut blob)?;
        let mut visit = CountTxs(0);
        bsl::Block::visit(&blob, &mut visit).expect("bad block");
        let dt = t0.elapsed().as_secs_f32();
        println!("{},{},{},{},{}", dt, hash, i, visit.0, size);
    }
    Ok(())
}
