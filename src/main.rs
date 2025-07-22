// The Rust Programming Language: A Crash Course and Building Our First Lexer
// CS152 Compiler Design using the Rust Programming Language.
// A Handwritten Compiler Using Rust.
// Creating a Lexer By Hand.

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;
// fn main() {

//     // Let us get commandline arguments and store them in a Vec<String>
//     let args: Vec<String> = env::args().collect();
//     if args.len() == 1 {
//         println!("Please provide an input file through the commandline arguments for the lexer.");
//         return;
//     }

//     if args.len() > 2 {
//         println!("Too many commandline arguments.");
//         return;
//     }

//     // read the entire file contents, storing them inside 'code' as a string.
//     let filename = &args[1];
//     let code = match fs::read_to_string(filename) {
//     Err(error) => {
//         println!("**Error. File \"{}\": {}", filename, error);
//         return;
//     }

//     Ok(code) => { 
//         code
//     } 

//     };

//     let tokens = match lex(&code) {
//     Err(error_message) => {
//         println!("**Error**");
//         println!("----------------------");
//         println!("{}", error_message);
//         println!("----------------------");
//         return;
//     }

//     Ok(data) => data,
    
//     };


//     // print out the lexer tokens parsed.

//     println!("----------------------");
//     println!("Finished Lexing the file {}", filename);
//     println!("File Contents:");
//     println!("{code}");
//     println!("Here are the Results:");
//     println!("----------------------");
//     for t in &tokens {
//       println!("{:?}", t);
//     }

// }
mod interpreter;
static mut loop_stack: Vec<String> = Vec::new();


fn main() {
  // get commandline arguments.
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
      println!("Please provide an input file.");
      return;
  }

  if args.len() > 2 {
      println!("Too many commandline arguments.");
      return;
  }

  // read the entire file.
  let filename = &args[1];
  let result = fs::read_to_string(filename);
  let code = match result {
  Err(error) => {
      println!("**Error. File \"{}\": {}", filename, error);
      return;
  }

  Ok(code) => {
    code
  } 

  };

  let tokens = match lex(&code) {
  Err(error_message) => {
      println!("**Error**");
      println!("----------------------");
      println!("{}", error_message);
      println!("----------------------");
      return;
  }

  Ok(tokens) => tokens,
  
  };

  let mut index: usize = 0;
  let mut symbol_table = SymbolTable {
    table: std::collections::HashMap::new(),
    has_main: false,
  };
  match parse_program(&tokens, &mut index, &mut symbol_table) {

   Ok(code) => {
        println!("Program Parsed Successfully.");
        println!("--------------------------------------------");
        println!("{code}");
        println!("--------------------------------------------");
        interpreter::execute_ir(&code);
    }

  Err(message) => {
      println!("**Error**");
      println!("----------------------");
      if tokens.len() == 0 {
          println!("No code has been provided.");
      } else {
          println!("Error: {message}");
          println!("----------------------");
      }
  }

  }
}


// Creating an Enum within Rust.
// Documentation: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// Enums are a way of saying a value is one of a possible set of values.
// Unlike C, Rust enums can have values associated with that particular enum value.
// for example, a Num has a 'i32' value associated with it, 
// but Plus, Subtract, Multiply, etc. have no values associated with it.
#[derive(Debug, Clone)]
enum Token {
  Plus, // '+'
  Subtract, // '-'
  Multiply, // 'x'
  Divide, // '/'
  Modulus, // '%'
  Assign, // '='
  LeftParen, // '('
  RightParen, // ')'
  LeftCurly, // '{'
  RightCurly, // '}'
  LeftBracket, // '['
  RightBracket, // ']'
  Semicolon, // ';'
  Comma, // ','
  Less, // '<'
  Greater, // '>'
  LessEqual, // '<='
  GreaterEqual, // '>='
  NotEqual, // '!='
  Equality, // '=='
  Num(i32), 
  Ident(String), 
  If, // 'if'
  Else, // 'else'
  While, // 'while'
  Read, // 'read'
  Func, // 'func'
  Return, // 'return'
  Print, // 'print'
  Continue, // 'continue'
  Break, // 'break'
  Int, // 'int'
  End, // 'end'
}

// In Rust, you can model the function behavior using the type system.
// https://doc.rust-lang.org/std/result/
// Result < Vec<Token>, String>
// means that this function can either return:
// - A list of tokens as a Vec<Token>
// - Or an error message represented as a string
// If there is an error, it will return an error
// If successful, it will return Vec<Token>
// A Result is an enum like this:
// enum Result {
//     Ok(the_result),
//     Err(the_error),
// }
struct Expression {
  code: String,
  name: String,
}


// This is a lexer that parses numbers and math operations
fn lex(code: &str) -> Result<Vec<Token>, String> {
  let bytes = code.as_bytes();
  let mut tokens: Vec<Token> = vec![];

  let mut i = 0;
  while i < bytes.len() {
    let c = bytes[i] as char;

    match c {

    '0'..='9' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let digit = bytes[i] as char;
        if digit >= '0' && digit <= '9' {
          i += 1;
        } else if (digit >= 'a' && digit <= 'z') | (digit >= 'A' && digit <= 'Z') | (digit == '_') {
          let s = format!("{}",&code[start..i+1]);
          return Err(format!("Invalid Identifier {:?}", s));
        } else {
          break;
        }
      }
      let end = i;
      let string_token = &code[start..end];
      let number_value = string_token.parse::<i32>().unwrap();
      let token = Token::Num(number_value);
      tokens.push(token);
    }

    '+' => {
      tokens.push(Token::Plus);
      i += 1;
    }

    '-' => {
      tokens.push(Token::Subtract);
      i += 1;
    }

    '*' => {
      tokens.push(Token::Multiply);
      i += 1;
    }

    '/' => {
      tokens.push(Token::Divide);
      i += 1;
    }

    '%' => {
      tokens.push(Token::Modulus);
      i += 1;
    }

    '(' => {
      tokens.push(Token::LeftParen);
      i += 1;
    }

    ')' => {
      tokens.push(Token::RightParen);
      i += 1;
    }

    '{' => {
      tokens.push(Token::LeftCurly);
      i += 1;
    }

    '}' => {
      tokens.push(Token::RightCurly);
      i += 1;
    }

    '[' => {
      tokens.push(Token::LeftBracket);
      i += 1;
    }

    ']' => {
      tokens.push(Token::RightBracket);
      i += 1;
    }

    ';' => {
      tokens.push(Token::Semicolon);
      i += 1;
    }

    ',' => {
      tokens.push(Token::Comma);
      i += 1;
    }

    '<' => {
      if i + 1 < bytes.len() {
        if bytes[i+1] as char == '=' {
          tokens.push(Token::LessEqual);
          i += 2;
        }
        else{
          tokens.push(Token::Less);
          i += 1;
        }
      }
      else{
        tokens.push(Token::Less);
        i += 1;
      }
    }

    '>' => {
      if i + 1 < bytes.len() {
        if bytes[i+1] as char == '=' {
          tokens.push(Token::GreaterEqual);
          i += 2;
        }
        else{
          tokens.push(Token::Greater);
          i += 1;
        }
      }
      else{
        tokens.push(Token::Greater);
        i += 1;
      }
    }

    '=' => {
      if i + 1 < bytes.len(){
        if bytes[i+1] as char == '=' {
          tokens.push(Token::Equality);
          i += 2
        }
        else{
          tokens.push(Token::Assign);
          i += 1;
        }
      }
      else{
        tokens.push(Token::Assign);
        i += 1;
      }
    }

    '!' => {
      if i + 1 < bytes.len() {
        if bytes[i+1] as char == '=' {
          tokens.push(Token::NotEqual);
          i += 2
        }
        else{
          let s = format!("{}{}", c, bytes[i+1] as char);
          return Err(format!("Unrecognized Symbol '{}'", s));
        }
      }
      else{
          return Err(format!("Unrecognized Symbol '{}'", c));
      }
      
    }



    ' ' | '\n' => {
      i += 1;
    }

    'a'..='z' | 'A'..='Z' => {
      let start = i;
      i += 1;
      while i < bytes.len() {
        let digit = bytes[i] as char;
        if (digit >= 'a' && digit <= 'z') | (digit >= 'A' && digit <= 'Z') | (digit >= '0' && digit <= '9') | (digit == '_') {
          i += 1;
        } else {
          break;
        }
      }
      let end = i;
      let stringtoken = &code[start..end];
      let token = create_identifier(stringtoken);
      tokens.push(token);
    }

    '#' => {
      i+=1;
      while i < bytes.len() {
        let digit = bytes[i] as char;
        if digit != '\n' {
          i += 1;
        } else {
          i += 1;
          break;
        }
      }
    }
    

    _ => {
      return Err(format!("Unrecognized symbol '{}'", c));
    }

    }
  }

  tokens.push(Token::End);
  return Ok(tokens);
}

fn create_identifier(code: &str) -> Token {
  match code {
  "func" => Token::Func,
  "return" => Token::Return,
  "int" => Token::Int,
  "print" => Token::Print,
  "read" => Token::Read,
  "while" => Token::While,
  "if" => Token::If,
  "else" => Token::Else,
  "break" => Token::Break,
  "continue" => Token::Continue,
  _ => Token::Ident(String::from(code)),
  }
}


// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &Vec<Token>, index: &mut usize, symbol_table:&mut SymbolTable) -> Result<String, String> {
  assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));
  let mut code = String::new();
  while !at_end(tokens, *index) {
    match parse_function(tokens, index, symbol_table) {
    Ok(function_code) => {
      code += &function_code; 
    }
    Err(e) => { return Err(e); }
    }
  }
  if symbol_table.has_main == false {
    return Err(String::from("Main function is not defined"));
  }
  return Ok(code);
}

fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
match tokens[index] {
Token::End => { true }
_ => { false }
}
}

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}

static mut LABEL_NUM: i64 = 0;

fn create_label() -> String {
  unsafe {
    LABEL_NUM += 1;
    format!("_label{}", LABEL_NUM)
  }
}
// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is done to handle statements.

//untested
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Int,
    IntArray(i32), 
    Function {defined: bool}
}

pub struct SymbolTable {
  table: std::collections::HashMap<String, SymbolType>,
  has_main: bool 
}

struct Declaration {
  ident: String,
  code: String,
  symtype: SymbolType,
}

fn parse_function(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<String, String> {
  
  //%func main()
  //%endfunc
  let mut local_symbol_table =  SymbolTable {
    table: std::collections::HashMap::new(),
    has_main: false,
  };

  match tokens[*index] {
  Token::Func => { *index += 1; }
  _ => { return Err(String::from("functions must begin with func")); }
  }

  let mut function_code: String;
  function_code = String::from("");

  match &tokens[*index] { 
  Token::Ident(ident) => { 
    if symbol_table.table.contains_key(ident) {
      return Err(format!("Function {ident} is already defined"));
    }
    if ident == "main" {
      symbol_table.has_main = true;
    }
    symbol_table.table.insert(ident.clone(), SymbolType::Function { defined: false });
    function_code += &format!("%func {ident}(");
    *index += 1; }
  _  => { return Err(String::from("functions must have a function identifier"));}
  }


  match tokens[*index] {
  Token::LeftParen => { *index += 1; }
  _ => { return Err(String::from("expected '('"));}
  }

  while !matches!(tokens[*index], Token::RightParen) {

    match parse_declaration_statement(tokens, index, symbol_table) {
    Ok(declaration_code) => {
      function_code += &declaration_code.code;
    }
    Err(e) => {return Err(e);}
    }

    if matches!(tokens[*index], Token::RightParen){
      break;
    }

    match tokens[*index]{
      Token::Comma => {
        *index += 1;
        match tokens[*index] {
          Token::RightParen => {
            return Err(String::from("expected expression after comma"))
          }
          _ => {
            *index += 0;
            function_code += &format!(", ");
          }
        }
      }
      _ => {
        return Err(String::from("expected ')' or ','"));
      }
    }
  }

  match tokens[*index] {
  Token::RightParen => { 
    *index += 1; 
    function_code += &format!(")\n");
  }
  _ => { return Err(String::from("expected ')'"));}
  }

  match tokens[*index] {
  Token::LeftCurly => { *index += 1; }
  _ => { return Err(String::from("expected '{'"));}
  }

  while !matches!(tokens[*index], Token::RightCurly) {
        match parse_statement(tokens, index, symbol_table, &mut local_symbol_table) {
        Ok(statement_code) => {
            function_code += &statement_code;
        }
        Err(e) => {return Err(e);}
        }
    }


  match tokens[*index] {
  Token::RightCurly => { *index += 1; }
  _ => { return Err(String::from("expected '}'"));}
  }

  function_code += "%endfunc\n";
  for (key, _typee) in &local_symbol_table.table {
      symbol_table.table.remove(key);
    }
  return Ok(function_code);
}

// parsing a statement such as:
// int a;
// a = a + b;
// a = a % b;
// print(a)
// read(a)
// returns epsilon if '}'

//untested
fn parse_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable, local_symbol_table: &mut SymbolTable) -> Result<String, String> {
  let statement: String;
  match tokens[*index] {
    Token::Int => {      
      match parse_declaration_statement(tokens, index, symbol_table) {
        Ok(declaration_code) => {
          statement = declaration_code.code + &format!("\n");
          local_symbol_table.table.insert(declaration_code.ident.clone(), declaration_code.symtype.clone());
          symbol_table.table.insert(declaration_code.ident.clone(), declaration_code.symtype.clone());
          //BOOKMARK2
        } 
        Err(e) => return Err(e),
      }

      match tokens[*index]{
        Token::Semicolon => {
          *index += 1;
          return Ok(statement);
        }
        _ => {return Err(String::from("Statements must end with a semicolon"));}
      }

    }
    Token::Ident(_) => parse_assignment_statement(tokens, index, symbol_table),
    Token::Break => {
      *index += 1;
      match tokens[*index] {
        Token::Semicolon => {
          *index += 1;
          let top = unsafe { loop_stack.last().cloned() };
          let label = top.unwrap_or_else(|| String::from("UNKNOWN"));
          if label == "UNKNOWN"{
            return Err(String::from("Break used outside of loop"));
          }
          statement = format!("%jmp :end{}\n", label); 
          return Ok(statement);
        }
        _ => {
          println!("Current token: {:?}", tokens[*index]);
          Err(String::from("Statements must end with a semicolon"))
        }
      }
    }
    Token::Continue => {
      *index += 1;
      match tokens[*index] {
        Token::Semicolon => {
          *index += 1;
          let top = unsafe { loop_stack.last().cloned() };
          let label = top.unwrap_or_else(|| String::from("UNKNOWN"));
          if label == "UNKNOWN"{
            return Err(String::from("Continue used outside of loop"));
          }
          statement = format!("%jmp :{}\n", label); 
          return Ok(statement);
        }
        _ => {
          println!("Current token: {:?}", tokens[*index]);
          Err(String::from("Statements must end with a semicolon"))
        }
      }
    }
    Token::While => parse_while_statement(tokens, index, symbol_table),
    Token::If => parse_if_statement(tokens, index, symbol_table),
    Token::Return => parse_return_statement(tokens, index, symbol_table),
    Token::Print => parse_print_statement(tokens, index, symbol_table),
    Token::Read => parse_read_statement(tokens, index, symbol_table),
    _ => {
      println!("Current token: {:?}", tokens[*index]);
      Err(String::from("invalid statement"))
    }
  }
}


fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<Declaration, String> {
  let mut statement = Declaration{
    ident: String::from(""),
    symtype: SymbolType::Int,
    code: String::from(""),
  };

  statement.code = String::from("");
  match tokens[*index] {
  Token::Int => {*index += 1;}
  _ => {
    println!("Current token: {:?}", tokens[*index]);
    return Err(String::from("Declaration statements must being with 'int' keyword"));}
  }

  match &tokens[*index] {
  Token::LeftBracket => {
    let mut arrnum;
    *index += 1;
    match tokens[*index] {
      Token::Num(array_size) => {
        if array_size <= 0 {
          return Err(String::from("Array size must be greater than 0"));
        }
        *index += 1;
        arrnum = array_size;
      }
      _ => {return Err(String::from("Brackets must contain a number"));}
    }

    match tokens[*index] {
      Token::RightBracket => {*index += 1;}
      _ => {return Err(String::from("must have a closing bracket"));}
    }

    match &tokens[*index] {
      Token::Ident(ident) => {
        if symbol_table.table.contains_key(ident) {
          return Err(format!("Variable {ident} is already defined"));
        }
        statement.ident = ident.clone();
        statement.symtype = SymbolType::IntArray(arrnum);
        //symbol_table.table.insert(ident.clone(), SymbolType::IntArray(arrnum));
        
        statement.code = format!("%int[] {ident}, {arrnum}\n");
        *index += 1;
      }
      _ => {return Err(String::from("Declarations must have an identifier"));}
    }
  }
  Token::Ident(ident) => {

    if symbol_table.table.contains_key(ident) {
      return Err(format!("Variable {ident} is already defined"));
    }
    statement.ident = ident.clone();
    statement.symtype = SymbolType::Int;
    //symbol_table.table.insert(ident.clone(), SymbolType::Int);
    *index += 1;
    statement.code = format!("%int {ident}")
  }
  _ => {return Err(String::from("Declarations must have an identifier"));}
  }

  // match tokens[*index] {
  // Token::Semicolon => {*index += 1;}
  // _ => {
  //   return Err(String::from("Statements must end with a semicolon"));}
  // }

  return Ok(statement);
}

//untested
fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<String, String> {
  let mut statement: String;
  let mut dest: String;
  let dest_type: String;
  
  match &tokens[*index] {
  Token::Ident(identifier) => {
    dest = identifier.clone();
    match symbol_table.table.get(identifier){
      Some(SymbolType::Int) => {
        dest_type = String::from("int");
      }
      Some(SymbolType::IntArray(_)) => {
        dest_type = String::from("array");
      }
      Some(SymbolType::Function {defined: _}) => {
        return Err(format!("Cannot assign to function {identifier}"));
      }
      None => {
        return Err(format!("Variable {identifier} is not defined"));
      }
    }
    *index += 1;
  }
  _ => {return Err(String::from("Assignment statements must being with an identifier"));}
  }

  match tokens[*index] {
    Token::LeftBracket => {
      if dest_type != "array" {
        return Err(format!("Type mismatch: using a scalar integer variable as an array of integers"));
      }
      *index += 1;
      match tokens[*index] {
        Token::Num(number) => {
          *index += 1;
          dest = format!("[{dest} + {number}]")
        }
        _ => {return Err(String::from("Brackets must contain a number"));}
      }
  
      match tokens[*index] {
        Token::RightBracket => {*index += 1;}
        _ => {return Err(String::from("must have a closing bracket"));}
      }
  
      match tokens[*index] {
        Token::Assign => {*index += 1;}
        _ => {return Err(String::from("Statement is missing the '=' operator"));}
      }
    }
  Token::Assign => {
    if dest_type != "int" {
      return Err(format!("Type mismatch: using an array of integers as a scalar integer"));
    }
    *index += 1;
  }
  _ => {return Err(String::from("Statement is missing the '=' operator"));}
  }

  match parse_expression(tokens, index, symbol_table) {
  Ok(expression) => {
    let src = expression.name;
    statement = expression.code;
    statement += &format!("%mov {dest}, {src}\n");
  },
  Err(e) => {return Err(e);}
  }

  match tokens[*index] {
  Token::Semicolon => {*index += 1;}
  _ => {return Err(String::from("Missing semicolon"));}
  }

  return Ok(statement);
}

fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<String, String> {
  let mut statement: String;
  statement = String::from("");
  match tokens[*index] {
  Token::Return => {*index += 1;}
  _ => {return Err(String::from("Return statements must being with a return keyword"));}
  }

  match parse_expression(tokens, index, symbol_table) {
  Ok(expression) => {
    let dest = expression.name;
    statement = expression.code;
    statement += &format!("%ret {dest}\n");
  },
  Err(e) => {return Err(e);}
  }

  match tokens[*index] {
  Token::Semicolon => {*index += 1;}
  _ => {return Err(String::from("Statement is missing the '=' operator"));}
  }
  return Ok(statement);
  //todo!()
}

fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<String, String> {
  let expression: Expression;
  match tokens[*index] {
  Token::Print=> {*index += 1;}
  _ => {return Err(String::from("Return statements must being with a return keyword"));}
  }

  match parse_expression(tokens, index, symbol_table) {
  Ok(expr) => { expression = expr; },
  Err(e) => {return Err(e);}
  }

  match tokens[*index] {
  Token::Semicolon => {*index += 1;}
  _ => {return Err(String::from("Statement is missing the '=' operator"));}
  }

  let mut statement = expression.code;
  let name = expression.name;
  statement += &format!("%out {name}\n");
  return Ok(statement);
}

fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<String, String> {
  let mut statement: String;
  match tokens[*index] {
  Token::Read => {*index += 1;}
  _ => {return Err(String::from("Return statements must being with a return keyword"));}
  }

  match parse_expression(tokens, index, symbol_table) {
    Ok(expr) => {},
    Err(e) => {return Err(e);}
  }
  match tokens[*index] {
  Token::Semicolon => {*index += 1;}
  _ => {return Err(String::from("Statement is missing the '=' operator"));}
  }

  //todo!();
  statement = String::from("");
  return Ok(statement) //temporary empty string
}

// parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
fn parse_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<Expression, String> {
  let mut expression: Expression;
  match parse_multiply_expression(tokens, index, symbol_table) {
  Ok(expr) => {
    expression = expr;
  },
  Err(e) => {return Err(e);}
  }
  loop {
     match tokens[*index] {

     Token::Plus => {
         *index += 1;
         match parse_multiply_expression(tokens, index, symbol_table) {
         Ok(expr2) => {
               let src1 = expression.name;
               let src2 = expr2.name;
               let dest = create_temp();
               expression.code += &expr2.code;
               expression.code += &format!("%int {dest}\n");
               expression.code += &format!("%add {dest}, {src1}, {src2}\n");
               expression.name = dest;
           },
         Err(e) => {return Err(e);}
         }
     }

     Token::Subtract => {
         *index += 1;
         match parse_multiply_expression(tokens, index, symbol_table) {
         Ok(expr2) => {
               let src1 = expression.name;
               let src2 = expr2.name;
               let dest = create_temp();
               expression.code += &expr2.code;
               expression.code += &format!("%int {dest}\n");
               expression.code += &format!("%add {dest}, {src1}, {src2}\n");
               expression.name = dest;
           },
         Err(e) => {return Err(e);}
         }
     }

     _ => { 
         break;
     }

     };
  }

  return Ok(expression);
}

fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<Expression, String> {
  let mut expression: Expression;
  match parse_term(tokens, index, symbol_table) {
  Ok(expr) => {
    expression = expr;
  },
  Err(e) => {return Err(e);}
  }
  loop {
     match tokens[*index] {
     Token::Multiply => {
        *index += 1;
        match parse_term(tokens, index, symbol_table) {
        Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%mult {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
        Err(e) => {return Err(e);}
        }
     }

     Token::Divide => {
        *index += 1;
        match parse_term(tokens, index, symbol_table) {
        Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%div {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
        Err(e) => {return Err(e);}
        }
     }

     Token::Modulus => {
        *index += 1;
        match parse_term(tokens, index, symbol_table) {
        Ok(expr2) => {
              let src1 = expression.name;
              let src2 = expr2.name;
              let dest = create_temp();
              expression.code += &expr2.code;
              expression.code += &format!("%int {dest}\n");
              expression.code += &format!("%mod {dest}, {src1}, {src2}\n");
              expression.name = dest;
          },
        Err(e) => {return Err(e);}
        }
     }

     _ => {
         break;
     }

     };

  }

  return Ok(expression);
}

// a term is either a Number or an Identifier.

//incomplete 
fn parse_term(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<Expression, String> {
  let mut expression = Expression {
    code : String::from(""),
    name : String::from("")
  };
  match &tokens[*index] {

    Token::Num(number) => {
      *index += 1;
      expression.name = number.to_string();
      return Ok(expression);
    }

    Token::LeftParen => {
      *index += 1;
      match parse_expression(tokens, index, symbol_table) {
      Ok(e) => {expression = e;},
      Err(e) => {return Err(e);}
      }

      match tokens[*index]{
        Token::RightParen => {
          *index += 1;
        }
        _ => {
          return Err(String::from("Expected ')"));
        }
      }

    }


    Token::Ident(ident) => {
      *index += 1;
      let mut funcargs = String::from("");
      if matches!(tokens[*index], Token::LeftParen) {
        *index += 1;
        
        while !matches!(tokens[*index], Token::RightParen){
          match parse_expression(tokens, index, symbol_table){
            Ok(expr) => {
              //BOOKMARK1
              funcargs += &expr.name;
              expression.code += &expr.code;
            },
            Err(e) => {return Err(e);}
          }

          match tokens[*index]{
            Token::Comma => {
              *index += 1;
              match tokens[*index] {
                Token::RightParen => {
                  return Err(String::from("expected expression after comma"))
                }
                _ => {
                  funcargs += &format!(", ");
                  *index += 0;
                }
              }
            }
            Token::RightParen => {
              break;
            }
            _ => {
              return Err(String::from("Expected ',' or ')'"));
            }
          }
        }
        match tokens[*index]{
          Token::RightParen => {
            // match tokens[*index - 1]{
            //   Token::Comma => {
            //     return Err(String::from("expected expression after comma"));
            //   }
            //   _ => {
            //     *index += 0;
            //   }
            // }
            *index += 1;
            expression.name = create_temp();
            if !symbol_table.table.contains_key(ident){
              return Err(format!("Function {ident} is not defined"));
            }
            expression.code += &format!("%int {0}\n%call {0}, {ident}({funcargs})\n", expression.name);
          }
          _ => {
            return Err(String::from("Expected ')"));
          }
        }

      }

      else if matches!(tokens[*index], Token::LeftBracket){
        *index += 1;

        match parse_expression(tokens, index, symbol_table){
          Ok(expr) => {
            expression.name = create_temp();
            expression.code = format!("%int {0}\n", expression.name);
            expression.code += &format!("%mov {0}, [{ident} + {1}] \n", expression.name, expr.name)
          },
          Err(e) => {return Err(e);}
        }

        match tokens[*index]{
          Token::RightBracket => {
            *index += 1;
          }
          _ => {return Err(String::from("Expected ']'"));}
        }
      }


      else{ 
        expression.name = ident.to_string();
        return Ok(expression) 
      }
    }
    _ => {
      println!("Current token: {:?}", tokens[*index]);
      return Err(String::from("Invalid Expression"));
    }
    
  }
  return Ok(expression);
}

//missing while statement, if statement, boolean expression, 

fn parse_while_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<String, String> {
  let mut local_symbol_table =  SymbolTable {
    table: std::collections::HashMap::new(),
    has_main: false,
  };
  let mut statement: String;
  statement = String::from("");
  match tokens[*index] {
    Token::While => {
      *index += 1;
    }
    _ => {return Err(String::from("parse while statement incomplete"));}
    
  }

  let boolean_expression = parse_boolean_expression(tokens, index, symbol_table)?;
  let start_label = create_label();
  unsafe{ loop_stack.push(start_label.clone()) };
  match tokens[*index] {
    Token::LeftCurly => {
      *index += 1;
    }
    _ => {return Err(String::from("parse while statement incomplete"));}
  }
  

  let mut while_loop_body = String::from(""); 
  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index, symbol_table, &mut local_symbol_table) {
      Ok(statement) => {while_loop_body += &statement;},
      Err(e) => {return Err(e);}
    }
  }

  match tokens[*index] {
    Token::RightCurly => {
      *index += 1;
    }
    _ => {return Err(String::from("parse while statement incomplete"));}
  }
  let mut loop_code = String::from("");
  loop_code += &format!(":{}\n", start_label);
  loop_code += &boolean_expression.code;
  loop_code += &format!("%branch_ifn {}, :end{}\n", boolean_expression.name, start_label);
  loop_code += &while_loop_body;
  loop_code += &format!("%jmp :{}\n", start_label);
  loop_code += &format!(":end{}\n", start_label);
  unsafe { loop_stack.pop() };
  for (key, _typee) in &local_symbol_table.table {
      symbol_table.table.remove(key);
    }
  Ok(loop_code)
}

fn parse_if_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<String, String> {
  let mut local_symbol_table =  SymbolTable {
    table: std::collections::HashMap::new(),
    has_main: false,
  };
  let mut statement: String;
  statement = String::from("");
  match tokens[*index] {
    Token::If => {
      *index += 1;
    }
    _ => {return Err(String::from("parse if statement incomplete"));}
  }

  let boolean_expression = parse_boolean_expression(tokens, index, symbol_table)?;


  match tokens[*index] {
    Token::LeftCurly => {
      *index += 1; 
    }
    _ => {return Err(String::from("expected '{'"));}
  }
  let mut if_body = String::from("");
  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index, symbol_table, &mut local_symbol_table) {
      Ok(statement) => { if_body += &statement },
      Err(e) => {return Err(e);}
    }
  }

  match tokens[*index] { 
    Token::RightCurly => {
      *index += 1;
    }
    _ => {return Err(String::from("expected '}'"));}
  }
  let mut else_body = String::from("");
  let mut if_code = String::from("");
  if matches!(tokens[*index], Token::Else) {
    *index += 1;
    match tokens[*index] {
      Token::LeftCurly => {
        *index += 1;
      }
      _ => {return Err(String::from("expected '{'"));}
    }
    while !matches!(tokens[*index], Token::RightCurly) {
      match parse_statement(tokens, index, symbol_table, &mut local_symbol_table) {
        Ok(statement) => { else_body += &statement; },
        Err(e) => {return Err(e);}
      }
    }

    match tokens[*index] {
      Token::RightCurly => {
        *index += 1;
        statement = String::from("");
      }
      _ => {return Err(String::from("expected '}'"));}

    
    }
    let label = create_label();
    let end_label = create_label();
    if_code += &boolean_expression.code;
    if_code += &format!("%branch_ifn {}, :{}\n", boolean_expression.name, label);
    if_code += &if_body;
    if_code += &format!("%jmp :{}\n", end_label);
    if_code += &format!(":{}\n", label);
    if_code += &else_body;
    if_code += &format!(":{}\n", end_label);
    for (key, _typee) in &local_symbol_table.table {
      symbol_table.table.remove(key);
    }
    Ok(if_code)
  }
  else {
    let label = create_label();
    if_code += &boolean_expression.code;
    if_code += &format!("%branch_ifn {}, :{}\n", boolean_expression.name, label);
    if_code += &if_body;
    if_code += &format!(":{}\n", label);
    for (key, _typee) in &local_symbol_table.table {
      symbol_table.table.remove(key);
    }
    Ok(if_code)
  }

}

fn parse_boolean_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut SymbolTable) -> Result<Expression, String> {
  let mut expression: Expression;
  let expr1;
  let opcode: &str;
  match parse_expression(tokens, index, symbol_table) {
    Ok(expr) => { expr1 = expr;},
    _ => {return Err(String::from("Invalid expression"));}
  }
  match tokens[*index] {
    Token::Less => {
      opcode = "%lt";
      *index += 1;
    }
    Token::Greater => {
      opcode = "%gt";
      *index += 1;
    }
    Token::LessEqual => {
      opcode = "%le";
      *index += 1;
    }
    Token::GreaterEqual => {
      opcode = "%ge";
      *index += 1;
    }
    Token::NotEqual => {
      opcode = "%ne";
      *index += 1;
    }
    Token::Equality => {
      opcode = "%eq";
      *index += 1;
    }
    _ => {return Err(String::from("Invalid boolean expression"));}
  }

  let expr2;
  match parse_expression(tokens, index, symbol_table) {
    Ok(expr) => { expr2 = expr; },
    _ => {return Err(String::from("Invalid boolean expression"));}
  }


  let dest = create_temp();
  let src1 = expr1.name;
  let src2 = expr2.name;  
  let mut statement = String::from("");
  statement += &expr1.code;
  statement += &expr2.code;
  statement += &format!("%int {dest}\n");
  statement += &format!("{opcode} {dest}, {src1}, {src2}\n");

  expression = Expression {
    code: statement,
    name: dest,
  };

  Ok(expression)
}

// writing tests!
// testing shows robustness in software, and is good for spotting regressions
// to run a test, type "cargo test" in the terminal.
// Rust will then run all the functions annotated with the "#[test]" keyword.
#[cfg(test)]
mod tests {
    use crate::Token;
    use crate::lex;
    use crate::parse_statement;

    #[test]
    fn lexer_test() {
        // test that lexer works on correct cases
        let toks = lex("1 + 2 + 3").unwrap();
        assert!(toks.len() == 6);
        assert!(matches!(toks[0], Token::Num(1)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(2)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::Num(3)));
        assert!(matches!(toks[5], Token::End));

        let toks = lex("3 + 215 +").unwrap();
        assert!(toks.len() == 5);
        assert!(matches!(toks[0], Token::Num(3)));
        assert!(matches!(toks[1], Token::Plus));
        assert!(matches!(toks[2], Token::Num(215)));
        assert!(matches!(toks[3], Token::Plus));
        assert!(matches!(toks[4], Token::End));

        // test that the lexer catches invalid tokens
        assert!(matches!(lex("^^^"), Err(_)));
    }
    fn test_statements() {

      // test that valid statements are correct.
      let tokens = lex("a = 1 + 2;").unwrap();
      parse_statement(&tokens, &mut 0).unwrap();

      let tokens = lex("b = 1 / 2;").unwrap();
      parse_statement(&tokens, &mut 0).unwrap();


      // test errors. missing semicolon
      let tokens = lex("b = 1 / 2").unwrap();
      assert!(matches!(parse_statement(&tokens, &mut 0), Err(_)));

  }


}