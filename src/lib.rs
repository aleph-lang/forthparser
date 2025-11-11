use lalrpop_util::lalrpop_mod;
use aleph_syntax_tree::syntax::AlephTree as at;

lalrpop_mod!(pub grammar);

/// Forth parser
///
/// # Arguments
/// * `source` - String containing Forth code to parse
///
/// # Returns
/// Returns an AlephTree representing the parsed Forth program
///
pub fn parse(source: String) -> at {
    let ast = grammar::ProgramParser::new().parse(&source);
    match ast {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            at::Unit
        }
    }
}

/// Parse a single Forth definition (word, constant, variable, etc.)
pub fn parse_definition(source: String) -> at {
    let ast = grammar::DefinitionParser::new().parse(&source);
    match ast {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            at::Unit
        }
    }
}

/// Parse a sequence of Forth words (for REPL or interactive mode)
pub fn parse_words(source: String) -> Vec<Box<at>> {
    let ast = grammar::WordBodyParser::new().parse(&source);
    match ast {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_constant() {
        let code = "42 CONSTANT answer";
        let ast = parse_definition(code.to_string());
        match ast {
            at::VarDecl { name, is_constant: true, .. } => assert_eq!(name, "answer"),
            _ => panic!("Expected constant VarDecl"),
        }
    }

    #[test]
    fn test_parse_variable() {
        let code = "VARIABLE counter";
        let ast = parse_definition(code.to_string());
        match ast {
            at::VarDecl { name, is_constant: false, .. } => assert_eq!(name, "counter"),
            _ => panic!("Expected variable VarDecl"),
        }
    }

    #[test]
    fn test_parse_word_definition() {
        let code = ": square dup * ;";
        let ast = parse_definition(code.to_string());
        match ast {
            at::ProcedureDef { name, body, .. } => {
                assert_eq!(name, "square");
                assert_eq!(body.len(), 2); // dup and *
            }
            _ => panic!("Expected ProcedureDef"),
        }
    }

    #[test]
    fn test_parse_if_then() {
        let code = ": abs dup 0< IF negate THEN ;";
        let ast = parse_definition(code.to_string());
        match ast {
            at::ProcedureDef { name, .. } => assert_eq!(name, "abs"),
            _ => panic!("Expected ProcedureDef"),
        }
    }

    #[test]
    fn test_parse_begin_until() {
        let code = ": countdown BEGIN dup . 1- dup 0= UNTIL drop ;";
        let ast = parse_definition(code.to_string());
        match ast {
            at::ProcedureDef { name, .. } => assert_eq!(name, "countdown"),
            _ => panic!("Expected ProcedureDef"),
        }
    }

    #[test]
    fn test_parse_do_loop() {
        let code = ": sum 0 swap 0 DO i + LOOP ;";
        let ast = parse_definition(code.to_string());
        match ast {
            at::ProcedureDef { name, .. } => assert_eq!(name, "sum"),
            _ => panic!("Expected ProcedureDef"),
        }
    }

    #[test]
    fn test_parse_comment() {
        let code = "( This is a comment ) : test dup ;";
        let ast = parse(code.to_string());
        match ast {
            at::ForthProgram { .. } => {},
            _ => panic!("Expected ForthProgram"),
        }
    }

    #[test]
    fn test_parse_hex_number() {
        let code = "0xFF CONSTANT max_byte";
        let ast = parse_definition(code.to_string());
        match ast {
            at::VarDecl { name, is_constant: true, .. } => assert_eq!(name, "max_byte"),
            _ => panic!("Expected constant VarDecl"),
        }
    }

    #[test]
    fn test_parse_create() {
        let code = "CREATE buffer 100 ALLOT ;";
        let ast = parse_definition(code.to_string());
        match ast {
            at::ForthCreate { name, .. } => assert_eq!(name, "buffer"),
            _ => panic!("Expected ForthCreate"),
        }
    }
}

