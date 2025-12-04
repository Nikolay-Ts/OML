use crate::core::oml_object::OmlObject;

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let obj = OmlObject::get_from_file("src/core/test/oml_files/hello.oml")?;
    println!("{:?}", obj);

    Ok(())
}