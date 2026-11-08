use crate::permissao::Permissao;

// struct que representa o arquivo
pub struct Arquivo {
    pub nome: String,
    pub tamanho: u64,
    // permissao, dono e grupo
    pub permissoes: (bool, bool, bool),
    pub uid_dono: u16, //user id
    pub gid_grupo: u16, //group id
}

impl Arquivo {
    //construtor de novo
    // Aqui vai a implementação de Arquivo::new
    pub fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        Arquivo{
            nome,
            tamanho,
            permissoes: (false, false, false),
            uid_dono: uid,
            gid_grupo: gid,
        }
    }

    // impl para alterar as permissoes
    pub fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = (
            permissoes.0.octal() != 0,
            permissoes.1.octal() != 0,
            permissoes.2.octal() != 0,
        );
    }

    // arquivo stat, info dos arquivos
    pub fn stat(&self) {
        println!("Nome: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!("UID do dono: {}", self.uid_dono);
        println!("GID do grupo: {}", self.gid_grupo);
        println!("Permissoes: {:?}", self.permissoes); //self.permissoes eh uma tupla, :? serve pra imprimir o conteudo diretamente
    }
}
