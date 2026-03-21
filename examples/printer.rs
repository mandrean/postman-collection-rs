fn main() -> postman_collection::Result<()> {
    if let Some(path) = std::env::args().nth(1) {
        let collection = postman_collection::from_path(path)?;
        println!(
            "Found {:?} collection with the name: {}",
            collection.version(),
            collection.name()
        );
    }

    Ok(())
}
