extern crate reqwest;

fn download_pom(group: &str, name: &str, version: &str) -> Result<String, ()> {
    // 1. Construct the URL
    let url = format!("http://central.maven.org/maven2/{}/{}/{}/{}-{}.pom",
                      group.replace(".", "/"), name, version, name, version);
    // 2. Download the URL
    match reqwest::get(url.as_str()) {
        Result::Ok(mut body) => {
           match body.text() {
               Result::Ok(txt) => return Ok(txt),
               Result::Err(_) => return Err(()),
           }
        }
        Result::Err(_) => return Err(())
    }
}



fn main() {
    let xml: String = download_pom("com.google.protobuf", "protobuf-java", "3.6.1").unwrap();
    println!("XML = {}", xml);
}
