
//Projeto Colaborativo RA1 e RA2
//Professor: Pedro Rocha Horchulhac
//Materia: Programação Imperativa
//Estudantes: Gustavo Fronza, Matheus Sommer 
fn input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    input.trim().to_string()
}
fn input_int(prompt: &str) -> u16 {
    loop {
        let texto = input(prompt);
        match texto.parse::<u16>() {
            Ok(n) => return n,
            Err(_) => println!("Valor inválido. Digite um número entre 0 e 65535."),
        }
    }
}   

fn menu_interativo() -> bool {
    println!();
    println!("1. Criar estrutura");
    println!("2. Editar estrutura");
    println!("3. Apagar estrutura");
    println!("4. Listar estruturas");
    println!("5. Sair");
 
    let mut escolha = String::new();
    match std::io::stdin().read_line(&mut escolha) {
        Ok(0) => {
            println!("Saindo...");
            return false;
        }
        Ok(_) => {}
        Err(erro) => {
            println!("Falha ao ler a entrada: {erro}");
            return false;
        }
    }
 
    match escolha.trim() {
        "1" => criar_estrutura(),
        "2" => editar_estrutura(),
        "3" => apagar_estrutura(),
        "4" => listar_estruturas(),
        "5" => {
            println!("Saindo...");
            return false;
        }
        _ => println!("Opção inválida!"),
    }
    true
}

struct Sistema {
    usuarios: Vec<Usuario>,
    grupos: Vec<Grupo>,
    diretorios: Vec<Diretorio>,
}

impl Sistema{
    fn new () -> Sistema {
        Sistema
        {
            usuarios: Vec::new(),
            grupos: Vec::new(),
            diretorios: Vec::new(),
        }

    }
}

struct Permissao {
    usuario: [bool; 3],
    grupo: [bool; 3],
    outros: [bool; 3],
}

impl Permissao {
    fn new(usuario: [bool; 3], grupo: [bool; 3], outros: [bool; 3]) -> Permissao {
        Permissao{
            usuario,
            grupo,
            outros,
        }
    }
    fn octal(&self) -> u16 {
        let mut valor_usuario = 0;
        if self.usuario[0] { valor_usuario += 4; }
        if self.usuario[1] { valor_usuario += 2; }
        if self.usuario[2] { valor_usuario += 1; }

        let mut valor_grupo = 0;
        if self.grupo[0] { valor_grupo += 4; }
        if self.grupo[1] { valor_grupo += 2; }
        if self.grupo[2] { valor_grupo += 1; }

        let mut valor_outros = 0;
        if self.outros[0] { valor_outros += 4; }
        if self.outros[1] { valor_outros += 2; }
        if self.outros[2] { valor_outros += 1; }

        valor_usuario * 100 + valor_grupo * 10 + valor_outros
}    }

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
    permissoes: Permissao,
    uid_dono: u16
}

impl Diretorio{
    fn new(nome: String, permissoes: Permissao, uid_dono: u16) -> Diretorio {
        Diretorio {
            nome,
            arquivos: Vec::new(),
            permissoes,
            uid_dono
        }
    }
    fn adiciona_arquivo(&mut self, gid_grupo: u16, arquivo: Arquivo) {
        if arquivo.gid_grupo == gid_grupo {
            self.arquivos.push(arquivo);
            println!("Arquivo adicionado com sucesso.");
        } else {
            println!("Erro: O GID do arquivo não corresponde ao GID do diretório.");
        }
    }
    fn remover_arquivo(&mut self, nome_arquivo: &str) {
        if let Some(pos) = self.arquivos.iter().position(|x| x.nome == nome_arquivo) {
            self.arquivos.remove(pos);
            println!("Arquivo '{}' removido com sucesso.", nome_arquivo);
        } else {
            println!("Arquivo '{}' não encontrado.", nome_arquivo);
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

fn criar_estrutura(sistema: &mut Sistema ) {
    println!("1. usuário    2. grupo    3. diretório    4. arquivo");
    let mut escolha = String::new();
    match std::io::stdin().read_line(&mut escolha) {
        Ok(_) => {}
        Err(erro) => {
            println!("Falha ao ler a entrada: {erro}");
            return;
        }
    }
    match escolha.trim() {
        "1" => 
        {
            let nome = input("Digite um nome do Usuário: ");
            let uid = input_int("Digite o UID do Usuário: ");

        if Sistema.usuarios.iter().any(|u| u.uid == uid) {
            println!("Erro: Já existe um usuário com o mesmo UID.");
            return;
            }

            Sistema.usuarios.push(Usuario::new(nome, uid));
            println!("Usuário criado com sucesso.");
        }

        "2" =>
        {
            let nome = input("Nome do Grupo: ");
            let gid = input_int("Digite um GID: ");
            
        }

    }
}
fn editar_estrutura() {

}
fn apagar_estrutura() {

} 
fn listar_estruturas() {

}

fn main() {
    let mut sistema = Sistema::new();
    while menu_interativo() {}
}