use std::cmp::PartialEq;
use std::fs;
use std::path::Path;
use regex::Regex;

use crate::core::errors;

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    ENUM,
    CLASS,
    STRUCT,
    UNDECIDED
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableModifier {
    CONST,
    MUT,
    STATIC,
    OPTIONAL,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableVisibility {
    PRIVATE,
    PUBLIC,
    PROTECTED
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub var_mod: Vec<VariableModifier>,
    pub visibility: VariableVisibility,
    pub var_type: String,
    pub name: String,
}

/// for testing purposes only
impl Variable {
    fn new(var_mod: Vec<VariableModifier>,visibility: VariableVisibility, var_type: String, name: String) -> Self {
        Variable {
            var_mod,
            visibility,
            var_type,
            name
        }
    }
}

#[derive(Debug)]
pub struct OmlObject {
    pub oml_type: ObjectType,
    pub name: String,
    pub variables: Vec<Variable>
}

impl OmlObject {
    const CLASS_NAME: &'static str = "class";
    const ENUM_NAME: &'static str = "enum";
    const STRUCT_NAME: &'static str = "struct";

    fn new() -> Self {
        Self {
            oml_type: ObjectType::UNDECIDED,
            name: String::from(""),
            variables: vec![]
        }
    }

    pub fn get_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(file_path);
        let content = fs::read_to_string(path)?;

        let mut oml_object = Self {
            oml_type: ObjectType::UNDECIDED,
            name: String::from("Nothing"),
            variables: vec![]
        };

        oml_object.scan_file(content)?;

        Ok(oml_object)
    }

    fn scan_file(&mut self, content: String) -> Result<(), Box<dyn std::error::Error>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut inside_body = false;
        let mut commenting = false;
        let mut body_lines: Vec<String> = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            let mut processed_line = String::new();
            let mut line_ref: &str = trimmed;

            if commenting {
                if let Some(pos) = line_ref.find("*/") {
                    commenting = false;
                    line_ref = line_ref[pos + 2..].trim_start();
                    if line_ref.is_empty() {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if let Some(pos) = line_ref.find("//") {
                line_ref = line_ref[..pos].trim_end();
                if line_ref.is_empty() {
                    continue;
                }
            }

            if let Some(pos) = line_ref.find("/*") {
                let before_comment = line_ref[..pos].trim_end();

                if let Some(end_pos) = line_ref[pos..].find("*/") {
                    let after_comment = line_ref[pos + end_pos + 2..].trim_start();
                    processed_line = format!("{} {}", before_comment, after_comment);
                    line_ref = processed_line.trim();
                } else {
                    commenting = true;
                    line_ref = before_comment;
                }

                if line_ref.is_empty() {
                    continue;
                }
            }

            if !inside_body {
                let tokens: Vec<&str> = line_ref.split_whitespace().collect();
                if tokens.is_empty() {
                    continue;
                }

                match tokens[0] {
                    Self::CLASS_NAME => {
                        self.oml_type = ObjectType::CLASS;
                        if tokens.len() > 1 {
                            self.assign_obj_name(tokens[1])?;
                        }
                    }
                    Self::ENUM_NAME => {
                        self.oml_type = ObjectType::ENUM;
                        if tokens.len() > 1 {
                            self.assign_obj_name(tokens[1])?;
                        }
                    }
                    Self::STRUCT_NAME => {
                        self.oml_type = ObjectType::STRUCT;
                        if tokens.len() > 1 {
                            self.assign_obj_name(tokens[1])?;
                        }
                    }
                    _ => {}
                }

                if line_ref.contains('{') {
                    inside_body = true;
                }
                continue;
            }

            if line_ref.contains('}') {
                break;
            }

            if !line_ref.is_empty() {
                let tokens: Vec<&str> = line_ref.split_whitespace().collect();
                let has_type_and_name = tokens.iter().any(|&t| Self::is_type(t))
                    && tokens.len() >= 2;

                if has_type_and_name || line_ref.ends_with(';') {
                    body_lines.push(line_ref.to_string());
                }
            }
        }

        if !body_lines.is_empty() {
            self.variables = Self::extract_object_variables(body_lines)?;
        }

        Ok(())
    }

    fn assign_obj_name(&mut self, name: &str) -> Result<(), errors::NameError> {
        match Self::is_valid_name(name) {
            true => self.name = name.to_string(),
            false => {
                let message = format!("{} is not a valid obj name.", name);
                return Err(errors::NameError::new(message));
            }
        }

        Ok(())
    }

    #[inline]
    fn parse_visibility(token: &str) -> Option<VariableVisibility> {
        match token {
            "private" => Some(VariableVisibility::PRIVATE),
            "public" => Some(VariableVisibility::PUBLIC),
            "protected" => Some(VariableVisibility::PROTECTED),
            _ => None
        }
    }

    #[inline]
    fn parse_modifier(token: &str) -> Option<VariableModifier> {
        match token {
            "const" => Some(VariableModifier::CONST),
            "mut" => Some(VariableModifier::MUT),
            "static" => Some(VariableModifier::STATIC),
            "optional" => Some(VariableModifier::OPTIONAL),
            _ => None,
        }
    }

    #[inline]
    fn is_type(token: &str) -> bool {
        matches!(token,
            "int8" | "int16" | "int32" | "int64" |
            "uint8" | "uint16" | "uint32" | "uint64" |
            "float" | "double" | "bool" | "string" | "char"
        ) || Self::is_valid_name(token)
    }

    fn extract_object_variables(lines: Vec<String>) -> Result<Vec<Variable>, Box<dyn std::error::Error>> {
        let mut vars: Vec<Variable> = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let cleaned = trimmed.trim_end_matches(|c| c == ';' || c == '\n').trim();

            match Self::parse_variable_declaration(cleaned) {
                Ok(var) => vars.push(var),
                Err(e) => {
                    return Err(format!("Error parsing line '{}': {}", line, e).into());
                }
            }
        }

        Ok(vars)
    }

    fn parse_variable_declaration(line: &str) -> Result<Variable, String> {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            return Err("Empty line".to_string());
        }

        let mut visibility: Option<VariableVisibility> = None;
        let mut modifiers: Vec<VariableModifier> = Vec::new();
        let mut var_type: Option<String> = None;
        let mut var_name: Option<String> = None;
        let mut type_seen = false;

        for token in tokens {
            if let Some(vis) = Self::parse_visibility(token) {
                if type_seen {
                    return Err(format!(
                        "Visibility modifier '{}' cannot appear after type",
                        token
                    ));
                }

                if visibility.is_some() {
                    return Err("Multiple visibility modifiers found".to_string());
                }
                visibility = Some(vis);
                continue;
            }

            if let Some(modifier) = Self::parse_modifier(token) {
                if type_seen {
                    return Err(format!(
                        "Modifier '{}' cannot appear after type",
                        token
                    ));
                }
                modifiers.push(modifier);
                continue;
            }

            if Self::is_type(token) && var_type.is_none() {
                var_type = Some(token.to_string());
                type_seen = true;
                continue;
            }

            if var_type.is_some() && var_name.is_none() {
                var_name = Some(token.to_string());
                break;
            }

            return Err(format!("Unexpected token: {}", token));
        }

        let final_type = var_type.ok_or("No type specified")?;
        let final_name = var_name.ok_or("No variable name specified")?;
        let final_visibility = visibility.unwrap_or(VariableVisibility::PRIVATE);

        if modifiers.contains(&VariableModifier::CONST) && modifiers.contains(&VariableModifier::MUT) {
            return Err(format!("Const Error: variable {} cannot be const and mut simultaneously!", final_name));
        }

        Ok(Variable {
            var_mod: modifiers,
            visibility: final_visibility,
            var_type: final_type,
            name: final_name,
        })
    }

    #[inline]
    fn is_valid_name(name: &str) -> bool {
        let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_.-]*$").unwrap();
        re.is_match(name)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const VALID_NAMES: [&str; 8] = [
        "myfile.txt",
        "variable_name",
        "Config",
        "test123",
        "my-file-name",
        "file.tar.gz",
        "a",
        "MyClass"
    ];

    const INVALID_NAMES: [&str; 8] = [
        "123file",
        "_private",
        "-file",
        "my file",
        "file@name",
        "my$var",
        "file/path",
        "",
    ];

    #[test]
    fn test_name_validity() {
        for valid_name in VALID_NAMES {
            assert_eq!(OmlObject::is_valid_name(valid_name), true);
        }

        for valid_name in INVALID_NAMES {
            assert_eq!(OmlObject::is_valid_name(valid_name), false);
        }
    }

    #[test]
    fn test_assign_name() {
        let mut oml_obj = OmlObject::new();

        for valid_name in VALID_NAMES {
            oml_obj.assign_obj_name(valid_name).expect("this should not happen");
            assert_eq!(oml_obj.name, valid_name);
        }

        for invalid_name in INVALID_NAMES {
            let error = oml_obj.assign_obj_name(invalid_name).unwrap_err();
            let message = format!("{} is not a valid obj name.", invalid_name);
            assert_eq!(error.message, message);
        }
    }

    #[test]
    fn test_parse_variable_declaration() {
        // Valid declarations
        let valid_cases = vec![
            ("public const int64 myVar", "myVar", "int64", 1, VariableVisibility::PUBLIC),
            ("private mut string name", "name", "string", 1, VariableVisibility::PRIVATE),
            ("protected static int32 count", "count", "int32", 1, VariableVisibility::PROTECTED),
            ("int64 simpleVar", "simpleVar", "int64", 0, VariableVisibility::PRIVATE),
            ("const int64 meow", "meow", "int64", 1, VariableVisibility::PRIVATE),
            ("string hello", "hello", "string", 0, VariableVisibility::PRIVATE),
            ("bool isTrue", "isTrue", "bool", 0, VariableVisibility::PRIVATE),
        ];

        for (input, expected_name, expected_type, expected_mod_count, _expected_vis) in valid_cases {
            let result = OmlObject::parse_variable_declaration(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let var = result.unwrap();
            assert_eq!(var.name, expected_name);
            assert_eq!(var.var_type, expected_type);
            assert_eq!(var.var_mod.len(), expected_mod_count);
            assert!(matches!(var.visibility, _expected_vis));
        }
        let invalid_cases = vec![
            "int64 private myVar",
            "int32 public x",
            "string private name",
            "int64 const x",
        ];

        for input in invalid_cases {
            let result = OmlObject::parse_variable_declaration(input);
            assert!(result.is_err(), "Should have failed: {}", input);
        }
    }

    #[test]
    fn test_parse_class_from_string() {
        let content = r#"
            class Hello {
                const int64 meow;
                string hello;
                bool isTrue;
            }
        "#;

        let mut oml_obj = OmlObject::new();
        let result = oml_obj.scan_file(content.to_string());

        assert!(result.is_ok());
        assert_eq!(oml_obj.name, "Hello");
        assert!(matches!(oml_obj.oml_type, ObjectType::CLASS));
        assert_eq!(oml_obj.variables.len(), 3);

        // check first variable
        assert_eq!(oml_obj.variables[0].name, "meow");
        assert_eq!(oml_obj.variables[0].var_type, "int64");
        assert_eq!(oml_obj.variables[0].var_mod.len(), 1);
        assert!(matches!(oml_obj.variables[0].var_mod[0], VariableModifier::CONST));

        // check second variable
        assert_eq!(oml_obj.variables[1].name, "hello");
        assert_eq!(oml_obj.variables[1].var_type, "string");
        assert_eq!(oml_obj.variables[1].var_mod.len(), 0);

        // check third variable
        assert_eq!(oml_obj.variables[2].name, "isTrue");
        assert_eq!(oml_obj.variables[2].var_type, "bool");
    }

    #[test]
    fn test_modifier_ordering_rule() {
        let content = r#"
            class Test {
                int64 private x;
            }
        "#;

        let mut oml_obj = OmlObject::new();
        let result = oml_obj.scan_file(content.to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_const() {
        let content = r#"
            class Test {
                const mut int64 x;
            }
        "#;

        let content_swaped = r#"
            class Test {
                const mut int64 x;
            }
        "#;

        let mut oml_obj = OmlObject::new();
        let result = oml_obj.scan_file(content.to_string());

        assert!(result.is_err());

        let result = oml_obj.scan_file(content_swaped.to_string());

        assert!(result.is_err());
    }

    #[cfg(test)]
    mod comment_tests {
        use super::*;

        #[test]
        fn test_single_comments() {
            let content = r#"
               // ignore me
               class Test {
                const int64 x;
                const int64 y; // this is good
                // const int64 z;
               }
            "#;

            let content2 = r#"
                // ignore me
                //class Test {
                // const int64 x;
                //const int64 y; // this is good
                //  const int64 z;
               //}
            "#;


            let vars = vec![
                Variable::new(vec![VariableModifier::CONST], VariableVisibility::PRIVATE,  String::from("int64"), String::from("x")),
                Variable::new(vec![VariableModifier::CONST], VariableVisibility::PRIVATE,  String::from("int64"), String::from("y"))
            ];

            let mut oml_obj = OmlObject::new();
            let mut result = oml_obj.scan_file(content.to_string());

            assert!(result.is_ok());
            assert_eq!(oml_obj.variables, vars);

            let mut oml_obj2 = OmlObject::new();
            result = oml_obj2.scan_file(content2.to_string());

            assert!(result.is_ok());
            assert_eq!(oml_obj2.variables, vec![]);
        }

        #[test]
        fn test_multi_lined_comments() {
            let content = r#"
               /* ignore me

               */
               class Test {
                const int64 x;
                const int64 y; /* hello world */
                const /* this should cause no issue */
               }
            "#;

            let content2 = r#"
                // ignore me
                //class Test {
                // const int64 x;
                //const int64 y; // this is good
                //  const int64 z;
               //}
            "#;

            let vars = vec![
                Variable::new(vec![VariableModifier::CONST], VariableVisibility::PRIVATE,  String::from("int64"), String::from("x")),
                Variable::new(vec![VariableModifier::CONST], VariableVisibility::PRIVATE,  String::from("int64"), String::from("y"))
            ];

            let mut oml_obj = OmlObject::new();
            let mut result = oml_obj.scan_file(content.to_string());

            assert!(result.is_ok());
            assert_eq!(oml_obj.variables, vars);

            let mut oml_obj2 = OmlObject::new();
            result = oml_obj2.scan_file(content2.to_string());

            assert!(result.is_ok());
            assert_eq!(oml_obj2.variables, vec![]);

        }
    }
}
