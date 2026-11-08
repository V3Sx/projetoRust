use crate::arquivo::Arquivo;
use crate::permissao::Permissao;
// Estrutura que representa um diretorio.
pub struct Diretorio {
    pub nome: String,
    pub arquivos: Vec<Arquivo>,
    pub permissoes: (Permissao, Permissao, Permissao),
    pub uid_dono: u16,
}

impl Diretorio {
    // Aqui vai a implementação de Diretorio::new
    pub fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Diretorio {
        Diretorio {
            nome,
            arquivos: Vec::new(),
            permissoes,
            uid_dono,
        }
    }


    // Aqui vai a implementação de Diretorio::adiciona_arquivo
    pub fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    // Aqui vai a implementação de Diretorio::remove_arquivo
    pub fn remove_arquivo(&mut self, nome_arquivo: String) {
        self.arquivos.retain(|a| a.nome != nome_arquivo);
    }

    // Aqui vai a implementação de Diretorio::listar_conteudo
    pub fn listar_conteudo(&self) {
        for arquivo in &self.arquivos {
            println!("{}", arquivo.nome);
        }
    }
}
