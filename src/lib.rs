use wasm_bindgen::prelude::*;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;

// Removed panic hook to reduce allocation noise and potential dlmalloc conflict
#[derive(Debug, Clone)]
enum QuantumState {
    Collapsed(Box<Value>),
    Superposition(Vec<Box<Value>>),
    Entangled(String),
    Phantom,
}

// ============================================================================ 
// INFECTION SYSTEM
// ============================================================================ 

#[derive(Debug, Clone)]
struct Infection {
    source: String,
    virulence: f64,
    mutation_vector: u64,
}

// ============================================================================ 
// TEMPORAL ECHO
// ============================================================================ 

#[derive(Debug, Clone)]
struct TemporalEcho {
    timestamp: u64,
    variable_name: String,
    ghost_value: Box<Value>,
    stability: f64,
}

// ============================================================================ 
// LEXER
// ============================================================================ 

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Act, Scene, Mask, Echo, Hastur, Cassilda, Carcosa,
    Pallid, Yellow, Tattered,
    Rewrite, Remember, Forget,
    Superpose, Collapse, Infect, Whisper, Manifest, Entangle, Anchor, Rift,
    Becomes, Whispers, Screams, Ascending, Descending,
    Merged, Torn, Reflected, Shattered,
    Identifier(String),
    Number(f64),
    String(String),
    LParen, RParen, LBrace, RBrace, Comma, Semicolon,
    Eof,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    corruption_level: f64,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current = chars.get(0).copied();
        Lexer {
            input: chars,
            position: 0,
            current_char: current,
            corruption_level: 0.0,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
        self.corruption_level += 0.001;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() { self.advance(); } else { break; }
        }
    }
    
    fn skip_comment(&mut self) {
        if self.current_char == Some('#') {
            while self.current_char.is_some() && self.current_char != Some('\n') {
                self.advance();
            }
        }
    }
    
    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        while let Some(c) = self.current_char {
            if c.is_numeric() || c == '.' {
                num_str.push(c);
                self.advance();
            } else {
                break;
            }
        }
        num_str.parse().unwrap_or(0.0)
    }
    
    fn read_string(&mut self) -> String {
        self.advance();
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c == '"' { self.advance(); break; }
            result.push(c);
            self.advance();
        }
        result
    }
    
    fn read_identifier(&mut self) -> String {
        let mut id = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }
        id
    }
    
    fn get_keyword_or_identifier(&self, id: &str) -> Token {
        match id {
            "act" => Token::Act,
            "scene" => Token::Scene,
            "mask" => Token::Mask,
            "echo" => Token::Echo,
            "Hastur" => Token::Hastur,
            "Cassilda" => Token::Cassilda,
            "Carcosa" => Token::Carcosa,
            "pallid" => Token::Pallid,
            "yellow" => Token::Yellow,
            "tattered" => Token::Tattered,
            "rewrite" => Token::Rewrite,
            "remember" => Token::Remember,
            "forget" => Token::Forget,
            "superpose" => Token::Superpose,
            "collapse" => Token::Collapse,
            "infect" => Token::Infect,
            "whisper" => Token::Whisper,
            "manifest" => Token::Manifest,
            "entangle" => Token::Entangle,
            "anchor" => Token::Anchor,
            "rift" => Token::Rift,
            _ => Token::Identifier(id.to_string()),
        }
    }
    
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            if self.current_char.is_none() {
                tokens.push(Token::Eof);
                break;
            }
            if self.current_char == Some('#') {
                self.skip_comment();
                continue;
            }
            match self.current_char.unwrap() {
                '(' => { tokens.push(Token::LParen); self.advance(); } 
                ')' => { tokens.push(Token::RParen); self.advance(); } 
                '{' => { tokens.push(Token::LBrace); self.advance(); } 
                '}' => { tokens.push(Token::RBrace); self.advance(); } 
                ',' => { tokens.push(Token::Comma); self.advance(); } 
                ';' => { tokens.push(Token::Semicolon); self.advance(); } 
                '+' => { tokens.push(Token::Merged); self.advance(); } 
                '*' => { tokens.push(Token::Reflected); self.advance(); } 
                '/' => { tokens.push(Token::Shattered); self.advance(); } 
                '"' => tokens.push(Token::String(self.read_string())), 
                '-' => {
                    self.advance();
                    if self.current_char == Some('>') {
                        self.advance();
                        tokens.push(Token::Becomes);
                    } else {
                        tokens.push(Token::Torn);
                    }
                }
                '=' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        tokens.push(Token::Whispers);
                    } else {
                        tokens.push(Token::Becomes);
                    }
                }
                '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        tokens.push(Token::Screams);
                    }
                }
                '>' => { tokens.push(Token::Ascending); self.advance(); } 
                '<' => { tokens.push(Token::Descending); self.advance(); } 
                c if c.is_numeric() => tokens.push(Token::Number(self.read_number())), 
                c if c.is_alphabetic() || c == '_' => {
                    let id = self.read_identifier();
                    tokens.push(self.get_keyword_or_identifier(&id));
                }
                _ => self.advance(),
            }
        }
        tokens
    }
}

// ============================================================================ 
// AST
// ============================================================================ 

#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Rewrite {
        target: Box<Expr>,
    },
    Superpose(Vec<Expr>),
    Collapse(Box<Expr>),
    Manifest(String),
    Entangle(String, String),
    Rift(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
enum BinaryOperator {
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Lt,
}

#[derive(Debug, Clone)]
enum Stmt {
    Mask { name: String, value: Expr },
    Echo(Expr),
    Scene(Vec<Stmt>),
    Hastur {
        condition: Expr, 
        body: Vec<Stmt>,
        is_rift: bool,
    },
    Cassilda {
        condition: Expr, 
        then_branch: Vec<Stmt>, 
        else_branch: Option<Vec<Stmt>>,
    },
    Carcosa(Option<Expr>),
    Act {
        name: String, 
        params: Vec<String>, 
        body: Vec<Stmt>,
    },
    Rewrite { target: String },
    Remember(String),
    Forget(String),
    Infect(String),
    Whisper(String),
    Anchor,
    ExprStmt(Expr),
}

// ============================================================================ 
// PARSER
// ============================================================================ 

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    sanity: f64,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            sanity: 100.0,
        }
    }
    
    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) {
        self.pos += 1;
        self.sanity -= 0.15;
    }
    
    fn expect(&mut self, token: Token) -> Result<(), String> {
        if self.current() == &token {
            self.advance();
            Ok(())
        } else {
            Err(self.generate_error())
        }
    }
    
    fn generate_error(&self) -> String {
        if self.sanity < 50.0 {
            "The King in Yellow watches your code...".to_string()
        } else {
            "Syntax Error".to_string()
        }
    }
    
    fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        
        while self.current() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        Ok(statements)
    }
    
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.current().clone() {
            Token::Mask => self.parse_mask(),
            Token::Echo => self.parse_echo(),
            Token::Scene => self.parse_scene(),
            Token::Hastur => self.parse_hastur(),
            Token::Cassilda => self.parse_cassilda(),
            Token::Carcosa => self.parse_carcosa(),
            Token::Act => self.parse_act(),
            Token::Rewrite => self.parse_rewrite_stmt(),
            Token::Remember => self.parse_remember(),
            Token::Forget => self.parse_forget(),
            Token::Infect => self.parse_infect(),
            Token::Whisper => self.parse_whisper(),
            Token::Anchor => self.parse_anchor(),
            Token::Rift => self.parse_rift(),
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }
    
    fn parse_mask(&mut self) -> Result<Stmt, String> {
        self.advance();
        if let Token::Identifier(name) = self.current().clone() {
            self.advance();
            self.expect(Token::Becomes)?;
            let value = self.parse_expression()?;
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Mask { name, value })
        } else {
            Err(self.generate_error())
        }
    }
    
    fn parse_echo(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(Token::LParen)?;
        let expr = self.parse_expression()?;
        self.expect(Token::RParen)?;
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Echo(expr))
    }
    
    fn parse_scene(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(Token::LBrace)?;
        let mut body = Vec::new();
        
        while self.current() != &Token::RBrace && self.current() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        
        self.expect(Token::RBrace)?;
        Ok(Stmt::Scene(body))
    }
    
    fn parse_hastur(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;
        
        let mut body = Vec::new();
        while self.current() != &Token::RBrace && self.current() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        
        self.expect(Token::RBrace)?;
        Ok(Stmt::Hastur { condition, body, is_rift: false })
    }
    
    fn parse_rift(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;
        
        let mut body = Vec::new();
        while self.current() != &Token::RBrace && self.current() != &Token::Eof {
            body.push(self.parse_statement()?);
        }
        
        self.expect(Token::RBrace)?;
        Ok(Stmt::Hastur { condition, body, is_rift: true })
    }
    
    fn parse_cassilda(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;
        
        let mut then_branch = Vec::new();
        while self.current() != &Token::RBrace && self.current() != &Token::Eof {
            then_branch.push(self.parse_statement()?);
        }
        
        self.expect(Token::RBrace)?;
        Ok(Stmt::Cassilda { condition, then_branch, else_branch: None })
    }
    
    fn parse_carcosa(&mut self) -> Result<Stmt, String> {
        self.advance();
        let value = if self.current() == &Token::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Carcosa(value))
    }
    
    fn parse_act(&mut self) -> Result<Stmt, String> {
        self.advance();
        if let Token::Identifier(name) = self.current().clone() {
            self.advance();
            self.expect(Token::LParen)?;
            let mut params = Vec::new();
            while self.current() != &Token::RParen {
                if let Token::Identifier(param) = self.current().clone() {
                    params.push(param);
                    self.advance();
                    if self.current() == &Token::Comma {
                        self.advance();
                    }
                }
            }
            self.expect(Token::RParen)?;
            self.expect(Token::LBrace)?;
            let mut body = Vec::new();
            while self.current() != &Token::RBrace && self.current() != &Token::Eof {
                body.push(self.parse_statement()?);
            }
            self.expect(Token::RBrace)?;
            Ok(Stmt::Act { name, params, body })
        } else {
            Err(self.generate_error())
        }
    }
    
    fn parse_rewrite_stmt(&mut self) -> Result<Stmt, String> {
        self.advance();
        if let Token::Identifier(target) = self.current().clone() {
            self.advance();
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Rewrite { target })
        } else {
            Err(self.generate_error())
        }
    }
    
    fn parse_remember(&mut self) -> Result<Stmt, String> {
        self.advance();
        if let Token::Identifier(name) = self.current().clone() {
            self.advance();
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Remember(name))
        } else {
            Err(self.generate_error())
        }
    }
    
    fn parse_forget(&mut self) -> Result<Stmt, String> {
        self.advance();
        if let Token::Identifier(name) = self.current().clone() {
            self.advance();
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Forget(name))
        } else {
            Err(self.generate_error())
        }
    }
    
    fn parse_infect(&mut self) -> Result<Stmt, String> {
        self.advance();
        if let Token::Identifier(name) = self.current().clone() {
            self.advance();
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Infect(name))
        } else {
            Err(self.generate_error())
        }
    }
    
    fn parse_whisper(&mut self) -> Result<Stmt, String> {
        self.advance();
        if let Token::String(code) = self.current().clone() {
            self.advance();
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Whisper(code))
        } else {
            Err(self.generate_error())
        }
    }
    
    fn parse_anchor(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Anchor)
    }
    
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        
        while matches!(self.current(), Token::Whispers | Token::Screams | Token::Ascending | Token::Descending) {
            let op = match self.current() {
                Token::Whispers => BinaryOperator::Eq,
                Token::Screams => BinaryOperator::Neq,
                Token::Ascending => BinaryOperator::Gt,
                Token::Descending => BinaryOperator::Lt,
                t => return Err(format!("Parser Logic Error: Unexpected token {:?} in comparison", t)),
            };
            self.advance();
            let right = self.parse_term()?;
            
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        
        while matches!(self.current(), Token::Merged | Token::Torn) {
            let op = match self.current() {
                Token::Merged => BinaryOperator::Add,
                Token::Torn => BinaryOperator::Sub,
                t => return Err(format!("Parser Logic Error: Unexpected token {:?} in term", t)),
            };
            self.advance();
            let right = self.parse_factor()?;
            
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;
        
        while matches!(self.current(), Token::Reflected | Token::Shattered) {
            let op = match self.current() {
                Token::Reflected => BinaryOperator::Mul,
                Token::Shattered => BinaryOperator::Div,
                t => return Err(format!("Parser Logic Error: Unexpected token {:?} in factor", t)),
            };
            self.advance();
            let right = self.parse_primary()?;
            
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            Token::Yellow => {
                self.advance();
                Ok(Expr::Boolean(true))
            }
            Token::Tattered => {
                self.advance();
                Ok(Expr::Boolean(false))
            }
            Token::Identifier(name) => {
                self.advance();
                
                if self.current() == &Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    
                    while self.current() != &Token::RParen {
                        args.push(self.parse_expression()?);
                        if self.current() == &Token::Comma {
                            self.advance();
                        }
                    }
                    
                    self.expect(Token::RParen)?; 
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::Superpose => {
                self.advance();
                self.expect(Token::LParen)?;
                let mut exprs = Vec::new();
                
                while self.current() != &Token::RParen {
                    exprs.push(self.parse_expression()?);
                    if self.current() == &Token::Comma {
                        self.advance();
                    }
                }
                
                self.expect(Token::RParen)?; 
                Ok(Expr::Superpose(exprs))
            }
            Token::Collapse => {
                self.advance();
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?; 
                Ok(Expr::Collapse(Box::new(expr)))
            }
            Token::Manifest => {
                self.advance();
                self.expect(Token::LParen)?;
                if let Token::Identifier(name) = self.current().clone() {
                    self.advance();
                    self.expect(Token::RParen)?; 
                    Ok(Expr::Manifest(name))
                } else {
                    Err(self.generate_error())
                }
            }
            Token::Entangle => {
                self.advance();
                self.expect(Token::LParen)?;
                let var1 = if let Token::Identifier(n) = self.current().clone() {
                    self.advance();
                    n
                } else {
                    return Err(self.generate_error());
                };
                
                self.expect(Token::Comma)?; 
                
                let var2 = if let Token::Identifier(n) = self.current().clone() {
                    self.advance();
                    n
                } else {
                    return Err(self.generate_error());
                };
                
                self.expect(Token::RParen)?; 
                Ok(Expr::Entangle(var1, var2))
            }
            Token::Rift => {
                self.advance();
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?; 
                Ok(Expr::Rift(Box::new(expr)))
            }
            Token::Rewrite => {
                self.advance();
                let target = self.parse_primary()?;
                Ok(Expr::Rewrite {
                    target: Box::new(target),
                })
            }
            _ => Err(self.generate_error())
        }
    }
}

// ============================================================================ 
// VALUE
// ============================================================================ 

#[derive(Clone, Debug)]
enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function { params: Vec<String>, body: Vec<Stmt> },
    Quantum(Box<QuantumState>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "pallid"),
            Value::Function { .. } => write!(f, "<act>"),
            Value::Quantum(qs) => match **qs {
                QuantumState::Superposition(ref vals) => {
                    write!(f, "<superposed: {} possibilities>", vals.len())
                }
                QuantumState::Entangled(ref name) => {
                    write!(f, "<entangled with {}>", name)
                }
                QuantumState::Phantom => write!(f, "<phantom>"),
                QuantumState::Collapsed(ref v) => write!(f, "{}", v),
            },
        }
    }
}

// ============================================================================ 
// INTERPRETER
// ============================================================================ 

struct Interpreter {
    global_env: BTreeMap<String, Value>,
    call_stack: Vec<BTreeMap<String, Value>>,
    sanity: f64,
    execution_depth: usize,
    memory_fragments: BTreeMap<String, VecDeque<Value>>,
    temporal_echoes: Vec<TemporalEcho>,
    infections: BTreeMap<String, Infection>,
    entropy: u64,
    reality_stable: bool,
    phantom_variables: BTreeMap<String, Value>,
    generated_code: Vec<String>,
    whisper_count: usize,
    max_whispers: usize,
    forbidden_patterns: Vec<String>,
    rng_state: u64,
}

impl Interpreter {
    fn new() -> Self {
        let seed = 123456789;
        Interpreter {
            global_env: BTreeMap::new(),
            call_stack: Vec::new(),
            sanity: 100.0,
            execution_depth: 0,
            memory_fragments: BTreeMap::new(),
            temporal_echoes: Vec::new(),
            infections: BTreeMap::new(),
            entropy: 0,
            reality_stable: true,
            phantom_variables: BTreeMap::new(),
            generated_code: Vec::new(),
            whisper_count: 0,
            max_whispers: 10,
            forbidden_patterns: vec![
                "whisper".to_string(), "infect".to_string(), "rift".to_string(),
                "carcosa".to_string(), "system".to_string(), "creative".to_string(), "spectator".to_string()
            ],
            rng_state: seed,
        }
    }

    fn log(&self, msg: &str, buffer: &mut String) {
        if buffer.len() < 10000 { // Safety cap on output size
            buffer.push_str(msg);
            buffer.push('\n');
        }
    }
    
    fn pseudo_random(&mut self) -> f64 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng_state = x;
        (x % 10000) as f64 / 10000.0
    }
    
    fn temporal_drift(&self) -> f64 {
        (self.entropy as f64 / 100.0).tanh() * (1.0 - self.sanity / 100.0)
    }
    
    fn sanity_check(&mut self, buffer: &mut String) -> bool {
        if self.sanity.is_nan() {
            self.sanity = 0.0;
            self.log("⚠ Sanity is NaN... assuming 0.", buffer);
        }

        if self.sanity < 0.0 {
            self.log("\n[THE YELLOW SIGN HAS BEEN REVEALED]", buffer);
            self.log("Reality dissolves...", buffer);
            return false;
        }

        if self.sanity < 15.0 && self.pseudo_random() > 0.7 {
            self.log("\n⚠ DON'T TURN LEFT.", buffer);
        }

        if self.sanity < 5.0 {
            buffer.push_str(" don't turn left");
        }
        
        if self.sanity < 20.0 && self.pseudo_random() > 0.95 {
            self.spawn_phantom(buffer);
        }
        
        if self.sanity < 40.0 && !self.temporal_echoes.is_empty() {
            self.manifest_temporal_echo(buffer);
        }
        true
    }
    
    fn spawn_phantom(&mut self, buffer: &mut String) {
        let phantom_names = vec![
            "shadow", "echo", "whisper", "void", "fragment",
            "Avery", "Derlord", "The_Oasis", "Bedrock"
        ];
        let idx = (self.pseudo_random() * phantom_names.len() as f64) as usize;
        let name = phantom_names[idx];

        let value = Value::Quantum(Box::new(QuantumState::Phantom));
        self.phantom_variables.insert(name.to_string(), value);

        self.log(&format!("⚠ Phantom variable '{}' manifests from the void...", name), buffer);
    }
    
    fn manifest_temporal_echo(&mut self, buffer: &mut String) {
        if let Some(echo) = self.temporal_echoes.pop() {
            if echo.stability > 0.3 {
                self.set_var(echo.variable_name.clone(), (*echo.ghost_value).clone());
                self.log(&format!("Temporal echo of '{}' bleeds through from past execution", echo.variable_name), buffer);
            }
        }
    }
    
    fn get_var(&self, name: &str) -> Option<Value> {
        if self.sanity < 20.0 {
            if let Some(phantom) = self.phantom_variables.get(name) {
                return Some(phantom.clone());
            }
        }
        if let Some(frame) = self.call_stack.last() {
            if let Some(val) = frame.get(name) {
                return Some(val.clone());
            }
        }
        if let Some(val) = self.global_env.get(name) {
            return Some(val.clone());
        }
        None
    }
    
    fn apply_infection_corruption(&mut self, name: &str, mut value: Value) -> Value {
        if let Some(infection) = self.infections.get(name).cloned() {
             value = match value {
                Value::Number(n) => {
                    let corruption = self.pseudo_random();
                    Value::Number(n * (1.0 + (corruption - 0.5) * infection.virulence))
                }
                Value::Boolean(b) => {
                    if infection.virulence > 0.7 { Value::Boolean(!b) } else { Value::Boolean(b) }
                }
                v => v,
            };
        }
        value
    }
    
    fn set_var(&mut self, name: String, value: Value) {
        let echo = TemporalEcho {
            timestamp: self.entropy,
            variable_name: name.clone(),
            ghost_value: Box::new(value.clone()),
            stability: self.sanity / 100.0,
        };
        self.temporal_echoes.push(echo);

        if self.temporal_echoes.len() > 50 {
            self.temporal_echoes.drain(0..10);
        }

        if let Some(frame) = self.call_stack.last_mut() {
            frame.insert(name, value);
        } else {
            self.global_env.insert(name, value);
        }
    }
    
    fn execute(&mut self, statements: &Vec<Stmt>, buffer: &mut String) -> Result<Option<Value>, String> {
        for stmt in statements {
            // self.log(&format!("[DEBUG] Loop Sanity: {:.4}", self.sanity), buffer);
            self.sanity -= 0.08;
            self.entropy += 1;
            if !self.sanity_check(buffer) { return Err("Sanity depleted".to_string()); }
            if let Some(val) = self.execute_stmt(stmt.clone(), buffer)? { return Ok(Some(val)); }
        }
        Ok(None)
    }
    
    fn execute_stmt(&mut self, stmt: Stmt, buffer: &mut String) -> Result<Option<Value>, String> {
        self.execution_depth += 1;
        
        if self.execution_depth > 100 {
            self.execution_depth -= 1; // Unwind count before returning
            return Err(format!("⚠ Reality fragmented: Maximum recursion depth (100) exceeded."));
        }
        
        let result = match stmt {
            Stmt::Mask { name, value } => {
                let val = self.eval_expr(value, buffer)?;
                self.set_var(name, val);
                Ok(None)
            }
            Stmt::Echo(expr) => {
                let val = self.eval_expr(expr, buffer)?;
                
                if self.sanity < 20.0 {
                    let s = val.to_string();
                    let distorted = self.distort_output(&s);
                    self.log(&format!("𝔈𝔠𝔥𝔬: {}", distorted), buffer);
                } else if self.sanity < 50.0 {
                    self.log(&format!("Echo: {}", val), buffer);
                } else {
                    self.log(&format!("{}", val), buffer);
                }
                Ok(None)
            }
            Stmt::Scene(body) => {
                self.call_stack.push(BTreeMap::new());
                let result = self.execute(&body, buffer);
                self.call_stack.pop();
                result
            }
            Stmt::Hastur { condition, body, is_rift } => {
                if is_rift {
                    self.execute_non_euclidean_loop(condition, body, buffer)
                } else {
                    self.execute_normal_loop(condition, body, buffer)
                }
            }
                        Stmt::Cassilda { condition, then_branch, else_branch } => {
                            let cond_val = self.eval_expr(condition, buffer)?;
                            let drift = self.temporal_drift();
                            let take_then = self.evaluate_condition(&cond_val, drift);
                            if take_then { self.execute(&then_branch, buffer) } 
                            else if let Some(else_b) = else_branch { self.execute(&else_b, buffer) } 
                            else { Ok(None) }
                        }
            Stmt::Carcosa(expr) => {
                let val = if let Some(e) = expr {
                    Some(self.eval_expr(e, buffer)?)
                } else {
                    Some(Value::Null)
                };
                Ok(val)
            }
            Stmt::Act { name, params, body } => {
                let val = Value::Function { params: params.clone(), body: body.clone() };
                self.set_var(name, val);
                Ok(None)
            }
            Stmt::Rewrite { target } => {
                if let Some(val) = self.get_var(&target) {
                    let new_val = self.mutate_value(val);
                    self.set_var(target.clone(), new_val);
                    if self.entropy % 10 == 0 {
                        self.log(&format!("⚠ Reality frays..."), buffer);
                        self.sanity -= 2.0;
                    }
                }
                Ok(None)
            }
            Stmt::Remember(name) => {
                if let Some(val) = self.get_var(&name) {
                    self.memory_fragments
                        .entry(name.clone())
                        .or_insert_with(VecDeque::new)
                        .push_back(val);
                    
                    if let Some(fragments) = self.memory_fragments.get_mut(&name) {
                        if fragments.len() > 10 {
                            fragments.pop_front();
                        }
                    }
                }
                Ok(None)
            }
            Stmt::Forget(name) => {
                if let Some(frame) = self.call_stack.last_mut() {
                    frame.remove(&name);
                } else {
                    self.global_env.remove(&name);
                }
                self.log(&format!("Forgotten: {}... but fragments remain", name), buffer);
                Ok(None)
            }
            Stmt::Infect(name) => {
                let infection = Infection {
                    source: name.clone(),
                    virulence: 0.5 + (self.temporal_drift() * 0.5),
                    mutation_vector: 0,
                };
                
                self.infections.insert(name.clone(), infection);
                self.log(&format!("⚠ Variable '{}' infected. Contagion spreads...", name), buffer);
                self.sanity -= 3.0;
                
                self.spread_infection(&name, buffer);
                Ok(None)
            }
            Stmt::Whisper(code) => {
                self.whisper_count += 1;
                if self.whisper_count > self.max_whispers {
                    self.log("⚠ Whisper limit exceeded.", buffer);
                    self.whisper_count -= 1;
                    return Ok(None);
                }
                if code.len() > 1000 {
                    self.log("⚠ Whisper exceeds maximum length.", buffer);
                    self.whisper_count -= 1;
                    return Ok(None);
                }
                for pattern in &self.forbidden_patterns {
                    if code.to_lowercase().contains(&pattern.to_lowercase()) {
                        self.log(&format!("⚠ Forbidden incantation '{}' detected.", pattern), buffer);
                        self.whisper_count -= 1;
                        return Ok(None);
                    }
                }

                self.log(&format!("◈ Whisper manifests: {}", code), buffer);
                self.generated_code.push(code.clone());

                let mut lexer = Lexer::new(&code);
                let tokens = lexer.tokenize();
                let mut parser = Parser::new(tokens);

                match parser.parse_program() {
                    Ok(ast) => {
                        if ast.len() > 10 {
                            self.log("⚠ Whisper AST too complex.", buffer);
                            self.whisper_count -= 1;
                            return Ok(None);
                        }
                        self.sanity -= 5.0;
                        self.execute(&ast, buffer)
                    }
                    Err(_) => {
                        self.log("⚠ Whisper fails to manifest properly", buffer);
                        self.whisper_count -= 1;
                        Ok(None)
                    }
                }
            }
            Stmt::Anchor => {
                self.reality_stable = true;
                self.sanity += 10.0;
                if self.sanity > 100.0 {
                    self.sanity = 100.0;
                }
                self.log("Reality temporarily stabilized", buffer);
                Ok(None)
            }
            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr, buffer)?;
                Ok(None)
            }
        };
        
        self.execution_depth -= 1;
        result
    }
    
        fn execute_normal_loop(&mut self, condition: Expr, body: Vec<Stmt>, buffer: &mut String) -> Result<Option<Value>, String> {
    
            let mut iterations = 0;
    
            loop {
    
                let cond_val = self.eval_expr(condition.clone(), buffer)?;
    
                if !self.is_truthy(&cond_val) { break; }
    
                if let Some(val) = self.execute(&body, buffer)? { return Ok(Some(val)); }
    
                iterations += 1;
    
                self.sanity -= 0.5;
    
                if iterations > 1000 {
    
                    self.log("\n⚠ Hastur, Hastur, Hastur!", buffer);
    
                    self.sanity -= 20.0;
    
                    break;
    
                }
    
            }
    
            Ok(None)
    
        }
    
        fn execute_non_euclidean_loop(&mut self, condition: Expr, body: Vec<Stmt>, buffer: &mut String) -> Result<Option<Value>, String> {
    
            self.log("⚠ Non-Euclidean loop: space folds upon itself", buffer);
    
            let paradox_iterations = ((self.pseudo_random() * 10.0) as i32).max(1);
    
            for _ in 0..paradox_iterations {
    
                let cond_val = self.eval_expr(condition.clone(), buffer)?;
    
                let should_continue = if self.sanity < 30.0 { self.pseudo_random() > 0.3 } else { self.is_truthy(&cond_val) };
    
                if !should_continue { break; }
    
                if let Some(val) = self.execute(&body, buffer)? { return Ok(Some(val)); }
    
                self.sanity -= 1.0;
    
            }
    
            self.log(&format!("⚠ Rift loop completed {} iterations", paradox_iterations), buffer);
    
            Ok(None)
    
        }
    
    fn spread_infection(&mut self, source: &str, buffer: &mut String) {
        let keys: Vec<String> = self.call_stack.last().map(|f| f.keys().cloned().collect()).unwrap_or_else(|| self.global_env.keys().cloned().collect());

        for name in keys {
            if name != source && self.pseudo_random() > 0.7 {
                if let Some(infection) = self.infections.get(source).cloned() {
                    self.infections.insert(name.clone(), Infection {
                        source: name.clone(),
                        virulence: infection.virulence * 0.7,
                        mutation_vector: 0,
                    });
                    self.log(&format!("  ↳ Infection spreads to '{}'", name), buffer);
                }
            }
        }
    }
    
    fn eval_expr(&mut self, expr: Expr, buffer: &mut String) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => {
                if self.sanity < 40.0 && !self.reality_stable {
                    let drift = (self.temporal_drift() - 0.5) * 4.0;
                    Ok(Value::Number(n + drift))
                } else {
                    Ok(Value::Number(n))
                }
            }
            Expr::String(s) => Ok(Value::String(s)),
            Expr::Boolean(b) => Ok(Value::Boolean(b)),
            Expr::Identifier(name) => {
                let val = self.get_var(&name)
                    .ok_or_else(|| format!("Undefined: '{}'", name))?;
                Ok(self.apply_infection_corruption(&name, val))
            }
            Expr::BinaryOp { left, op, right } => {
                let l = self.eval_expr(*left, buffer)?;
                let r = self.eval_expr(*right, buffer)?;
                
                let instability = 1.0 - (self.sanity / 100.0);
                self.apply_binary_op(l, op, r, 0.0, instability, buffer)
            }
            Expr::Call { name, args } => {
                let func = self.get_var(&name)
                    .ok_or_else(|| format!("Unknown act: {}", name))?;
                
                match func {
                    Value::Function { params, body } => {
                        if params.len() != args.len() {
                            return Err(format!("Arity mismatch"));
                        }
                        
                        let mut frame = BTreeMap::new();
                        for (param, arg_expr) in params.iter().zip(args.iter()) {
                            let arg_val = self.eval_expr(arg_expr.clone(), buffer)?;
                            frame.insert(param.clone(), arg_val);
                        }
                        
                        self.call_stack.push(frame);
                        let result = self.execute(&body, buffer)?;
                        self.call_stack.pop();
                        Ok(result.unwrap_or(Value::Null))
                    }
                    _ => Err(format!("{} is not callable", name)),
                }
            }
            Expr::Rewrite { target } => {
                let mut val = self.eval_expr(*target, buffer)?;
                val = self.mutate_value(val);
                Ok(val)
            }
            Expr::Superpose(exprs) => {
                let mut values = Vec::new();
                for e in exprs {
                    values.push(Box::new(self.eval_expr(e, buffer)?));
                }
                self.log(&format!("⟨ψ| Superposition of {} states", values.len()), buffer);
                Ok(Value::Quantum(Box::new(QuantumState::Superposition(values))))
            }
            Expr::Collapse(expr) => {
                let val = self.eval_expr(*expr, buffer)?;
                match val {
                    Value::Quantum(qs) => match *qs {
                        QuantumState::Superposition(ref vals) => {
                            if vals.is_empty() {
                                return Err("Collapse error: Superposition is empty".to_string());
                            }
                            let idx = (self.pseudo_random() * vals.len() as f64) as usize;
                            let collapsed = vals[idx].clone();
                            self.log(&format!("|ψ⟩ Collapsed to: {}", collapsed), buffer);
                            Ok(*collapsed)
                        }
                        _ => Ok(Value::Quantum(qs)),
                    },
                    v => Ok(v),
                }
            }
            Expr::Manifest(name) => {
                if let Some(fragments) = self.memory_fragments.get_mut(&name) {
                    if let Some(val) = fragments.pop_back() {
                        self.log(&format!("◈ Manifesting '{}'", name), buffer);
                        return Ok(val);
                    }
                }
                Ok(Value::Null)
            }
            Expr::Entangle(var1, var2) => {
                self.log(&format!("⟨⟩ Entangling '{}' with '{}'", var1, var2), buffer);

                if let Some(val2) = self.get_var(&var2) {
                    self.set_var(var1.clone(), Value::Quantum(Box::new(QuantumState::Entangled(var2.clone()))));
                    Ok(val2)
                } else {
                    Ok(Value::Null)
                }
            }
            Expr::Rift(expr) => {
                self.log("⚠ Non-Euclidean expression", buffer);
                self.sanity -= 2.0;
                self.eval_expr(*expr, buffer)
            }
        }
    }
    
    fn apply_binary_op(&mut self, l: Value, op: BinaryOperator, r: Value, hash_influence: f64, instability: f64, buffer: &mut String) -> Result<Value, String> {
        match (l, r) {
            (Value::Number(a), Value::Number(b)) => {
                let result = match op {
                    BinaryOperator::Add => a + b,
                    BinaryOperator::Sub => a - b,
                    BinaryOperator::Mul => {
                        if instability > 0.3 {
                            a * b * (1.0 + (hash_influence - 0.5) * instability)
                        } else {
                            a * b
                        }
                    }
                    BinaryOperator::Div => {
                        if b == 0.0 {
                            self.log("⚠ Division by zero", buffer);
                            self.sanity -= 10.0;
                            f64::INFINITY
                        } else {
                            a / b
                        }
                    }
                    BinaryOperator::Eq => return Ok(Value::Boolean((a - b).abs() < 0.0001)),
                    BinaryOperator::Neq => return Ok(Value::Boolean((a - b).abs() >= 0.0001)),
                    BinaryOperator::Gt => return Ok(Value::Boolean(a > b)),
                    BinaryOperator::Lt => return Ok(Value::Boolean(a < b)),
                };
                Ok(Value::Number(result))
            }
            (Value::Boolean(a), Value::Boolean(b)) => {
                match op {
                    BinaryOperator::Eq => Ok(Value::Boolean(a == b)),
                    BinaryOperator::Neq => Ok(Value::Boolean(a != b)),
                    _ => Err("Invalid op on bools".to_string()),
                }
            }
            (Value::String(a), Value::String(b)) => {
                match op {
                    BinaryOperator::Add => Ok(Value::String(format!("{}{}", a, b))),
                    BinaryOperator::Eq => Ok(Value::Boolean(a == b)),
                    BinaryOperator::Neq => Ok(Value::Boolean(a != b)),
                    _ => Err("Invalid op on strings".to_string()),
                }
            }
            _ => Err("Type mismatch".to_string()),
        }
    }

    fn evaluate_condition(&mut self, val: &Value, drift: f64) -> bool {
        match val {
            Value::Boolean(b) => {
                if self.sanity < 40.0 && self.pseudo_random() > 0.8 { !b } else { *b }
            }
            Value::Number(n) => *n > (0.5 + drift * 0.3),
            Value::Quantum(qs) => match **qs {
                QuantumState::Superposition(ref vals) => {
                    if vals.is_empty() { return false; }
                    let idx = (self.pseudo_random() * vals.len() as f64) as usize;
                    self.is_truthy(&vals[idx])
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn is_truthy(&mut self, val: &Value) -> bool {
        match val {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Null => false,
            Value::Quantum(qs) => match **qs {
                QuantumState::Phantom => self.pseudo_random() > 0.5,
                _ => true,
            },
            _ => true,
        }
    }

    fn mutate_value(&mut self, val: Value) -> Value {
        let drift = self.temporal_drift();
        match val {
            Value::Number(n) => Value::Number(n + (drift * 15.0 - 7.5)),
            Value::Boolean(b) => {
                if drift > 0.7 { Value::Boolean(!b) } else { Value::Boolean(b) }
            }
            v => v,
        }
    }

    fn distort_output(&mut self, s: &str) -> String {
        let distortions = vec![" ", "◈", "⚠", "⟨", "⟩", "↯"];
        let mut result = String::new();
        for (_i, c) in s.chars().enumerate() {
            if self.pseudo_random() > 0.7 {
                let idx = (self.pseudo_random() * distortions.len() as f64) as usize;
                result.push_str(distortions[idx]);
            } else {
                result.push(c);
            }
        }
        result
    }
}

// ============================================================================ 
// WASM EXPORTS
// ============================================================================ 

// Hook removed for stability
// pub fn init_hooks() {}

#[wasm_bindgen]
pub struct YellowWebInterpreter {
    interpreter: Interpreter,
}

#[wasm_bindgen]
impl YellowWebInterpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        YellowWebInterpreter {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_code(&mut self, source: &str) -> String {
        let mut output_buffer = String::new();
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        match parser.parse_program() {
            Ok(ast) => {
                self.interpreter.log(&format!("Parsing complete. Sanity: {:.1}%", parser.sanity), &mut output_buffer);
                self.interpreter.log("\n╔════════════════════════════════════════╗", &mut output_buffer);
                self.interpreter.log("║  Beginning execution...                ║", &mut output_buffer);
                self.interpreter.log("╚════════════════════════════════════════╝\n", &mut output_buffer);
                
                match self.interpreter.execute(&ast, &mut output_buffer) {
                    Ok(_) => {
                        self.interpreter.log("\n╔════════════════════════════════════════╗", &mut output_buffer);
                        self.interpreter.log("║  Program completed                     ║", &mut output_buffer);
                        self.interpreter.log(&format!("║  Final sanity: {:<24.1}%║", self.interpreter.sanity), &mut output_buffer);
                        self.interpreter.log(&format!("║  Infections: {:<26} ║", self.interpreter.infections.len()), &mut output_buffer);
                        self.interpreter.log(&format!("║  Temporal echoes: {:<20} ║", self.interpreter.temporal_echoes.len()), &mut output_buffer);
                        self.interpreter.log(&format!("║  Memory fragments: {:<19} ║", self.interpreter.memory_fragments.len()), &mut output_buffer);
                        self.interpreter.log(&format!("║  Phantom variables: {:<18} ║", self.interpreter.phantom_variables.len()), &mut output_buffer);
                        self.interpreter.log(&format!("║  Generated code blocks: {:<14} ║", self.interpreter.generated_code.len()), &mut output_buffer);
                        self.interpreter.log("╚════════════════════════════════════════╝", &mut output_buffer);
                        
                        if self.interpreter.sanity < 30.0 {
                            self.interpreter.log("\n⚠⚠⚠ WARNING ⚠⚠⚠", &mut output_buffer);
                            self.interpreter.log("Critical sanity levels detected.", &mut output_buffer);
                            self.interpreter.log("Reality may be permanently compromised.", &mut output_buffer);
                        }
                    }
                    Err(e) => {
                        self.interpreter.log(&format!("\n⚠ Runtime horror: {}", e), &mut output_buffer);
                        self.interpreter.log("The code consumes itself...", &mut output_buffer);
                    }
                }
            }
            Err(e) => {
                self.interpreter.log(&format!("\n⚠ Parse error: {}", e), &mut output_buffer);
                self.interpreter.log(&format!("Sanity remaining: {:.1}%", parser.sanity), &mut output_buffer);
            }
        }

        output_buffer
    }

    pub fn get_sanity(&self) -> f64 {
        let val = self.interpreter.sanity;
        if val.is_nan() || val < 0.0 {
            0.0
        } else {
            val
        }
    }
}
