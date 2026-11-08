
// Estrutura que representa um usuario.
pub struct Usuario {
    pub nome: String,
    pub uid: u16,
    // Vetor armazenando os GIDs dos grupos do usuario.
    pub grupos_ids: Vec<u16>,
}

impl Usuario {
    // Aqui vai a implementação de Usuario::new
    pub fn new(nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupos_ids: Vec::new()
        }
    }

    // Aqui vai a implementação de Usuario::adiciona_grupo
    pub fn adiciona_grupo(&mut self, gid: u16) {
        self.grupos_ids.push(gid);
    }

    // Aqui vai a implementação de Usuario::remove_grupo
    pub fn remove_grupo(&mut self, gid: u16) {
        self.grupos_ids.retain(|&g| g != gid);
    }

    // Aqui vai a implementação de Usuario::listar_grupos
    pub fn listar_grupos(&self) {
        for gid in &self.grupos_ids {
            println!("Grupo: {}", gid);
    } 
    }
}
