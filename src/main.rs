
//Projeto Colaborativo RA1 e RA2
//Professor: Pedro Rocha Horchulhac
//Materia: Programação Imperativa
//Estudantes: Gustavo Fronza, Matheus Sommer 
fn menu_interativo() {
    println!("1. Criar estrutura");
    println!("2. Editar  estrutura");
    println!("3. Apagar estrutura");
    println!("4. Listar estruturas");
    println!("5. Sair");

    let mut escolha = String::new();
    std::io::stdin().read_line(&mut escolha).expect("Falha ao ler a entrada");
    
    match escolha.trim() {
        "1" => criar_estrutura(),
        "2" => editar_estrutura(),
        "3" => apagar_estrutura(),
        "4" => listar_estruturas(),
        "5" => {
            println!("Saindo...");
            return;
        },
        _ => println!("Opção inválida!"),
    }
}

struct Permissao {
    usuario: [bool; 3],
    grupo: [bool; 3],
    outros: [bool; 3],
}

impl Permissao {
    fn new(leitura_usuario: bool, escrita_usuario: bool, execucao_usuario: bool,
           leitura_grupo: bool, escrita_grupo: bool, execucao_grupo: bool,
           leitura_outros: bool, escrita_outros: bool, execucao_outros: bool) -> Permissao {
        Permissao {
            usuario: [leitura_usuario, escrita_usuario, execucao_usuario],
            grupo: [leitura_grupo, escrita_grupo, execucao_grupo],
            outros: [leitura_outros, escrita_outros, execucao_outros],
        }
    }
    fn octal(&self) -> u8 {
        let mut octal = 0;
        for (i, &perm) in self.usuario.iter().enumerate() {
            if perm {
                octal += 1 << (2 - i);
            }
        }
        for (i, &perm) in self.grupo.iter().enumerate() {
            if perm {
                octal += 1 << (5 - i);
            }
        }
        for (i, &perm) in self.outros.iter().enumerate() {
            if perm {
                octal += 1 << (8 - i);
            }
        }
        octal
    }
}
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: Permissao,
    uid_dono: u16,
    gid_grupo: u16,
}
impl Arquivo {
    fn new(nome: String, tamanho: u64, permissoes: Permissao, uid_dono: u16, gid_grupo: u16) -> Arquivo {
        Arquivo {
            nome,
            tamanho,
            permissoes,
            uid_dono,
            gid_grupo,
        }
    }
    fn alterar_permissoes(&mut self, novas_permissoes: Permissao) {
        self.permissoes = novas_permissoes;
    }
    fn stat(&self) {
        println!("Nome: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!("UID do dono: {}", self.uid_dono);
        println!("GID do grupo: {}", self.gid_grupo);
        println!("Permissões:");
        println!("  Usuário - Leitura: {}, Escrita: {}, Execução: {}",
                 self.permissoes.usuario[0], self.permissoes.usuario[1], self.permissoes.usuario[2]);
        println!("  Grupo - Leitura: {}, Escrita: {}, Execução: {}",
                 self.permissoes.grupo[0], self.permissoes.grupo[1], self.permissoes.grupo[2]);
        println!("  Outros - Leitura: {}, Escrita: {}, Execução: {}",
                 self.permissoes.outros[0], self.permissoes.outros[1], self.permissoes.outros[2]);
    }
}


struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16
}

impl Diretorio{
    fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Diretorio {
        Diretorio {
            nome,
            arquivos: Vec::new(),
            permissoes,
            uid_dono
        }
    }
    fn adiciona_arquivo(&mut self, gid_grupo: u16, arquivo: Arquivo) {
        if self.uid_dono == arquivo.uid_dono && self.permissoes.0[1] {
            self.arquivos.push(arquivo);
            println!("Arquivo adicionado com sucesso.");
        } else if self.permissoes.1[1] && self.uid_dono != arquivo.uid_dono {
            self.arquivos.push(arquivo);
            println!("Arquivo adicionado com sucesso.");
        } else {
            println!("Permissão negada para adicionar arquivo.");
        }
    }
    fn remover_arquivo(&mut self, nome_arquivo: &str) {
        if let Some(pos) = self.arquivos.iter().position(|x| x.nome == nome_arquivo) {
            if self.uid_dono == self.arquivos[pos].uid_dono && self.permissoes.0[1] {
                self.arquivos.remove(pos);
                println!("Arquivo removido com sucesso.");
            } else if self.permissoes.1[1] && self.uid_dono != self.arquivos[pos].uid_dono {
                self.arquivos.remove(pos);
                println!("Arquivo removido com sucesso.");
            } else {
                println!("Permissão negada para remover arquivo.");
            }
        } else {
            println!("Arquivo não encontrado.");
        }
    }
    fn listar_arquivos(&self) {
        println!("Arquivos no diretório '{}':", self.nome);
        for arquivo in &self.arquivos {
            arquivo.stat();
        }
    }
}

struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>
}
impl Usuario{
    fn new(nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupos_ids: Vec::new()
        }
    }
    fn adicionar_grupo(&mut self, gid: u16) {
        if !self.grupos_ids.contains(&gid) {
            self.grupos_ids.push(gid);
            println!("Grupo adicionado ao usuário com sucesso.");
        } else {
            println!("Usuário já pertence a este grupo.");
        }
    }
    fn remover_grupo(&mut self, gid: u16) {
        if let Some(pos) = self.grupos_ids.iter().position(|&x| x == gid) {
            self.grupos_ids.remove(pos);
            println!("Grupo removido do usuário com sucesso.");
        } else {
            println!("Usuário não pertence a este grupo.");
        }
    }
    fn listar_grupos(&self) {
        println!("Grupos do usuário '{}':", self.nome);
        for gid in &self.grupos_ids {
            println!("GID: {}", gid);
        }
    }
}

struct Grupo {
    nome: String,
    gid: u16,
    usuarios_ids: Vec<u16>
}
impl Grupo{
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            usuarios_ids: Vec::new()
        }
    }
    fn adicionar_membro(&mut self, uid: u16) {
        if !self.usuarios_ids.contains(&uid) {
            self.usuarios_ids.push(uid);
            println!("Usuário adicionado ao grupo com sucesso.");
        } else {
            println!("Usuário já pertence a este grupo.");
        }
    }
    fn remover_membro(&mut self, uid: u16) {
        if let Some(pos) = self.usuarios_ids.iter().position(|&x| x == uid) {
            self.usuarios_ids.remove(pos);
            println!("Usuário removido do grupo com sucesso.");
        } else {
            println!("Usuário não pertence a este grupo.");
        }
    }
    fn listar_membros(&self) {
        println!("Membros do grupo '{}':", self.nome);
        for uid in &self.usuarios_ids {
            println!("UID: {}", uid);
        }
    }
}

fn criar_estrutura() {
    println!("Função para criar estrutura ainda não implementada.");
}
fn editar_estrutura() {
    println!("Função para editar estrutura ainda não implementada.");
}
fn apagar_estrutura() {
    println!("Função para apagar estrutura ainda não implementada.");
} 
fn listar_estruturas() {
    println!("Função para listar estruturas ainda não implementada.");
}

fn main() {
    while True {
        menu_interativo();
    }
}