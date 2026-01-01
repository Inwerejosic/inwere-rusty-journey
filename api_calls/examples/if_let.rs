fn main() {
    let config: Option<u32> = Some(44);

    let mut processed_value = 0;

    if let Some(val) = config {
        processed_value = val * 2;
    }
}
