use crate::core::oml_object::{OmlObject, ObjectType};
use std::fmt::Write;
pub fn oml_to_cpp(oml_object: &OmlObject, file_name: &String) -> Result<String, std::fmt::Error> {
    let mut cpp_file = String::from("");
    let header_guard = format!("{}_H", file_name.to_uppercase());

    writeln!(cpp_file, "// This file has been generated from {}.oml", file_name)?;
    writeln!(cpp_file, "#ifndef {}", header_guard)?;
    writeln!(cpp_file, "#define {}", header_guard)?;
    writeln!(cpp_file, "#\ninclude <cstdint>")?;
    writeln!(cpp_file, "#\ninclude <string>")?;

    match &oml_object.oml_type {
        ObjectType::ENUM => generate_enum(oml_object, &mut cpp_file)?,
        ObjectType::CLASS => generate_enum(oml_object, &mut cpp_file)?,
        ObjectType::STRUCT => generate_enum(oml_object, &mut cpp_file)?,
        ObjectType::UNDECIDED => return Err(std::fmt::Error),
    }


    writeln!(cpp_file, "#endif // {}", header_guard)?;
    
    Ok(cpp_file)
}

#[inline]
fn convert_type(var_type: &str) -> String {
    match var_type {
        "int8" => "int8_t",
        "int16" => "int16_t",
        "int32" => "int32_t",
        "int64" => "int64_t",
        "uint8" => "uint8_t",
        "uint16" => "uint16_t",
        "uint32" => "uint32_t",
        "uint64" => "uint64_t",
        "float" => "float",
        "double" => "double",
        "bool" => "bool",
        "string" => "std::string",
        "char" => "char",
        _ => ""
    }.to_string()
}

fn generate_enum(oml_object: &OmlObject, cpp_file: &mut String) -> Result<(), std::fmt::Error>{
    writeln!(cpp_file, "enum class {} {{", oml_object.name)?;
    let length = oml_object.variables.len();

    for (index, var) in oml_object.variables.iter().enumerate() {
        write!(cpp_file, "\t{}", var.name.to_uppercase())?;
        if index == length-1 {
            continue
        }
        writeln!(cpp_file, ",")?;

    }

    writeln!(cpp_file, "}}")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::oml_object::{OmlObject, ObjectType, Variable, VariableVisibility};

    #[test]
    fn test_generate_enum_basic() {
        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "Color".to_string(),
            variables: vec![
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "Red".to_string(),
                },
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "Green".to_string(),
                },
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "Blue".to_string(),
                },
            ],
        };

        let mut output = String::new();
        generate_enum(&oml_object, &mut output).unwrap();

        assert!(output.contains("enum class Color {"));
        assert!(output.contains("\tRED,"));
        assert!(output.contains("\tGREEN,"));
        assert!(output.contains("\tBLUE"));
        assert!(!output.contains("BLUE,"));
        assert!(output.contains("}"));
    }

    #[test]
    fn test_generate_enum_single_variant() {
        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "Status".to_string(),
            variables: vec![
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "Active".to_string(),
                },
            ],
        };

        let mut output = String::new();
        generate_enum(&oml_object, &mut output).unwrap();

        assert!(output.contains("enum class Status {"));
        assert!(output.contains("\tACTIVE"));
        assert!(!output.contains("ACTIVE,"));
        assert!(output.contains("}"));
    }

    #[test]
    fn test_generate_enum_empty() {
        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "Empty".to_string(),
            variables: vec![],
        };

        let mut output = String::new();
        generate_enum(&oml_object, &mut output).unwrap();

        assert!(output.contains("enum class Empty {"));
        assert!(output.contains("}"));
        // Should not contain any variant names or commas
        assert_eq!(output.matches('\t').count(), 0);
    }

    #[test]
    fn test_generate_enum_uppercase_conversion() {
        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "WeaponType".to_string(),
            variables: vec![
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "sword".to_string(),
                },
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "bow".to_string(),
                },
            ],
        };

        let mut output = String::new();
        generate_enum(&oml_object, &mut output).unwrap();

        assert!(output.contains("\tSWORD,"));
        assert!(output.contains("\tBOW"));
        assert!(!output.contains("sword"));
        assert!(!output.contains("bow"));
    }

    #[test]
    fn test_generate_enum_mixed_case_names() {
        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "Direction".to_string(),
            variables: vec![
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "NorthEast".to_string(),
                },
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "SouthWest".to_string(),
                },
            ],
        };

        let mut output = String::new();
        generate_enum(&oml_object, &mut output).unwrap();

        assert!(output.contains("\tNORTHEAST,"));
        assert!(output.contains("\tSOUTHWEST"));
    }

    #[test]
    fn test_generate_enum_many_variants() {
        let mut variables = vec![];
        for i in 0..10 {
            variables.push(Variable {
                var_mod: vec![],
                visibility: VariableVisibility::PUBLIC,
                var_type: "".to_string(),
                name: format!("Variant{}", i),
            });
        }

        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "LargeEnum".to_string(),
            variables,
        };

        let mut output = String::new();
        generate_enum(&oml_object, &mut output).unwrap();

        assert!(output.contains("enum class LargeEnum {"));
        assert!(output.contains("\tVARIANT0,"));
        assert!(output.contains("\tVARIANT5,"));
        assert!(output.contains("\tVARIANT9"));
        assert!(!output.contains("VARIANT9,"));  // Last should not have comma

        // Count commas - should be 9 (one less than number of variants)
        assert_eq!(output.matches(',').count(), 9);
    }

    #[test]
    fn test_generate_enum_special_characters_in_name() {
        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "Test_Enum-123".to_string(),
            variables: vec![
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "Value".to_string(),
                },
            ],
        };

        let mut output = String::new();
        generate_enum(&oml_object, &mut output).unwrap();

        assert!(output.contains("enum class Test_Enum-123 {"));
    }

    #[test]
    fn test_oml_to_cpp_with_enum() {
        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "Status".to_string(),
            variables: vec![
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "Active".to_string(),
                },
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "Inactive".to_string(),
                },
            ],
        };

        let result = oml_to_cpp(&oml_object, &"Status".to_string()).unwrap();

        // Check header guard
        assert!(result.contains("#ifndef STATUS_H"));
        assert!(result.contains("#define STATUS_H"));
        assert!(result.contains("#endif // STATUS_H"));

        // Check comment
        assert!(result.contains("// This file has been generated from Status.oml"));

        // Check enum content
        assert!(result.contains("enum class Status {"));
        assert!(result.contains("\tACTIVE,"));
        assert!(result.contains("\tINACTIVE"));
    }

    #[test]
    fn test_convert_type_all_types() {
        assert_eq!(convert_type("int8"), "int8_t");
        assert_eq!(convert_type("int16"), "int16_t");
        assert_eq!(convert_type("int32"), "int32_t");
        assert_eq!(convert_type("int64"), "int64_t");
        assert_eq!(convert_type("uint8"), "uint8_t");
        assert_eq!(convert_type("uint16"), "uint16_t");
        assert_eq!(convert_type("uint32"), "uint32_t");
        assert_eq!(convert_type("uint64"), "uint64_t");
        assert_eq!(convert_type("float"), "float");
        assert_eq!(convert_type("double"), "double");
        assert_eq!(convert_type("bool"), "bool");
        assert_eq!(convert_type("string"), "std::string");
        assert_eq!(convert_type("char"), "char");
    }

    #[test]
    fn test_convert_type_unknown() {
        assert_eq!(convert_type("CustomType"), "");
        assert_eq!(convert_type("unknown"), "");
        assert_eq!(convert_type(""), "");
    }

    #[test]
    fn test_enum_formatting_has_proper_newlines() {
        let oml_object = OmlObject {
            oml_type: ObjectType::ENUM,
            name: "Test".to_string(),
            variables: vec![
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "A".to_string(),
                },
                Variable {
                    var_mod: vec![],
                    visibility: VariableVisibility::PUBLIC,
                    var_type: "".to_string(),
                    name: "B".to_string(),
                },
            ],
        };

        let mut output = String::new();
        generate_enum(&oml_object, &mut output).unwrap();

        let newline_count = output.matches('\n').count();
        assert!(newline_count >= 3);
    }
}