use std::fmt;

#[derive(Debug, Clone)]
pub enum TipoErro {
    Lexico,
    Sintaxe,
    Execucao,
}

#[derive(Debug, Clone)]
pub struct GaeXError {
    pub tipo: TipoErro,
    pub mensagem: String,
    pub linha: usize,
    pub coluna: usize,
}

impl GaeXError {
    pub fn novo(
        tipo: TipoErro,
        mensagem: impl Into<String>,
        linha: usize,
        coluna: usize,
    ) -> Self {
        Self {
            tipo,
            mensagem: mensagem.into(),
            linha,
            coluna,
        }
    }
}

impl fmt::Display for GaeXError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tipo = match self.tipo {
            TipoErro::Lexico => "Erro Léxico",
            TipoErro::Sintaxe => "Erro de Sintaxe",
            TipoErro::Execucao => "Erro de Execução",
        };

        write!(
            f,
            "{}\nLinha {}, Coluna {}\n{}",
            tipo,
            self.linha,
            self.coluna,
            self.mensagem
        )
    }
}

impl std::error::Error for GaeXError {}

