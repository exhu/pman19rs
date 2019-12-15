
pub fn dosmth() {
    let msg = if cfg!(feature = "feature_a") {
        "feature A" 
    } else {
        "feature B"
    };
    println!("{}", msg);
}