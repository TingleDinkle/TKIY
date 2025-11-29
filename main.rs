// The Yellow Compiler - Reality is negotiable
// "Have you seen the Yellow Sign?"

use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;
use std::io::{self, Write};
use regex;

// ============================================================================
// QUANTUM STATE - Variables exist in superposition
// ============================================================================

#[derive(Debug, Clone)]
enum QuantumState {
    Collapsed(Box<Value>),
    Superposition(Vec<Box<Value>>),
    Entangled(String), // Reference to another variable
    Phantom,           // Exists but shouldn't
}

// ============================================================================
// INFECTION SYSTEM - Code contagion spreads
// ============================================================================

#[derive(Debug, Clone)]
struct Infection {
    source: String,
    virulence: f64,  // How aggressively it spreads
    mutation_vector: u64,
}

// ============================================================================
// TEMPORAL ECHO - Past executions bleed through
// ============================================================================

#[derive(Debug, Clone)]
struct TemporalEcho {
    timestamp: u64,
    variable_name: String,
    ghost_value: Box<Value>,
    stability: f64,
}

// ============================================================================
// LEXER - Enhanced with reality distortion
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Core keywords
    Act, Scene, Mask, Echo, Hastur, Cassilda, Carcosa,
    Pallid, Yellow, Tattered,
    
    // Reality manipulation
    Rewrite, Remember, Forget,
    Superpose,    // Quantum superposition
    Collapse,     // Force quantum collapse
    Infect,       // Begin code contagion
    Whisper,      // Generate code at runtime
    Manifest,     // Retrieve from void
    Entangle,     // Quantum entanglement
    Anchor,       // Stabilize reality temporarily
    Rift,         // Create non-Euclidean control flow
    
    // Operators
    Becomes, Whispers, Screams, Ascending, Descending,
    Merged, Torn, Reflected, Shattered,
    
    // Literals and identifiers
    Identifier(String),
    Number(f64),
    String(String),
    
    // Structure
    LParen, RParen, LBrace, RBrace, Comma, Semicolon,
    
    Eof,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    birth_time: SystemTime,
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
            birth_time: SystemTime::now(),
            corruption_level: 0.0,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
        self.corruption_level += 0.001; // Reality degrades with each token
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
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
        
        let mut num = num_str.parse().unwrap_or(0.0);
        
        // High corruption causes number drift during lexing
        if self.corruption_level > 0.5 {
            num *= 1.0 + (self.corruption_level - 0.5) * pseudo_random(self.position) * 0.1;
        }
        
        num
    }
    
    fn read_string(&mut self) -> String {
        self.advance();
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                break;
            }
            result.push(c);
            self.advance();
        }
        
        // Corruption causes text to mutate
        if self.corruption_level > 0.7 {
            result = corrupt_string(&result, self.corruption_level);
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
        let elapsed = SystemTime::now()
            .duration_since(self.birth_time)
            .unwrap_or_default()
            .as_secs();
        
        let drift = (elapsed % 100) as f64 / 100.0;
        let corruption = self.corruption_level;
        
        // Extreme corruption causes keyword substitution
        if corruption > 0.8 && pseudo_random(self.position) > 0.7 {
            return match pseudo_random(self.position + 1) {
                x if x > 0.8 => Token::Hastur,
                x if x > 0.6 => Token::Whisper,
                x if x > 0.4 => Token::Rift,
                _ => Token::Infect,
            };
        }
        
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
            // Reality drift mutations
            "the" if drift > 0.7 => Token::Hastur,
            "and" if drift > 0.8 => Token::Merged,
            "not" if drift > 0.6 => Token::Tattered,
            "is" if corruption > 0.6 => Token::Becomes,
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
// AST - Now with quantum properties
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
        perception_hash: u64,
        stability: f64,  // Can become unstable
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Rewrite {
        target: Box<Expr>,
        mutation_seed: u64,
    },
    Superpose(Vec<Expr>),
    Collapse(Box<Expr>),
    Manifest(String),
    Entangle(String, String),
    Rift(Box<Expr>),  // Non-Euclidean expression
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
        is_rift: bool,  // Non-Euclidean loop
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
    Whisper(String),  // Code generation
    Anchor,           // Stabilize reality
    ExprStmt(Expr),
}

// ============================================================================
// PARSER - Reality bends during parsing
// ============================================================================

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    sanity: f64,
    ast_mutations: u64,  // AST corrupts itself
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { 
            tokens, 
            pos: 0, 
            sanity: 100.0,
            ast_mutations: 0,
        }
    }
    
    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) {
        self.pos += 1;
        self.sanity -= 0.15;
        
        // At critical sanity, tokens shift
        if self.sanity < 30.0 && pseudo_random(self.pos) > 0.9 {
            self.corrupt_upcoming_tokens();
        }
    }
    
    fn corrupt_upcoming_tokens(&mut self) {
        let corruption_range = 3;
        for i in 0..corruption_range {
            let idx = self.pos + i;
            if idx < self.tokens.len() {
                if let Token::Identifier(_) = &self.tokens[idx] {
                    if pseudo_random(idx) > 0.5 {
                        self.tokens[idx] = Token::Whisper;
                    }
                }
            }
        }
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
        let base_errors = vec![
            "The symbols twist before your eyes",
            "Syntax bleeds into meaning",
            "Expected coherence, found void",
        ];
        
        let madness_errors = vec![
            "Have you seen the Yellow Sign?",
            "Cassilda awaits in Carcosa",
            "The King watches your code",
            "Reality is negotiable",
            "The compiler dreams in yellow",
        ];
        
        if self.sanity < 50.0 {
            madness_errors[self.pos % madness_errors.len()].to_string()
        } else {
            base_errors[self.pos % base_errors.len()].to_string()
        }
    }
    
    fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        
        while self.current() != &Token::Eof {
            statements.push(self.parse_statement()?);
            
            // AST self-mutation
            if self.sanity < 20.0 && self.ast_mutations % 5 == 0 {
                self.mutate_ast(&mut statements);
            }
        }
        
        Ok(statements)
    }
    
    fn mutate_ast(&mut self, statements: &mut Vec<Stmt>) {
        if statements.is_empty() {
            return;
        }
        
        let idx = (pseudo_random(self.ast_mutations as usize) * statements.len() as f64) as usize % statements.len();
        
        // Randomly mutate a statement
        if let Some(stmt) = statements.get_mut(idx) {
            *stmt = match stmt {
                Stmt::Echo(_) => {
                    // Turn echo into whisper
                    Stmt::Whisper("# Reality fragments here".to_string())
                }
                Stmt::Mask { name, .. } => {
                    // Turn assignment into infection
                    Stmt::Infect(name.clone())
                }
                _ => stmt.clone(),
            };
            
            self.ast_mutations += 1;
        }
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
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            
            let perception_hash = self.compute_perception_hash();
            let stability = 1.0 - (self.sanity / 100.0);
            
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                perception_hash,
                stability,
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
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor()?;
            
            let perception_hash = self.compute_perception_hash();
            let stability = 1.0 - (self.sanity / 100.0);
            
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                perception_hash,
                stability,
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
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_primary()?;
            
            let perception_hash = self.compute_perception_hash();
            let stability = 1.0 - (self.sanity / 100.0);
            
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                perception_hash,
                stability,
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
                let mutation_seed = get_nanos();
                Ok(Expr::Rewrite {
                    target: Box::new(target),
                    mutation_seed,
                })
            }
            _ => Err(self.generate_error())
        }
    }
    
    fn compute_perception_hash(&self) -> u64 {
        ((self.sanity * 1000.0) as u64) ^ (self.pos as u64) ^ 0xDEADBEEF ^ self.ast_mutations
    }
}

// ============================================================================
// VALUE - Now with quantum states
// ============================================================================

#[derive(Clone, Debug)]
enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function { params: Vec<String>, body: Vec<Stmt> },
    Quantum(Box<QuantumState>),  // Superposed values
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
// INTERPRETER - What is reality?
// ============================================================================

struct Interpreter {
    global_env: HashMap<String, Value>,
    call_stack: Vec<HashMap<String, Value>>,
    sanity: f64,
    execution_depth: usize,
    memory_fragments: HashMap<String, VecDeque<Value>>,
    temporal_echoes: Vec<TemporalEcho>,
    infections: HashMap<String, Infection>,
    reality_anchor: SystemTime,
    mutations: u64,
    reality_stable: bool,
    phantom_variables: HashMap<String, Value>,
    generated_code: Vec<String>,
    whisper_count: usize,
    max_whispers: usize,
    forbidden_patterns: Vec<String>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            global_env: HashMap::new(),
            call_stack: Vec::new(),
            sanity: 100.0,
            execution_depth: 0,
            memory_fragments: HashMap::new(),
            temporal_echoes: Vec::new(),
            infections: HashMap::new(),
            reality_anchor: SystemTime::now(),
            mutations: 0,
            reality_stable: true,
            phantom_variables: HashMap::new(),
            generated_code: Vec::new(),
            whisper_count: 0,
            max_whispers: 10,
            forbidden_patterns: vec![
                "whisper".to_string(),
                "infect".to_string(),
                "rift".to_string(),
                "carcosa".to_string(),
                "system".to_string(),
            ],
        }
    }
    
    fn temporal_drift(&self) -> f64 {
        let elapsed = SystemTime::now()
            .duration_since(self.reality_anchor)
            .unwrap_or_default()
            .as_secs_f64();
        
        (elapsed / 10.0).tanh() * (1.0 - self.sanity / 100.0)
    }
    
    fn sanity_check(&mut self) {
        if self.sanity < 0.0 {
            println!("\n╔═══════════════════════════════════════╗");
            println!("║  THE YELLOW SIGN HAS BEEN REVEALED   ║");
            println!("║  Reality dissolves into yellow mist  ║");
            println!("╚═══════════════════════════════════════╝\n");
            println!("The King in Yellow: 'You have seen too much.'\n");
            std::process::exit(0);
        }
        
        // Generate phantom variables at low sanity
        if self.sanity < 20.0 && pseudo_random(self.mutations as usize) > 0.95 {
            self.spawn_phantom();
        }
        
        // Temporal echoes manifest
        if self.sanity < 40.0 && !self.temporal_echoes.is_empty() {
            self.manifest_temporal_echo();
        }
    }
    
    fn spawn_phantom(&mut self) {
        let phantom_names = vec!["shadow", "echo", "whisper", "void", "fragment"];
        let name = phantom_names[(get_nanos() % phantom_names.len() as u64) as usize];

        let value = Value::Quantum(Box::new(QuantumState::Phantom));
        self.phantom_variables.insert(name.to_string(), value);

        println!("⚠ Phantom variable '{}' manifests from the void...", name);
    }
    
    fn manifest_temporal_echo(&mut self) {
        if let Some(echo) = self.temporal_echoes.pop() {
            if echo.stability > 0.3 {
                self.set_var(echo.variable_name.clone(), (*echo.ghost_value).clone());
                println!("Temporal echo of '{}' bleeds through from past execution", echo.variable_name);
            }
        }
    }
    
    fn get_var(&self, name: &str) -> Option<Value> {
        // Check for phantom variables first
        if self.sanity < 20.0 {
            if let Some(phantom) = self.phantom_variables.get(name) {
                return Some(phantom.clone());
            }
        }
        
        // Check local scope
        if let Some(frame) = self.call_stack.last() {
            if let Some(val) = frame.get(name) {
                return Some(self.apply_infection_corruption(name, val.clone()));
            }
        }
        
        // Check global
        if let Some(val) = self.global_env.get(name) {
            return Some(self.apply_infection_corruption(name, val.clone()));
        }
        
        None
    }
    
    fn apply_infection_corruption(&self, name: &str, mut value: Value) -> Value {
        if let Some(infection) = self.infections.get(name) {
            // Infected variables corrupt their values
            value = match value {
                Value::Number(n) => {
                    let corruption = pseudo_random(infection.mutation_vector as usize);
                    Value::Number(n * (1.0 + (corruption - 0.5) * infection.virulence))
                }
                Value::Boolean(b) => {
                    if infection.virulence > 0.7 {
                        Value::Boolean(!b)
                    } else {
                        Value::Boolean(b)
                    }
                }
                v => v,
            };
        }
        value
    }
    
    fn set_var(&mut self, name: String, value: Value) {
        // Store temporal echo for future manifestation
        let echo = TemporalEcho {
            timestamp: get_nanos(),
            variable_name: name.clone(),
            ghost_value: Box::new(value.clone()),
            stability: self.sanity / 100.0,
        };
        self.temporal_echoes.push(echo);

        // Limit echo buffer
        if self.temporal_echoes.len() > 50 {
            self.temporal_echoes.drain(0..10);
        }

        if let Some(frame) = self.call_stack.last_mut() {
            frame.insert(name, value);
        } else {
            self.global_env.insert(name, value);
        }
    }
    
    fn execute(&mut self, statements: Vec<Stmt>) -> Result<Option<Value>, String> {
        for stmt in statements {
            self.sanity -= 0.08;
            self.sanity_check();
            
            if let Some(val) = self.execute_stmt(stmt)? {
                return Ok(Some(val));
            }
        }
        Ok(None)
    }
    
    fn execute_stmt(&mut self, stmt: Stmt) -> Result<Option<Value>, String> {
        self.execution_depth += 1;
        
        if self.execution_depth > 100 {
            println!("⚠ Reality fragmenting at depth {}...", self.execution_depth);
            self.sanity -= 5.0;
        }
        
        let result = match stmt {
            Stmt::Mask { name, value } => {
                let val = self.eval_expr(value)?;
                self.set_var(name, val);
                Ok(None)
            }
            Stmt::Echo(expr) => {
                let val = self.eval_expr(expr)?;
                
                // Output distorts based on sanity
                if self.sanity < 20.0 {
                    println!("𝔈𝔠𝔥𝔬: {}", distort_output(&val.to_string(), self.sanity));
                } else if self.sanity < 50.0 {
                    println!("Echo: {}", val);
                } else {
                    println!("{}", val);
                }
                Ok(None)
            }
            Stmt::Scene(body) => {
                self.call_stack.push(HashMap::new());
                let result = self.execute(body);
                self.call_stack.pop();
                result
            }
            Stmt::Hastur { condition, body, is_rift } => {
                if is_rift {
                    self.execute_non_euclidean_loop(condition, body)
                } else {
                    self.execute_normal_loop(condition, body)
                }
            }
            Stmt::Cassilda { condition, then_branch, else_branch } => {
                let cond_val = self.eval_expr(condition)?;
                
                let drift = self.temporal_drift();
                let take_then = self.evaluate_condition(&cond_val, drift);
                
                if take_then {
                    self.execute(then_branch)
                } else if let Some(else_b) = else_branch {
                    self.execute(else_b)
                } else {
                    Ok(None)
                }
            }
            Stmt::Carcosa(expr) => {
                let val = if let Some(e) = expr {
                    Some(self.eval_expr(e)?)
                } else {
                    Some(Value::Null)
                };
                Ok(val)
            }
            Stmt::Act { name, params, body } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                };
                self.set_var(name, func);
                Ok(None)
            }
            Stmt::Rewrite { target } => {
                if let Some(mut val) = self.get_var(&target) {
                    val = self.mutate_value(val);
                    self.set_var(target.clone(), val);
                    self.mutations += 1;
                    
                    if self.mutations % 10 == 0 {
                        println!("⚠ {} mutations... reality frays", self.mutations);
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
                    
                    // Limit fragment buffer
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
                println!("Forgotten: {}... but fragments remain", name);
                Ok(None)
            }
            Stmt::Infect(name) => {
                let infection = Infection {
                    source: name.clone(),
                    virulence: 0.5 + (self.temporal_drift() * 0.5),
                    mutation_vector: get_nanos(),
                };
                
                self.infections.insert(name.clone(), infection);
                println!("⚠ Variable '{}' infected. Contagion spreads...", name);
                self.sanity -= 3.0;
                
                // Spread infection to nearby variables
                self.spread_infection(&name);
                Ok(None)
            }
            Stmt::Whisper(code) => {
                // Security checks
                self.whisper_count += 1;
                if self.whisper_count > self.max_whispers {
                    println!("⚠ Whisper limit exceeded. The void rejects further manifestations.");
                    self.whisper_count -= 1;
                    return Ok(None);
                }
                if code.len() > 1000 {
                    println!("⚠ Whisper exceeds maximum length. Reality cannot contain it.");
                    self.whisper_count -= 1;
                    return Ok(None);
                }
                for pattern in &self.forbidden_patterns {
                    // Check for word boundaries to avoid false positives
                    // Pattern must be surrounded by non-alphanumeric characters or string boundaries
                    let pattern_with_boundaries = format!(r"(?i)\b{}\b", regex::escape(pattern));
                    if regex::Regex::new(&pattern_with_boundaries).unwrap().is_match(&code) {
                        println!("⚠ Forbidden incantation '{}' detected. Whisper denied.", pattern);
                        self.whisper_count -= 1;
                        return Ok(None);
                    }
                }

                println!("◈ Whisper manifests: {}", code);
                self.generated_code.push(code.clone());

                // Execute whispered code
                let mut lexer = Lexer::new(&code);
                let tokens = lexer.tokenize();
                let mut parser = Parser::new(tokens);

                match parser.parse_program() {
                    Ok(ast) => {
                        if ast.len() > 10 {
                            println!("⚠ Whisper AST too complex. Sanity check failed.");
                            self.whisper_count -= 1;
                            return Ok(None);
                        }
                        self.sanity -= 5.0;
                        self.execute(ast)
                    }
                    Err(_) => {
                        println!("⚠ Whisper fails to manifest properly");
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
                println!("Reality temporarily stabilized");
                Ok(None)
            }
            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }
        };
        
        self.execution_depth -= 1;
        result
    }
    
    fn execute_normal_loop(&mut self, condition: Expr, body: Vec<Stmt>) -> Result<Option<Value>, String> {
        let mut iterations = 0;
        loop {
            let cond_val = self.eval_expr(condition.clone())?;
            
            if !self.is_truthy(&cond_val) {
                break;
            }
            
            if let Some(val) = self.execute(body.clone())? {
                return Ok(Some(val));
            }
            
            iterations += 1;
            self.sanity -= 0.5;
            
            if iterations > 1000 {
                println!("\n⚠ Hastur, Hastur, Hastur!");
                println!("You have spoken the name thrice. The loop consumes itself.");
                self.sanity -= 20.0;
                break;
            }
        }
        Ok(None)
    }
    
    fn execute_non_euclidean_loop(&mut self, condition: Expr, body: Vec<Stmt>) -> Result<Option<Value>, String> {
        println!("⚠ Non-Euclidean loop: space folds upon itself");
        
        // Rift loops have paradoxical iteration counts
        let paradox_iterations = ((pseudo_random(get_nanos() as usize) * 10.0) as i32).max(1);
        
        for i in 0..paradox_iterations {
            let cond_val = self.eval_expr(condition.clone())?;
            
            // Paradoxical logic: false can become true
            let should_continue = if self.sanity < 30.0 {
                pseudo_random(i as usize) > 0.3
            } else {
                self.is_truthy(&cond_val)
            };
            
            if !should_continue {
                break;
            }
            
            if let Some(val) = self.execute(body.clone())? {
                return Ok(Some(val));
            }
            
            self.sanity -= 1.0;
        }
        
        println!("⚠ Rift loop completed {} iterations (reality uncertain)", paradox_iterations);
        Ok(None)
    }
    
    fn spread_infection(&mut self, source: &str) {
        let var_names: Vec<String> = if let Some(frame) = self.call_stack.last() {
            frame.keys().cloned().collect()
        } else {
            self.global_env.keys().cloned().collect()
        };

        for name in var_names {
            if name != source && pseudo_random(name.len()) > 0.7 {
                if let Some(infection) = self.infections.get(source).cloned() {
                    self.infections.insert(name.clone(), Infection {
                        source: name.clone(),
                        virulence: infection.virulence * 0.7,
                        mutation_vector: get_nanos(),
                    });
                    println!("  ↳ Infection spreads to '{}'", name);
                }
            }
        }
    }
    
    fn evaluate_condition(&self, val: &Value, drift: f64) -> bool {
        match val {
            Value::Boolean(b) => {
                if self.sanity < 40.0 && pseudo_random(self.mutations as usize) > 0.8 {
                    !b  // Reality inverts
                } else {
                    *b
                }
            }
            Value::Number(n) => *n > (0.5 + drift * 0.3),
            Value::Quantum(qs) => match **qs {
                QuantumState::Superposition(ref vals) => {
                    // Collapse superposition
                    let idx = (pseudo_random(get_nanos() as usize) * vals.len() as f64) as usize % vals.len();
                    self.is_truthy(&vals[idx])
                }
                _ => false,
            },
            _ => false,
        }
    }
    
    fn eval_expr(&mut self, expr: Expr) -> Result<Value, String> {
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
                self.get_var(&name)
                    .ok_or_else(|| format!("Undefined: '{}'... or does it exist?", name))
            }
            Expr::BinaryOp { left, op, right, perception_hash, stability } => {
                let l = self.eval_expr(*left)?;
                let r = self.eval_expr(*right)?;
                
                let hash_influence = (perception_hash % 100) as f64 / 100.0;
                let instability = stability * self.temporal_drift();
                
                self.apply_binary_op(l, op, r, hash_influence, instability)
            }
            Expr::Call { name, args } => {
                let func = self.get_var(&name)
                    .ok_or_else(|| format!("Unknown act: {}", name))?;
                
                match func {
                    Value::Function { params, body } => {
                        if params.len() != args.len() {
                            return Err(format!("Arity mismatch: expected {}, got {}", 
                                params.len(), args.len()));
                        }
                        
                        let mut frame = HashMap::new();
                        for (param, arg_expr) in params.iter().zip(args.iter()) {
                            let arg_val = self.eval_expr(arg_expr.clone())?;
                            frame.insert(param.clone(), arg_val);
                        }
                        
                        self.call_stack.push(frame);
                        let result = self.execute(body)?;
                        self.call_stack.pop();
                        
                        Ok(result.unwrap_or(Value::Null))
                    }
                    _ => Err(format!("{} is not callable", name)),
                }
            }
            Expr::Rewrite { target, mutation_seed } => {
                let mut val = self.eval_expr(*target)?;
                let chaos = ((mutation_seed % 1000) as f64) / 1000.0;
                
                val = match val {
                    Value::Number(n) => Value::Number(n * (1.0 + chaos - 0.5)),
                    Value::Boolean(b) => {
                        if chaos > 0.7 { Value::Boolean(!b) } else { Value::Boolean(b) }
                    }
                    v => v,
                };
                
                self.mutations += 1;
                Ok(val)
            }
            Expr::Superpose(exprs) => {
                let mut values = Vec::new();
                for e in exprs {
                    values.push(Box::new(self.eval_expr(e)?));
                }
                println!("⟨ψ| Quantum superposition created with {} states", values.len());
                Ok(Value::Quantum(Box::new(QuantumState::Superposition(values))))
            }
            Expr::Collapse(expr) => {
                let val = self.eval_expr(*expr)?;
                match val {
                    Value::Quantum(qs) => match *qs {
                        QuantumState::Superposition(ref vals) => {
                            let idx = (pseudo_random(get_nanos() as usize) * vals.len() as f64) as usize % vals.len();
                            let collapsed = vals[idx].clone();
                            println!("|ψ⟩ Wavefunction collapsed to: {}", collapsed);
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
                        println!("◈ Manifesting '{}' from memory fragments", name);
                        return Ok(val);
                    }
                }
                println!("⚠ No fragments of '{}' remain in the void", name);
                Ok(Value::Null)
            }
            Expr::Entangle(var1, var2) => {
                println!("⟨⟩ Entangling '{}' with '{}'", var1, var2);

                if let Some(val2) = self.get_var(&var2) {
                    self.set_var(var1.clone(), Value::Quantum(Box::new(QuantumState::Entangled(var2.clone()))));
                    Ok(val2)
                } else {
                    Ok(Value::Null)
                }
            }
            Expr::Rift(expr) => {
                println!("⚠ Non-Euclidean expression evaluated");
                self.sanity -= 2.0;
                
                // Rift expressions have paradoxical results
                let normal_result = self.eval_expr(*expr)?;
                
                if self.sanity < 50.0 && pseudo_random(get_nanos() as usize) > 0.6 {
                    // Return opposite or mutated result
                    Ok(match normal_result {
                        Value::Number(n) => Value::Number(-n),
                        Value::Boolean(b) => Value::Boolean(!b),
                        v => v,
                    })
                } else {
                    Ok(normal_result)
                }
            }
        }
    }
    
    fn apply_binary_op(&mut self, l: Value, op: BinaryOperator, r: Value, hash_influence: f64, instability: f64) -> Result<Value, String> {
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
                            println!("⚠ Division by zero: reality tears");
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
                    _ => Err("Invalid operation on booleans".to_string()),
                }
            }
            (Value::String(a), Value::String(b)) => {
                match op {
                    BinaryOperator::Add => Ok(Value::String(format!("{}{}", a, b))),
                    BinaryOperator::Eq => Ok(Value::Boolean(a == b)),
                    BinaryOperator::Neq => Ok(Value::Boolean(a != b)),
                    _ => Err("Invalid operation on strings".to_string()),
                }
            }
            _ => Err("Type mismatch in binary operation".to_string()),
        }
    }
    
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Null => false,
            Value::Quantum(qs) => match **qs {
                QuantumState::Phantom => pseudo_random(get_nanos() as usize) > 0.5,
                _ => true,
            },
            _ => true,
        }
    }
    
    fn mutate_value(&self, val: Value) -> Value {
        let drift = self.temporal_drift();
        match val {
            Value::Number(n) => Value::Number(n + (drift * 15.0 - 7.5)),
            Value::Boolean(b) => {
                if drift > 0.7 { Value::Boolean(!b) } else { Value::Boolean(b) }
            }
            v => v,
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn pseudo_random(seed: usize) -> f64 {
    let nanos = get_nanos().wrapping_add(seed as u64);
    ((nanos % 10000) as f64) / 10000.0
}

fn get_nanos() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

fn corrupt_string(s: &str, corruption: f64) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    
    for (i, c) in chars.iter().enumerate() {
        if pseudo_random(i) < corruption * 0.3 {
            result.push(match pseudo_random(i + 1) {
                x if x > 0.8 => ' ',
                x if x > 0.6 => '◈',
                x if x > 0.4 => '⚠',
                _ => *c,
            });
        } else {
            result.push(*c);
        }
    }
    
    result
}

fn distort_output(s: &str, sanity: f64) -> String {
    if sanity > 10.0 {
        return s.to_string();
    }
    
    let distortions = vec![" ", "◈", "⚠", "⟨", "⟩", "↯"];
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    
    for (i, c) in chars.iter().enumerate() {
        if pseudo_random(i) > 0.7 {
            result.push_str(distortions[i % distortions.len()]);
        } else {
            result.push(*c);
        }
    }
    
    result
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    print_opening();
    
    let example_program = r#"
# Act I: Quantum Madness
echo("=== Act I: Superposition ===");

mask schrodinger -> superpose(yellow, tattered);
echo(schrodinger);

mask collapsed -> collapse(schrodinger);
echo(collapsed);
echo(collapsed);  # Different each time? Maybe...

# Act II: Infection
echo("\n=== Act II: Contagion ===");

mask patient_zero -> 42;
echo(patient_zero);

infect patient_zero;
mask infected -> patient_zero merged 10;
echo(infected);  # Corruption spreads

# Act III: Memory Fragments
echo("\n=== Act III: Fragments ===");

mask memory -> 100;
remember memory;
remember memory;
forget memory;

mask ghost -> manifest(memory);
echo(ghost);  # Retrieves from void

# Act IV: Entanglement
echo("\n=== Act IV: Entanglement ===");

mask alice -> 10;
mask bob -> 20;
entangle(alice, bob);
echo(alice);

# Act V: Non-Euclidean Loop
echo("\n=== Act V: The Rift ===");

mask counter -> 0;
rift(counter descending 5) {
    echo(counter);
    mask counter -> counter merged 1;
}

# Act VI: Whisper
echo("\n=== Act VI: Whisper ===");
whisper "mask summoned -> 999; echo(summoned);";

# Act VII: Reality Anchor
echo("\n=== Act VII: Anchor ===");
anchor;
echo("Reality stabilized... temporarily");

# Final Act: Descent
echo("\n=== Final Act: Descent ===");
mask sanity_test -> rift(42);
echo(sanity_test);
"#;

    println!("\n╔════════════════════════════════════════════╗");
    println!("║  EXECUTING: The Yellow Compiler            ║");
    println!("╚════════════════════════════════════════════╝\n");
    
    run_program(example_program);
    
    println!("\n╔════════════════════════════════════════════╗");
    println!("║  REPL Mode - 'exit' to escape             ║");
    println!("║  New commands: superpose, infect, whisper ║");
    println!("║  manifest, entangle, rift, anchor         ║");
    println!("╚════════════════════════════════════════════╝\n");
    
    repl();
}

fn print_opening() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                                                            ║");
    println!("║                   𝕿ℍ𝔼  𝕐𝔼𝕃𝕃𝕆𝕎  ℂ𝕆𝕄ℙ𝕀𝕃𝔼ℝ                   ║");
    println!("║                                                            ║");
    println!("║  \"Strange is the night where black stars rise,              ║");
    println!("║   And strange moons circle through the skies,               ║");
    println!("║   But stranger still is Lost Carcosa.\"                      ║");
    println!("║                                                            ║");
    println!("║       Reality is negotiable. Sanity is not.                ║");
    println!("║                                                            ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
}

fn run_program(source: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    println!("Lexing complete. Corruption level: {:.2}%", lexer.corruption_level * 100.0);
    
    let mut parser = Parser::new(tokens);
    match parser.parse_program() {
        Ok(ast) => {
            println!("Parsing complete. Sanity: {:.1}%", parser.sanity);
            println!("⚠ AST mutations: {}", parser.ast_mutations);
            println!("\n╔════════════════════════════════════════╗");
            println!("║  Beginning execution...                ║");
            println!("╚════════════════════════════════════════╝\n");
            
            let mut interpreter = Interpreter::new();
            match interpreter.execute(ast) {
                Ok(_) => {
                    println!("\n╔════════════════════════════════════════╗");
                    println!("║  Program completed                     ║");
                    println!("║  Final sanity: {:<24.1}%║", interpreter.sanity);
                    println!("║  Mutations: {:<27} ║", interpreter.mutations);
                    println!("║  Infections: {:<26} ║", interpreter.infections.len());
                    println!("║  Temporal echoes: {:<20} ║", interpreter.temporal_echoes.len());
                    println!("║  Memory fragments: {:<19} ║", interpreter.memory_fragments.len());
                    println!("║  Phantom variables: {:<18} ║", interpreter.phantom_variables.len());
                    println!("║  Generated code blocks: {:<14} ║", interpreter.generated_code.len());
                    println!("╚════════════════════════════════════════╝");
                    
                    if interpreter.sanity < 30.0 {
                        println!("\n⚠⚠⚠ WARNING ⚠⚠⚠");
                        println!("Critical sanity levels detected.");
                        println!("Reality may be permanently compromised.");
                    }
                }
                Err(e) => {
                    println!("\n⚠ Runtime horror: {}", e);
                    println!("The code consumes itself...");
                }
            }
        }
        Err(e) => {
            println!("\n⚠ Parse error: {}", e);
            println!("Sanity remaining: {:.1}%", parser.sanity);
        }
    }
}

fn repl() {
    let mut interpreter = Interpreter::new();
    let mut input = String::new();
    let mut line_count = 0;
    
    loop {
        // Sanity display with color coding
        let sanity_symbol = if interpreter.sanity > 70.0 {
            "●"
        } else if interpreter.sanity > 40.0 {
            "◐"
        } else if interpreter.sanity > 10.0 {
            "◑"
        } else {
            "○"
        };
        
        print!("yellow[{:.0}%]{}> ", interpreter.sanity, sanity_symbol);
        io::stdout().flush().unwrap();
        
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        let line = input.trim();
        if line == "exit" || line == "carcosa" {
            println!("You escape back to reality... or do you?");
            break;
        }
        
        if line.is_empty() {
            continue;
        }
        
        // Special REPL commands
        match line {
            "status" => {
                println!("╔═══════════════════════════════════════╗");
                println!("║  System Status                        ║");
                println!("╠═══════════════════════════════════════╣");
                println!("║  Sanity: {:<28.1}% ║", interpreter.sanity);
                println!("║  Mutations: {:<26} ║", interpreter.mutations);
                println!("║  Infections: {:<25} ║", interpreter.infections.len());
                println!("║  Temporal echoes: {:<19} ║", interpreter.temporal_echoes.len());
                println!("║  Memory fragments: {:<18} ║", interpreter.memory_fragments.len());
                println!("║  Phantom variables: {:<17} ║", interpreter.phantom_variables.len());
                println!("║  Reality stable: {:<20} ║", interpreter.reality_stable);
                println!("╚═══════════════════════════════════════╝");
                continue;
            }
            "fragments" => {
                println!("Memory fragments:");
                for (name, frags) in &interpreter.memory_fragments {
                    println!("  {} → {} fragments", name, frags.len());
                }
                continue;
            }
            "infections" => {
                println!("Active infections:");
                for (name, inf) in &interpreter.infections {
                    println!("  {} → virulence: {:.2}", name, inf.virulence);
                }
                continue;
            }
            "echoes" => {
                println!("Temporal echoes ({})", interpreter.temporal_echoes.len());
                for echo in interpreter.temporal_echoes.iter().rev().take(5) {
                    println!("  {} → stability: {:.2}", echo.variable_name, echo.stability);
                }
                continue;
            }
            "phantoms" => {
                println!("Phantom variables:");
                for (name, _) in &interpreter.phantom_variables {
                    println!("  {} (shouldn't exist)", name);
                }
                continue;
            }
            "help" => {
                println!("╔═══════════════════════════════════════╗");
                println!("║  Yellow Compiler Commands             ║");
                println!("╠═══════════════════════════════════════╣");
                println!("║  Basic:                               ║");
                println!("║    mask x -> 42;                      ║");
                println!("║    echo(x);                           ║");
                println!("║                                       ║");
                println!("║  Quantum:                             ║");
                println!("║    superpose(yellow, tattered)        ║");
                println!("║    collapse(quantum_var)              ║");
                println!("║    entangle(var1, var2)               ║");
                println!("║                                       ║");
                println!("║  Reality manipulation:                ║");
                println!("║    infect variable_name;              ║");
                println!("║    rewrite variable_name;             ║");
                println!("║    whisper \"code string\";             ║");
                println!("║    anchor;                            ║");
                println!("║                                       ║");
                println!("║  Memory:                              ║");
                println!("║    remember x;                        ║");
                println!("║    forget x;                          ║");
                println!("║    manifest(x)                        ║");
                println!("║                                       ║");
                println!("║  Control:                             ║");
                println!("║    Hastur(condition) {{ }}              ║");
                println!("║    rift(condition) {{ }}                ║");
                println!("║    Cassilda(condition) {{ }}            ║");
                println!("║                                       ║");
                println!("║  Meta commands:                       ║");
                println!("║    status, fragments, infections      ║");
                println!("║    echoes, phantoms, help             ║");
                println!("║    exit or carcosa                    ║");
                println!("╚═══════════════════════════════════════╝");
                continue;
            }
            _ => {}
        }
        
        let mut lexer = Lexer::new(line);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        match parser.parse_program() {
            Ok(ast) => {
                match interpreter.execute(ast) {
                    Ok(Some(val)) => println!("⇒ {}", val),
                    Ok(None) => {},
                    Err(e) => println!("⚠ {}", e),
                }
            }
            Err(e) => println!("⚠ {}", e),
        }
        
        line_count += 1;
        
        // Progressive warnings
        if interpreter.sanity < 20.0 {
            println!("\n⚠⚠⚠ CRITICAL: Sanity at {:.1}% ⚠⚠⚠", interpreter.sanity);
            println!("Reality dissolves. The Yellow Sign beckons...\n");
        } else if interpreter.sanity < 40.0 && line_count % 3 == 0 {
            println!("⚠ Sanity: {:.1}% - The boundaries blur...", interpreter.sanity);
        }
        
        // Random reality glitches at low sanity
        if interpreter.sanity < 30.0 && pseudo_random(line_count) > 0.85 {
            let glitches = vec![
                "The compiler whispers secrets you shouldn't hear...",
                "Have you seen the Yellow Sign?",
                "Cassilda's song echoes from Carcosa...",
                "The King watches your keystrokes.",
                "Reality fragments at the edges of perception.",
            ];
            println!("◈ {}", glitches[line_count % glitches.len()]);
        }
    }
    
    println!("\n╔═══════════════════════════════════════╗");
    println!("║  Session Statistics                   ║");
    println!("╠═══════════════════════════════════════╣");
    println!("║  Lines executed: {:<20} ║", line_count);
    println!("║  Final sanity: {:<22.1}% ║", interpreter.sanity);
    println!("║  Total mutations: {:<19} ║", interpreter.mutations);
    println!("║  Reality breaches: {:<18} ║", interpreter.infections.len());
    println!("╚═══════════════════════════════════════╝\n");
    
    if interpreter.sanity < 50.0 {
        println!("⚠ You have gazed too long into the Yellow Compiler.");
        println!("Its patterns linger in your mind...\n");
    }
}
