use std::io::{self, Write};
fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Falha ao mostrar o prompt");

    let mut texto = String::new();
    let bytes = io::stdin()
        .read_line(&mut texto)
        .expect("Falha ao ler a entrada");

    if bytes == 0 {
        std::process::exit(0);
    }

    texto.trim().to_string()
}

fn input_int(prompt: &str) -> u16 {
    loop {
        match input(prompt).parse::<u16>() {
            Ok(numero) => return numero,
            Err(_) => println!("Digite um número entre 0 e 65535."),
        }
    }
}

struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>,
}

impl Usuario {
    fn new(nome: String, uid: u16) -> Self {
        Self {
            nome,
            uid,
            grupos_ids: Vec::new(),
        }
    }
}

struct Grupo {
    nome: String,
    gid: u16,
    membros_uids: Vec<u16>,
}

impl Grupo {
    fn new(nome: String, gid: u16) -> Self {
        Self {
            nome,
            gid,
            membros_uids: Vec::new(),
        }
    }
}

struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}

impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Self {
        Self {
            leitura,
            escrita,
            execucao,
        }
    }

    fn octal(&self) -> u8 {
        (self.leitura as u8) * 4
            + (self.escrita as u8) * 2
            + (self.execucao as u8)
    }

    fn ler_permissao(para_quem: &str) -> Self {
        loop {
            println!("Permissão para {para_quem}");
            let valor = input_int("Digite um número de 0 a 7: ");

            if valor <= 7 {
                return Self::new(
                    (valor & 4) != 0,
                    (valor & 2) != 0,
                    (valor & 1) != 0,
                );
            }

            println!("Permissão inválida.");
        }
    }
}
type Permissoes = (Permissao, Permissao, Permissao);

fn ler_permissoes() -> Permissoes {
    let dono = Permissao::ler_permissao("dono");
    let grupo = Permissao::ler_permissao("grupo");
    let outros = Permissao::ler_permissao("outros");
    (dono, grupo, outros)
}

struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: Permissoes,
    uid_dono: u16,
    gid_grupo: u16,
}

impl Arquivo {
    fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Self {
        Self {
            nome,
            tamanho,
            permissoes: (
                Permissao::new(true, true, false),
                Permissao::new(true, false, false),
                Permissao::new(true, false, false),
            ),
            uid_dono: uid,
            gid_grupo: gid,
        }
    }

    fn alterar_permissao(&mut self, permissoes: Permissoes) {
        self.permissoes = permissoes;
    }

    fn pode_ler(&self, usuario: &Usuario) -> bool {
        if usuario.uid == 0 {
            return true;
        }

        if usuario.uid == self.uid_dono {
            self.permissoes.0.leitura
        } else if usuario.grupos_ids.contains(&self.gid_grupo) {
            self.permissoes.1.leitura
        } else {
            self.permissoes.2.leitura
        }
    }

    fn stat(&self) {
        let (dono, grupo, outros) = &self.permissoes;
        let mut simbolos = String::new();

        for permissao in [dono, grupo, outros] {
            simbolos.push(if permissao.leitura { 'r' } else { '-' });
            simbolos.push(if permissao.escrita { 'w' } else { '-' });
            simbolos.push(if permissao.execucao { 'x' } else { '-' });
        }

        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!(
            "Permissões: ({}{}{} / {})",
            dono.octal(), grupo.octal(), outros.octal(), simbolos
        );
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);
    }
}

struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: Permissoes,
    uid_dono: u16,
}

impl Diretorio {
    fn new(nome: String, permissoes: Permissoes, uid_dono: u16) -> Self {
        Self {
            nome,
            arquivos: Vec::new(),
            permissoes,
            uid_dono,
        }
    }

    fn permissao_para(&self, uid: u16) -> &Permissao {
        if uid == self.uid_dono {
            &self.permissoes.0
        } else {
            &self.permissoes.2
        }
    }

    fn pode_modificar(&self, uid: u16) -> bool {
        let permissao = self.permissao_para(uid);
        uid == 0 || (uid == self.uid_dono && permissao.escrita && permissao.execucao)
    }

    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    fn remove_arquivo(&mut self, nome_arquivo: &str) {
        let posicao = self.arquivos.iter().position(|a| a.nome == nome_arquivo);

        match posicao {
            Some(indice) => {
                self.arquivos.remove(indice);
                println!("Arquivo apagado.");
            }
            None => println!("Arquivo não encontrado."),
        }
    }

    fn listar_conteudo(&self) {
        if self.arquivos.is_empty() {
            println!("Diretório vazio.");
        }

        for arquivo in &self.arquivos {
            arquivo.stat();
        }
    }
}

struct Sistema {
    usuarios: Vec<Usuario>,
    grupos: Vec<Grupo>,
    diretorios: Vec<Diretorio>,
}

impl Sistema {
    fn new() -> Self {
        let root = Usuario::new(String::from("root"), 0);
        let mut user = Usuario::new(String::from("user"), 1000);

        let mut grupo = Grupo::new(String::from("usuarios"), 1000);
        user.grupos_ids.push(grupo.gid);
        grupo.membros_uids.push(user.uid);

        Self {
            usuarios: vec![root, user],
            grupos: vec![grupo],
            diretorios: Vec::new(),
        }
    }
}
fn login(sistema: &Sistema) -> Option<u16> {
    loop {
        println!("\n--- Login ---");
        let nome = input("Usuário (ou 'sair'): ");

        if nome == "sair" {
            return None;
        }

        let senha = input("Senha: ");
        let senha_correta = match nome.as_str() {
            "root" => senha == "root123",
            "user" => senha == "user123",
            _ => false,
        };

        if senha_correta {
            if let Some(usuario) = sistema.usuarios.iter().find(|u| u.nome == nome) {
                println!("Login realizado como {}.", usuario.nome);
                return Some(usuario.uid);
            }
        }

        println!("Usuário ou senha incorretos.");
    }
}

fn criar_diretorio(sistema: &mut Sistema, uid: u16) {
    let nome = input("Nome do diretório: ");

    if sistema.diretorios.iter().any(|d| d.nome == nome) {
        println!("Já existe um diretório com esse nome.");
        return;
    }

    let uid_dono = if uid == 0 {
        input_int("UID do dono (0 = root, 1000 = user): ")
    } else {
        uid
    };

    let permissoes = ler_permissoes();
    let diretorio = Diretorio::new(nome, permissoes, uid_dono);
    sistema.diretorios.push(diretorio);
    println!("Diretório criado.");
}

fn criar_arquivo(sistema: &mut Sistema, uid: u16) {
    if sistema.diretorios.is_empty() {
        println!("Crie um diretório primeiro.");
        return;
    }

    let nome_dir = input("Em qual diretório deseja criar o arquivo? ");
    let Some(dir) = sistema.diretorios.iter_mut().find(|d| d.nome == nome_dir) else {
        println!("Diretório não encontrado.");
        return;
    };

    if !dir.pode_modificar(uid) {
        println!("Acesso negado.");
        return;
    }

    let nome = input("Nome do arquivo: ");
    if dir.arquivos.iter().any(|a| a.nome == nome) {
        println!("Já existe um arquivo com esse nome no diretório.");
        return;
    }

    let tamanho = loop {
        match input("Tamanho em bytes: ").parse::<u64>() {
            Ok(valor) => break valor,
            Err(_) => println!("Digite um tamanho válido."),
        }
    };

    let (dono, gid) = if uid == 0 {
        (input_int("UID do dono: "), input_int("GID do grupo: "))
    } else {
        (uid, 1000)
    };

    dir.adiciona_arquivo(Arquivo::new(nome, tamanho, dono, gid));
    println!("Arquivo criado.");
}

fn editar_arquivo(sistema: &mut Sistema, uid: u16) {
    let nome_dir = input("Em qual diretório está o arquivo? ");
    let Some(dir) = sistema.diretorios.iter_mut().find(|d| d.nome == nome_dir) else {
        println!("Diretório não encontrado.");
        return;
    };

    if uid != 0 && !dir.permissao_para(uid).execucao {
        println!("Acesso negado ao diretório.");
        return;
    }

    let nome = input("Nome do arquivo: ");
    let Some(arquivo) = dir.arquivos.iter_mut().find(|a| a.nome == nome) else {
        println!("Arquivo não encontrado.");
        return;
    };

    if uid != 0 && uid != arquivo.uid_dono {
        println!("Somente o dono ou root pode alterar as permissões.");
        return;
    }

    arquivo.alterar_permissao(ler_permissoes());
    println!("Permissões atualizadas.");
}

fn apagar_arquivo(sistema: &mut Sistema, uid: u16) {
    let nome_dir = input("Em qual diretório está o arquivo? ");
    let Some(dir) = sistema.diretorios.iter_mut().find(|d| d.nome == nome_dir) else {
        println!("Diretório não encontrado.");
        return;
    };

    if !dir.pode_modificar(uid) {
        println!("Acesso negado.");
        return;
    }

    let nome = input("Nome do arquivo que deseja apagar: ");
    dir.remove_arquivo(&nome);
}

fn listar_arquivos(sistema: &Sistema, usuario: &Usuario) {
    if sistema.diretorios.is_empty() {
        println!("Nenhum diretório cadastrado.");
        return;
    }

    for dir in &sistema.diretorios {
        println!("\nDiretório: {}", dir.nome);

        if usuario.uid == 0 {
            dir.listar_conteudo();
            continue;
        }

        let permissao = dir.permissao_para(usuario.uid);
        if !permissao.leitura || !permissao.execucao {
            println!("Acesso negado ao conteúdo.");
            continue;
        }

        if dir.arquivos.is_empty() {
            println!("Diretório vazio.");
        }

        for arquivo in &dir.arquivos {
            if arquivo.pode_ler(usuario) {
                arquivo.stat();
            } else {
                println!("Arquivo com leitura restrita.");
            }
        }
    }
}

fn main() {
    let mut sistema = Sistema::new();

    while let Some(uid) = login(&sistema) {
        loop {
            println!("\n--- Menu ---");
            println!("1. Criar diretório");
            println!("2. Criar arquivo");
            println!("3. Editar permissões de um arquivo");
            println!("4. Apagar arquivo");
            println!("5. Listar arquivos");
            println!("6. Sair do programa");
            println!("7. Trocar usuário");

            match input("Escolha uma opção: ").as_str() {
                "1" => criar_diretorio(&mut sistema, uid),
                "2" => criar_arquivo(&mut sistema, uid),
                "3" => editar_arquivo(&mut sistema, uid),
                "4" => apagar_arquivo(&mut sistema, uid),
                "5" => {
                    if let Some(usuario) = sistema.usuarios.iter().find(|u| u.uid == uid) {
                        listar_arquivos(&sistema, usuario);
                    }
                }
                "6" => return,
                "7" => break,
                _ => println!("Opção inválida."),
            }
        }
    }
}