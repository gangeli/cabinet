extern crate reqwest;

use std::string::String;


///
/// Download the POM file at the given group+name+version, and read it as a
/// simple String.
///
/// @param group The group of the artifact we are downloading. E.g., `com.google.protobuf`
/// @param name The name of the artifact we are downloading. E.g., `protobuf-java`
/// @param version The version of the artifact we are downloading. E.g., `3.6.1`
///
/// @return The text of the POM file (as an XML string) for the given artifact.
///
fn download_pom(group: &str, name: &str, version: &str) -> Result<String, String> {
    // 1. Construct the URL
    let url: String = format!("http://central.maven.org/maven2/{}/{}/{}/{}-{}.pom",
                      group.replace(".", "/"), name, version, name, version);
    // 2. Download the URL
    return reqwest::get(url.as_str())
        .map_err( |e| e.to_string() )
        .and_then( |mut resp|
            if resp.status() == 200 {
                return resp.text().map_err( |e| e.to_string() );
            } else {
                return Err(format!("Could not download file (code={})", resp.status()).to_owned());
            }
        );
}



fn main() {
    let xml: String = download_pom("com.google.protobuf", "protobuf-java", "3.6.1")
        .expect("Could not download POM");
    println!("XML = {}", xml);
}
