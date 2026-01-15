use crate::core::oml_object::OmlObject;
use std::fmt::Write;
pub fn oml_to_cpp(oml_object: &OmlObject, file_name: &String) -> Result<String, std::fmt::Error> {
    let mut cpp_file = String::from("");
    let header_guard = format!("{}_H", file_name.to_uppercase());

    writeln!(cpp_file, "// This file has been generated from {}.oml", file_name)?;
    writeln!(cpp_file, "#ifndef {}", header_guard)?;
    writeln!(cpp_file, "#define {}", header_guard)?;


    writeln!(cpp_file, "#endif // {}", header_guard)?;
    
    Ok(cpp_file)
}
