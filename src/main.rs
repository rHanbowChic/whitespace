use whitespace::{aquifer, incubation};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut a = aquifer::Aquifer::new("https://note.ms");
    assert_eq!(a.set_text("test_ns", "1", "Hello, Whitespace!").await?, "200 OK");
    assert_eq!(a.get_text("test_ns", "1").await?, "Hello, Whitespace!");
    dbg!(a.get_actual_page("test_ns", "1"));

    let mut i = incubation::Incubator::new();
    dbg!(i.get_mapping("note.ms", "test_ns", "1"));
    dbg!(i.get_encryption_key_hex("note.ms", "test_ns", "1"));
    Ok(())
}