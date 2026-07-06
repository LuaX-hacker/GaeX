use std::collections::HashMap;

use crate::{
lexer::Token,
parser::{
Expr,
Stmt,
},
};

#[derive(Debug, Clone)]
pub enum Value {

Numero(f64),

Texto(String),

Booleano(bool),

Nulo,

}

#[derive(Debug, Clone)]
pub struct Funcao {
pub params: Vec<String>,
pub corpo: Vec<Stmt>,
}

pub struct Interpreter {
variaveis: Vec<HashMap<String, Value>>,
funcoes: HashMap<String, Funcao>,
}

impl Interpreter {

pub fn new() -> Self {

Self {
variaveis: vec![HashMap::new()],
funcoes: HashMap::new(),
}

}

fn entrar_escopo(&mut self) {
self.variaveis.push(HashMap::new());
}

fn sair_escopo(&mut self) {
if self.variaveis.len() > 1 {
self.variaveis.pop();
}
}

fn definir_variavel(&mut self, nome: String, valor: Value) {
self.variaveis
.last_mut()
.unwrap()
.insert(nome, valor);
}

fn obter_variavel(&self, nome: &str) -> Option<Value> {
for escopo in self.variaveis.iter().rev() {
if let Some(v) = escopo.get(nome) {
return Some(v.clone());
}
}
None
}

pub fn interpretar(
&mut self,
stmts: &[Stmt],
) -> Result<(), String> {

for stmt in stmts {

let _ = self.executar(stmt)?;

}

Ok(())

}

fn executar(
&mut self,
stmt: &Stmt,
) -> Result<Option<Value>, String> {

match stmt {

Stmt::Expr(expr) => {

self.avaliar(expr)?;
Ok(None)

}

Stmt::Se {

cond,
corpo,
senao,

} => {

match self.avaliar(cond)? {

Value::Booleano(true) => {
self.entrar_escopo();

for stmt in corpo {
if let Some(valor) = self.executar(stmt)? {
self.sair_escopo();
return Ok(Some(valor));
}
}

self.sair_escopo();

}

Value::Booleano(false) => {
self.entrar_escopo();

for stmt in senao {
if let Some(valor) = self.executar(stmt)? {
self.sair_escopo();
return Ok(Some(valor));
}
}

self.sair_escopo();

}

_ => {
return Err("A condição do 'se' deve ser um booleano.".into());
}

}

Ok(None)

}

Stmt::Enquanto {

cond,
corpo,

} => {

loop {
match self.avaliar(cond)? {

Value::Booleano(true) => {

self.entrar_escopo();

for stmt in corpo {
if let Some(valor) = self.executar(stmt)? {
self.sair_escopo();
return Ok(Some(valor));
}
}

self.sair_escopo();

}

Value::Booleano(false) => break,

_ => {
return Err("A condição do 'enquanto' deve ser um booleano.".into());
}

}

}

Ok(None)

}

Stmt::Funcao {
nome,
params,
corpo,
} => {
self.funcoes.insert(
nome.clone(),
Funcao {
params: params.clone(),
corpo: corpo.clone(),
},
);

Ok(None)

}

Stmt::Retorna(expr) => {

let valor = self.avaliar(expr)?;
Ok(Some(valor))

}

Stmt::Empty => Ok(None),

}

}

fn avaliar(
&mut self,
expr: &Expr,
) -> Result<Value, String> {

match expr {

Expr::Numero(n) => {

Ok(Value::Numero(*n))

}

Expr::Texto(s) => {

Ok(Value::Texto(s.clone()))

}

Expr::Booleano(b) => {

Ok(Value::Booleano(*b))

}

Expr::Ident(nome) => {
match self.obter_variavel(nome) {
Some(valor) => Ok(valor.clone()),
None => Err(format!("Variável '{}' não existe.", nome)),
}
},

Expr::Atribuicao {

nome,
valor,

} => {

let valor = self.avaliar(valor)?;

self.atribuir_variavel(nome, valor.clone());

Ok(valor)

},

Expr::UnOp {

op,
expr,

} => {

let valor = self.avaliar(expr)?;

self.avaliar_unario(
op,
valor,
)

},

Expr::BinOp {

left,
op,
right,

} => {

let esquerdo =
self.avaliar(left)?;

let direito =
self.avaliar(right)?;

self.avaliar_binario(
esquerdo,
op,
direito,
)

},

Expr::Chamada {

nome,
args,

} => {

self.chamar_funcao(
nome,
args,
)

}

}

}

fn avaliar_binario(
&self,
esquerdo: Value,
op: &Token,
direito: Value,
) -> Result<Value, String> {

match op {

// ------------------------
// Soma
// ------------------------

Token::Mais => {

match (esquerdo, direito) {

(Value::Numero(a), Value::Numero(b)) => {

Ok(Value::Numero(a + b))

}

(Value::Texto(a), Value::Texto(b)) => {

Ok(Value::Texto(a + &b))

}

(Value::Texto(a), Value::Numero(b)) => {

Ok(Value::Texto(a + &b.to_string()))

}

(Value::Numero(a), Value::Texto(b)) => {

Ok(Value::Texto(a.to_string() + &b))

}

_ => Err(
"Tipos incompatíveis para '+'.".to_string()
),

}

}

// ------------------------
// Subtração
// ------------------------

Token::Menos => {

match (esquerdo, direito) {

(Value::Numero(a), Value::Numero(b)) => {

Ok(Value::Numero(a - b))

}

_ => Err(
"Tipos incompatíveis para '-'.".to_string()
),

}

}

// ------------------------
// Multiplicação
// ------------------------

Token::Asterisco => {

match (esquerdo, direito) {

(Value::Numero(a), Value::Numero(b)) => {

Ok(Value::Numero(a * b))

}

_ => Err(
"Tipos incompatíveis para '*'.".to_string()
),

}

}

// ------------------------
// Divisão
// ------------------------

Token::Barra => {

match (esquerdo, direito) {

(Value::Numero(_), Value::Numero(0.0)) => {

Err(
"Divisão por zero.".to_string()
)

}

(Value::Numero(a), Value::Numero(b)) => {

Ok(Value::Numero(a / b))

}

_ => Err(
"Tipos incompatíveis para '/'.".to_string()
),

}

}

// ------------------------
// Porcentagem
// ------------------------

Token::Porcentagem => {
match (esquerdo, direito) {
(Value::Numero(_), Value::Numero(0.0)) => {
Err("Divisão por zero.".into())
}

(Value::Numero(a), Value::Numero(b)) => {
Ok(Value::Numero(a % b))
}

_ => Err("Tipos incompatíveis para '%'.".to_string()),

}

}

// ------------------------
// Igual
// ------------------------

Token::IgualIgual => {

Ok(

Value::Booleano(

match (esquerdo, direito) {

(Value::Numero(a), Value::Numero(b))
=> a == b,

(Value::Texto(a), Value::Texto(b))
=> a == b,

(Value::Booleano(a), Value::Booleano(b))
=> a == b,

_ => false,

}

)

)

}

// ------------------------
// Diferente
// ------------------------

Token::Diferente => {

match self.avaliar_binario(
esquerdo,
&Token::IgualIgual,
direito,
)? {

Value::Booleano(b) => {

Ok(Value::Booleano(!b))

}

_ => unreachable!(),

}

}

// ------------------------
// Maior
// ------------------------

Token::Maior => {

match (esquerdo, direito) {

(Value::Numero(a), Value::Numero(b)) => {

Ok(Value::Booleano(a > b))

}

_ => Err(
"Tipos incompatíveis para '>'.".to_string()
),

}

}

// ------------------------
// Menor
// ------------------------

Token::Menor => {

match (esquerdo, direito) {

(Value::Numero(a), Value::Numero(b)) => {

Ok(Value::Booleano(a < b))

}

_ => Err(
"Tipos incompatíveis para '<'.".to_string()
),

}

}

// ------------------------
// Maior ou igual
// ------------------------

Token::MaiorIgual => {

match (esquerdo, direito) {

(Value::Numero(a), Value::Numero(b)) => {

Ok(Value::Booleano(a >= b))

}

_ => Err(
"Tipos incompatíveis.".to_string()
),

}

}

// ------------------------
// Menor ou igual
// ------------------------

Token::MenorIgual => {

match (esquerdo, direito) {

(Value::Numero(a), Value::Numero(b)) => {

Ok(Value::Booleano(a <= b))

}

_ => Err(
"Tipos incompatíveis.".to_string()
),

}

}

// ------------------------
// E
// ------------------------

Token::E => {

match (esquerdo, direito) {

(
Value::Booleano(a),
Value::Booleano(b),
) => {

Ok(Value::Booleano(a && b))

}

_ => Err(
"'e' espera booleanos.".to_string()
),

}

}

// ------------------------
// OU
// ------------------------

Token::OU => {

match (esquerdo, direito) {

(
Value::Booleano(a),
Value::Booleano(b),
) => {

Ok(Value::Booleano(a || b))

}

_ => Err(
"'ou' espera booleanos.".to_string()
),

}

}

_ => Err(
"Operador não implementado.".to_string()
),

}

}

fn avaliar_unario(
&self,
op: &Token,
valor: Value,
) -> Result<Value, String> {

match op {

Token::Menos => {

match valor {

Value::Numero(n) => {
Ok(Value::Numero(-n))
}

_ => Err(
"Operador '-' espera um número."
.to_string()
),

}

}

Token::NAO => {

match valor {

Value::Booleano(b) => {
Ok(Value::Booleano(!b))
}

_ => Err(
"Operador 'nao' espera um booleano."
.to_string()
),

}

}

_ => Err(
"Operador unário inválido."
.to_string()
),

}

}

fn chamar_funcao(
&mut self,
nome: &str,
args: &[Expr],
) -> Result<Value, String> {

// ------------------------
// Funções nativas
// ------------------------

if nome == "mostra" {

    if args.len() != 1 {
        return Err("mostra() espera exatamente 1 argumento.".into());
    }

    let valor = self.avaliar(&args[0])?;

    match &valor {
        Value::Numero(n) => println!("{}", n),
        Value::Texto(s) => println!("{}", s),
        Value::Booleano(b) => println!("{}", b),
        Value::Nulo => println!("nulo"),
    }

    return Ok(Value::Nulo);
}

let funcao = self
.funcoes
.get(nome)
.cloned()
.ok_or(format!("Função '{}' não existe.", nome))?;

if args.len() != funcao.params.len() {
return Err(format!(
"Esperava {} argumentos, recebeu {}.",
funcao.params.len(),
args.len()
));
}

let mut valores = Vec::new();

for arg in args {
valores.push(self.avaliar(arg)?);
}

self.entrar_escopo();

for (param, valor) in funcao.params.iter().zip(valores) {
self.definir_variavel(param.clone(), valor);
}

let mut retorno = Value::Nulo;

for stmt in &funcao.corpo {
    if let Some(valor) = self.executar(stmt)? {
        retorno = valor;
        break;
    }
}

self.sair_escopo();

Ok(retorno)
}

fn atribuir_variavel(&mut self, nome: &str, valor: Value) {
for escopo in self.variaveis.iter_mut().rev() {
if escopo.contains_key(nome) {
escopo.insert(nome.to_string(), valor);
return;
}
}

self.definir_variavel(nome.to_string(), valor);

}

}
