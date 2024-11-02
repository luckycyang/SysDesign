fn main() {
    let cache_dirs = dirs::cache_dir().unwrap();
    println!("the default cache dir: {:?}", cache_dirs);
}
