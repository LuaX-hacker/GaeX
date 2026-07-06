#[derive(Debug, Clone)]
pub enum Expr {
    Diga(String),

    Numero(f64),
    Variavel(String),

    Atribuir(String, Box<Expr>),

    BinOp {
        op: Operador,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Operador {
    Soma,
    Sub,
    Mul,
    Div,
}
