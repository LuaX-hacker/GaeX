use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
Numero(f64),
Texto(String),

Booleano(bool),

Ident(String),

BinOp {
left: Box<Expr>,
op: Token,
right: Box<Expr>,
},

UnOp {
op: Token,
expr: Box<Expr>,
},

Atribuicao {
nome: String,
valor: Box<Expr>,
},

Chamada {
nome: String,
args: Vec<Expr>,
},

}

#[derive(Debug, Clone)]
pub enum Stmt {

Expr(Expr),

Se {
cond: Expr,
corpo: Vec<Stmt>,
senao: Vec<Stmt>,
},

Enquanto {
cond: Expr,
corpo: Vec<Stmt>,
},

Funcao {
nome: String,
params: Vec<String>,
corpo: Vec<Stmt>,
},

Retorna(Expr),

Empty,

}

pub struct Parser {

tokens: Vec<Token>,
pos: usize,

}

impl Parser {

pub fn new(tokens: Vec<Token>) -> Self {

Self {

tokens,
pos: 0,

}

}

fn acabou(&self) -> bool {

matches!(self.peek(), Token::EOF)

}

fn peek(&self) -> &Token {

&self.tokens[self.pos]

}

fn peek_next(&self) -> &Token {

if self.pos + 1 >= self.tokens.len() {

self.peek()

} else {

&self.tokens[self.pos + 1]

}

}

fn advance(&mut self) -> Token {

let token = self.tokens[self.pos].clone();

if !self.acabou() {

self.pos += 1;

}

token

}

fn check(&self, token: &Token) -> bool {

self.peek() == token

}

fn match_token(&mut self, token: &Token) -> bool {

if self.check(token) {

self.advance();
true

} else {

false

}

}

fn consumir(
&mut self,
esperado: &Token,
mensagem: &str,
) -> Result<(), String> {

if self.check(esperado) {

self.advance();
Ok(())

} else {

Err(mensagem.to_string())

}

}

pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {

let mut stmts = Vec::new();

while !self.acabou() {

stmts.push(self.statement()?);

}

Ok(stmts)

}

fn statement(&mut self) -> Result<Stmt, String> {

    if self.match_token(&Token::Se) {
        return self.if_statement();
    }

    if self.match_token(&Token::Enquanto) {
        return self.while_statement();
    }

    if self.match_token(&Token::Funcao) {
        return self.function_statement();
    }

    if self.match_token(&Token::Retorna) {
        let valor = self.expression()?;
        return Ok(Stmt::Retorna(valor));
    }

    let expr = self.expression()?;
    Ok(Stmt::Expr(expr))
}

fn expression(&mut self) -> Result<Expr, String> {
self.assignment()
}

fn assignment(&mut self) -> Result<Expr, String> {

    let expr = self.or()?;

    if self.match_token(&Token::Igual) {

        let valor = self.assignment()?;

        if let Expr::Ident(nome) = expr {
            return Ok(Expr::Atribuicao {
                nome,
                valor: Box::new(valor),
            });
        }

        return Err("Destino inválido para atribuição.".to_string());
    }

    Ok(expr)
}

fn or(&mut self) -> Result<Expr, String> {

let mut expr = self.and()?;

while self.match_token(&Token::OU) {

let op = Token::OU;
let right = self.and()?;

expr = Expr::BinOp {

left: Box::new(expr),
op,
right: Box::new(right),

};

}

Ok(expr)

}

fn and(&mut self) -> Result<Expr, String> {

let mut expr = self.equality()?;

while self.match_token(&Token::E) {

let op = Token::E;
let right = self.equality()?;

expr = Expr::BinOp {

left: Box::new(expr),
op,
right: Box::new(right),

};

}

Ok(expr)

}

fn equality(&mut self) -> Result<Expr, String> {

let mut expr = self.comparison()?;

loop {

let op = if self.match_token(&Token::IgualIgual) {

Token::IgualIgual

} else if self.match_token(&Token::Diferente) {

Token::Diferente

} else {

break;

};

let right = self.comparison()?;

expr = Expr::BinOp {

left: Box::new(expr),
op,
right: Box::new(right),

};

}

Ok(expr)

}

fn comparison(&mut self) -> Result<Expr, String> {

let mut expr = self.term()?;

loop {

let op =
if self.match_token(&Token::Maior) {

Token::Maior

} else if self.match_token(&Token::MaiorIgual) {

Token::MaiorIgual

} else if self.match_token(&Token::Menor) {

Token::Menor

} else if self.match_token(&Token::MenorIgual) {

Token::MenorIgual

} else {

break;

};

let right = self.term()?;

expr = Expr::BinOp {

left: Box::new(expr),
op,
right: Box::new(right),

};

}

Ok(expr)

}

fn term(&mut self) -> Result<Expr, String> {

let mut expr = self.factor()?;

loop {

let op =
if self.match_token(&Token::Mais) {

Token::Mais

} else if self.match_token(&Token::Menos) {

Token::Menos

} else {

break;

};

let right = self.factor()?;

expr = Expr::BinOp {

left: Box::new(expr),
op,
right: Box::new(right),

};

}

Ok(expr)

}

fn factor(&mut self) -> Result<Expr, String> {

let mut expr = self.unary()?;

loop {

let op =
if self.match_token(&Token::Asterisco) {

Token::Asterisco

} else if self.match_token(&Token::Barra) {

Token::Barra

} else if self.match_token(&Token::Porcentagem) {

Token::Porcentagem

} else {

break;

};

let right = self.unary()?;

expr = Expr::BinOp {

left: Box::new(expr),
op,
right: Box::new(right),

};

}

Ok(expr)

}

fn unary(&mut self) -> Result<Expr, String> {

if self.match_token(&Token::Menos) {

return Ok(

Expr::UnOp {

op: Token::Menos,
expr: Box::new(self.unary()?),

}

);

}

if self.match_token(&Token::NAO) {

return Ok(

Expr::UnOp {

op: Token::NAO,
expr: Box::new(self.unary()?),

}

);

}

self.call()

}

fn call(&mut self) -> Result<Expr, String> {

self.primary()

}

fn primary(&mut self) -> Result<Expr, String> {

    match self.advance() {
        Token::Numero(n) => Ok(Expr::Numero(n)),

        Token::Texto(s) => Ok(Expr::Texto(s)),

        Token::Verdadeiro => Ok(Expr::Booleano(true)),

        Token::Falso => Ok(Expr::Booleano(false)),

        Token::Ident(nome) => {
            if self.match_token(&Token::AbrePar) {
                let mut args = Vec::new();

                if !self.check(&Token::FechaPar) {
                    loop {
                        args.push(self.expression()?);

                        if !self.match_token(&Token::Virgula) {
                            break;
                        }
                    }
                }

                self.consumir(
                    &Token::FechaPar,
                    "Esperado ')' após argumentos.",
                )?;

                return Ok(Expr::Chamada { nome, args });
            }

            Ok(Expr::Ident(nome))
        }

        Token::AbrePar => {
            let expr = self.expression()?;

            self.consumir(
                &Token::FechaPar,
                "Esperado ')' após expressão.",
            )?;

            Ok(expr)
        }

        token => Err(format!("Expressão inesperada: {:?}", token)),
    }
}

fn block(&mut self) -> Result<Vec<Stmt>, String> {

self.consumir(
&Token::AbreChave,
"Esperado '{'."
)?;

let mut corpo = Vec::new();

while !self.check(&Token::FechaChave)
&& !self.acabou()
{

corpo.push(self.statement()?);

}

self.consumir(
&Token::FechaChave,
"Esperado '}'."
)?;

Ok(corpo)

}

fn if_statement(&mut self) -> Result<Stmt, String> {

let cond = self.expression()?;

let corpo = self.block()?;

let senao = if self.match_token(&Token::Senao) {

self.block()?

} else {

Vec::new()

};

Ok(

Stmt::Se {

cond,      
corpo,      
senao,

}

)

}

fn while_statement(&mut self) -> Result<Stmt, String> {

let cond = self.expression()?;

let corpo = self.block()?;

Ok(

Stmt::Enquanto {

cond,      
corpo,

}

)

}

fn function_statement(&mut self) -> Result<Stmt, String> {

let nome = match self.advance() {

Token::Ident(n) => n,

_ => {

return Err(      
    "Esperado nome da função.".to_string()      
);

}

};

self.consumir(
&Token::AbrePar,
"Esperado '(' após nome da função."
)?;

let mut params = Vec::new();

if !self.check(&Token::FechaPar) {

loop {

match self.advance() {      

    Token::Ident(nome) => params.push(nome),      

    _ => {      

        return Err(      
            "Esperado nome do parâmetro."      
                .to_string()      
        );      

    }      

}      

if !self.match_token(&Token::Virgula) {      
    break;      
}

}

}

self.consumir(
&Token::FechaPar,
"Esperado ')'."
)?;

let corpo = self.block()?;

Ok(

Stmt::Funcao {

nome,      
params,      
corpo,

}

)

}

}
