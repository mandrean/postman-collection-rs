extern crate postman_collection;

use postman_collection::PostmanCollection;
use std::io::Write;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        match postman_collection::from_path(path) {
            Ok(collection) => {
                match collection {
                    PostmanCollection::V1_0_0(spec) => {
                        println!("Found v1.0.0 collection with the name: {}", spec.name);
                    }
                    PostmanCollection::V2_0_0(spec) => {
                        println!("Found v2.0.0 collection with the name: {}", spec.info.name);
                    }
                    PostmanCollection::V2_1_0(spec) => {
                        println!("Found v2.1.0 collection with the name: {}", spec.info.name);
                    }
                }
                //println!("{}", postman_collection::to_json(&spec).unwrap());
            }
            Err(e) => {
                let stderr = &mut ::std::io::stderr();
                let errmsg = "Error writing to stderr";

                writeln!(stderr, "error: {}", e).expect(errmsg);

                for e in e.iter().skip(1) {
                    writeln!(stderr, "caused by: {}", e).expect(errmsg);
                }

                // The backtrace is not always generated. Try to run this example
                // with `RUST_BACKTRACE=1`.
                if let Some(backtrace) = e.backtrace() {
                    writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
                }

                ::std::process::exit(1);
            }
        }
    }
}
