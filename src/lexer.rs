#[derive(Debug, Clone, PartialEq)]
pub enum Token {
// Literais
Numero(f64),
Texto(String),
Ident(String),

// Operadores
Mais,
Menos,
Asterisco,
Barra,
Porcentagem,

Igual,
IgualIgual,
Diferente,

Maior,
Menor,
MaiorIgual,
MenorIgual,

E,
OU,
NAO,

// Delimitadores
AbrePar,
FechaPar,

AbreChave,
FechaChave,

Virgula,
PontoVirgula,

// Palavras-chave
Se,
Senao,
Enquanto,

Funcao,
Retorna,

Verdadeiro,
Falso,

// Fim
EOF,

}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub linha: usize,
    pub coluna: usize,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    linha: usize,
    coluna: usize,
}

impl Lexer {

pub fn new(input: &str) -> Self {
    Self {
        chars: input.chars().collect(),
        pos: 0,
        linha: 1,
        coluna: 1,
    }
}

fn acabou(&self) -> bool {
    self.pos >= self.chars.len()
}

fn peek(&self) -> Option<char> {
self.chars.get(self.pos).copied()
}

fn peek_next(&self) -> Option<char> {
    self.chars.get(self.pos + 1).copied()
}

fn advance(&mut self) -> Option<char> {

if self.acabou() {
return None;
}

let c = self.chars[self.pos];

self.pos += 1;

if c == '\n' {
self.linha += 1;
self.coluna = 1;
} else {
self.coluna += 1;
}

Some(c)

}

fn match_char(&mut self, esperado: char) -> bool {

if self.peek() == Some(esperado) {  
self.advance();  
true

}else {
false
}

}

fn ignorar_espacos(&mut self) {

loop {    

    match self.peek() {    

        Some(' ')    
        | Some('\t')    
        | Some('\r')    
        | Some('\n') => {    
            self.advance();    
        }    

        _ => break,    

    }    

}

}

fn ler_numero(&mut self) -> Token {

let mut numero = String::new();  
let mut ponto = false;  

while let Some(c) = self.peek() {  

    if c.is_ascii_digit() {  

        numero.push(c);  
        self.advance();  

    } else if c == '.' && !ponto {  

        ponto = true;  
        numero.push(c);  
        self.advance();  

    } else {  

        break;  

    }  

}  

Token::Numero(numero.parse::<f64>().unwrap())

}

fn ler_identificador(&mut self) -> Token {

let mut ident = String::new();    

while let Some(c) = self.peek() {    

    if c.is_ascii_alphanumeric() || c == '_' {    

        ident.push(c);    
        self.advance();    

    } else {    
        break;    
    }    

}    

match ident.as_str() {    

    "se" => Token::Se,    
    "senao" => Token::Senao,    
    "enquanto" => Token::Enquanto,    

    "funcao" => Token::Funcao,    
    "retorna" => Token::Retorna,    

    "verdadeiro" => Token::Verdadeiro,    
    "falso" => Token::Falso,    

    "e" => Token::E,    
    "ou" => Token::OU,    
    "nao" => Token::NAO,    

    _ => Token::Ident(ident),    

}

}

fn ler_string(&mut self) -> Result<Token, String> {

self.advance();

let mut texto = String::new();

while let Some(c) = self.peek() {

match c {    

    '"' => {    
        self.advance();    
        return Ok(Token::Texto(texto));    
    }    

    '\\' => {    

        self.advance();    

        match self.peek() {    

            Some('n') => {    
                texto.push('\n');    
                self.advance();    
            }    

            Some('t') => {    
                texto.push('\t');    
                self.advance();    
            }    

            Some('"') => {    
                texto.push('"');    
                self.advance();    
            }    

            Some('\\') => {    
                texto.push('\\');    
                self.advance();    
            }    

            Some(outro) => {    
                return Err(format!(    
                    "Escape inválido '\\{}'",    
                    outro    
                ));    
            }    

            None => {    
                return Err(    
                    "String terminou após '\\'".to_string()    
                );    
            }    

        }    

    }    

    _ => {    

        texto.push(c);    
        self.advance();    

    }    

}

}

Err(format!(
"String não finalizada (linha {})",
self.linha
))

}

pub fn tokenize(&mut self) -> Result<Vec<TokenInfo>, String> {

let mut tokens = Vec::new();    

while !self.acabou() {    

    self.ignorar_espacos();    

    if self.acabou() {    
        break;    
    }    

    let linha = self.linha;
    let coluna = self.coluna;

    let c = self.peek().unwrap();    

    let token = match c {    

        // -------------------------    
        // Números    
        // -------------------------    
        '0'..='9' => self.ler_numero(),    

        // -------------------------    
        // Identificadores    
        // -------------------------    
        'a'..='z'    
        | 'A'..='Z'    
        | '_' => self.ler_identificador(),    

        // -------------------------    
        // Strings    
        // -------------------------    
        '"' => self.ler_string()?,    

        // -------------------------    
        // Operadores matemáticos    
        // -------------------------    
        '+' => {    
            self.advance();    
            Token::Mais    
        }    

        '-' => {    
            self.advance();    
            Token::Menos    
        }    

        '*' => {    
            self.advance();    
            Token::Asterisco    
        }    

        '%' => {    
            self.advance();    
            Token::Porcentagem    
        }    

        // -------------------------    
        // Barra ou comentário    
        // -------------------------    
        '/' => {    

            self.advance();    

            if self.match_char('/') {    

                while let Some(ch) = self.peek() {    

                    if ch == '\n' {    
                        break;    
                    }    

                    self.advance();    

                }    

                continue;    

            }    

            if self.match_char('*') {    

                loop {    

                    if self.acabou() {    
                        return Err(    
                            "Comentário de bloco não fechado."    
                                .to_string()    
                        );    
                    }    

                    if self.peek() == Some('*')    
                        && self.peek_next() == Some('/')    
                    {    

                        self.advance();    
                        self.advance();    

                        break;    
                    }    

                    self.advance();    

                }    

                continue;    

            }    

            Token::Barra    

        }    

        // -------------------------    
        // Igual    
        // -------------------------    
        '=' => {    

            self.advance();    

            if self.match_char('=') {    
                Token::IgualIgual    
            } else {    
                Token::Igual    
            }    

        }    

        // -------------------------    
        // Diferente    
        // -------------------------    
        '!' => {    

            self.advance();    

            if self.match_char('=') {    
                Token::Diferente    
            } else {    
                Token::NAO    
            }    

        }    

        // -------------------------    
        // Maior    
        // -------------------------    
        '>' => {    

            self.advance();    

            if self.match_char('=') {    
                Token::MaiorIgual    
            } else {    
                Token::Maior    
            }    

        }    

        // -------------------------    
        // Menor    
        // -------------------------    
        '<' => {    

            self.advance();    

            if self.match_char('=') {    
                Token::MenorIgual    
            } else {    
                Token::Menor    
            }    

        }    

        // -------------------------    
        // Delimitadores    
        // -------------------------    
        '(' => {    
            self.advance();    
            Token::AbrePar    
        }    

        ')' => {    
            self.advance();    
            Token::FechaPar    
        }    

        '{' => {    
            self.advance();    
            Token::AbreChave    
        }    

        '}' => {    
            self.advance();    
            Token::FechaChave    
        }    

        ',' => {    
            self.advance();    
            Token::Virgula    
        }    

        ';' => {    
            self.advance();    
            Token::PontoVirgula    
        }    

        // -------------------------    
        // Erro    
        // -------------------------    
        _ => {    

            return Err(format!(

"Erro léxico\nLinha: {}\nColuna: {}\nCaractere inesperado '{}'",
self.linha,
self.coluna,
c

));

}

};    

    tokens.push(TokenInfo {
    token,
    linha: self.linha,
    coluna: self.coluna,
});

}    

tokens.push(TokenInfo {
    token: Token::EOF,
    linha: self.linha,
    coluna: self.coluna,
});

Ok(tokens)

}

}
