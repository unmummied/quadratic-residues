mod prime;
use prime::*;

fn main() {
    let n = 27;
    println!("  n: {n}");
    println!(" qr: {:?}", n.qr_pp().unwrap());
    println!("len: {}", n.qr_pp().unwrap().len());
    assert_eq!(n.qr_pp().unwrap().len(), n.fast_ord_qr_pp().unwrap() as _);
    assert_eq!(
        n.qr_pp().unwrap().len(),
        n.fast_ord_qr_pp_v2().unwrap() as _,
    );
}
