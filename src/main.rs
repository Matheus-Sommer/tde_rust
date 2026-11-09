//Projeto Colaborativo RA1 e RA2
//Professor: Pedro Rocha Horchulhac
//Materia: Programação Imperativa
//Estudantes: Gustavo Fronza, Matheus Sommer 

fn input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Falha ao ler a entrada");
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
fn input_nome(prompt: &str) -> String {
    loop {
        let nome = input(prompt);
        if !nome.is_empty() {
            return nome;
        }
        println!("O nome não pode ficar vazio.");
    }
}
fn input_tamanho() -> u64 {
    loop {
        let texto = input("Tamanho em bytes: ");
        match texto.parse::<u64>() {
            Ok(valor) => return valor,
            Err(_) => println!("Digite um tamanho válido em bytes."),
        }
    }
}
struct Sistema {
    usuarios: Vec<Usuario>,
    grupos: Vec<Grupo>,
    diretorios: Vec<Diretorio>,
}

impl Sistema {
    fn new() -> Sistema {
        Sistema {
            usuarios: Vec::new(),
            grupos: Vec::new(),
            diretorios: Vec::new(),
        }
    }
}

struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>,
}
impl Usuario {
    fn new(nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupos_ids: Vec::new(),
        }
    }

    fn adiciona_grupo(&mut self, gid: u16) {
        for grupo_id in &self.grupos_ids {
            if *grupo_id == gid {
                return;
            }
        }
        self.grupos_ids.push(gid);
    }

    fn remove_grupo(&mut self, gid: u16) {
        for i in 0..self.grupos_ids.len() {
            if self.grupos_ids[i] == gid {
                self.grupos_ids.remove(i);
                return;
            }
        }
    }

    fn listar_grupos(&self) {
        println!("Grupos do usuário '{}' (UID {}):", self.nome, self.uid);
        if self.grupos_ids.is_empty() {
            println!("Nenhum grupo associado.");
        }
        for gid in &self.grupos_ids {
            println!("GID: {}", gid);
        }
    }
}

struct Grupo {
    nome: String,
    gid: u16,
    membros_uids: Vec<u16>,
}

impl Grupo {
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros_uids: Vec::new(),
        }
    }

    fn adiciona_membro(&mut self, uid: u16) {
        for membro_uid in &self.membros_uids {
            if *membro_uid == uid {
                return;
            }
        }
        self.membros_uids.push(uid);
    }

    fn remove_membro(&mut self, uid: u16) {
        for i in 0..self.membros_uids.len() {
            if self.membros_uids[i] == uid {
                self.membros_uids.remove(i);
                return;
            }
        }
    }

    fn listar_membros(&self) {
        println!("Membros do grupo '{}' (GID {}):", self.nome, self.gid);
        if self.membros_uids.is_empty() {
            println!("Nenhum membro associado.");
        }
        for uid in &self.membros_uids {
            println!("UID: {}", uid);
        }
    }
}

struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
}

impl Diretorio {
    fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Self {
        Self {
            nome,
            arquivos: Vec::new(),
            permissoes,
            uid_dono,
        }
    }

    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }
    fn remove_arquivo(&mut self, nome_arquivo: &str) {
        let posicao = self.buscar_arquivo(nome_arquivo);

        match posicao {
            Some(indice) => {
                self.arquivos.remove(indice);
                println!("Arquivo '{}' apagado.", nome_arquivo);
            }
            None => println!("Arquivo '{}' não encontrado.", nome_arquivo),
        }
    }

    fn buscar_arquivo(&self, nome: &str) -> Option<usize> {
        for i in 0..self.arquivos.len() {
            if self.arquivos[i].nome == nome {
                return Some(i);
            }
        }
        None
    }

    fn listar_conteudo(&self) {
        if self.arquivos.is_empty() {
            println!("  Nenhum arquivo neste diretório.");
            return;
        }

        for arquivo in &self.arquivos {
            arquivo.stat();
        }
    }
}
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16,
}
impl Arquivo {
    fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        Arquivo {
            nome,
            tamanho,
            permissoes: (
                Permissao::new(true, true, false),
                Permissao::new(true, true, false),
                Permissao::new(true, false, false),
            ),
            uid_dono: uid,
            gid_grupo: gid,
        }
    }

    fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }
    fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        mostrar_permissoes(&self.permissoes);
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);
    }
}

fn mostrar_permissoes(permissoes: &(Permissao, Permissao, Permissao)) {
    let (dono, grupo, outros) = permissoes;
    println!(
        "Permissões: ({}{}{} / {}{}{})",
        dono.octal(),
        grupo.octal(),
        outros.octal(),
        dono.simbolos(),
        grupo.simbolos(),
        outros.simbolos()
    );
}

fn ler_permissoes() -> (Permissao, Permissao, Permissao) {
    loop {
        let texto = input("Permissões (3 dígitos de 0 a 7, ex.: 777 ou 644): ");
        let mut digitos: Vec<u8> = Vec::new();
        let mut valido = true;

        for caractere in texto.chars() {
            match caractere {
                '0' => digitos.push(0),
                '1' => digitos.push(1),
                '2' => digitos.push(2),
                '3' => digitos.push(3),
                '4' => digitos.push(4),
                '5' => digitos.push(5),
                '6' => digitos.push(6),
                '7' => digitos.push(7),
                _ => valido = false,
            }
        }

        if valido && digitos.len() == 3 {
            return (
                Permissao::de_octal(digitos[0]),
                Permissao::de_octal(digitos[1]),
                Permissao::de_octal(digitos[2]),
            );
        }
        println!("Permissão inválida. Digite exatamente 3 dígitos entre 0 e 7.");
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
        (self.leitura as u8) * 4 + (self.escrita as u8) * 2 + (self.execucao as u8)
    }

    fn de_octal(valor: u8) -> Self {
        Self::new((valor & 4) != 0, (valor & 2) != 0, (valor & 1) != 0)
    }

    fn simbolos(&self) -> String {
        format!(
            "{}{}{}",
            if self.leitura { 'r' } else { '-' },
            if self.escrita { 'w' } else { '-' },
            if self.execucao { 'x' } else { '-' }
        )
    }
}

fn buscar_usuario(sistema: &Sistema, uid: u16) -> Option<usize> {
    for i in 0..sistema.usuarios.len() {
        if sistema.usuarios[i].uid == uid {
            return Some(i);
        }
    }
    None
}

fn buscar_grupo(sistema: &Sistema, gid: u16) -> Option<usize> {
    for i in 0..sistema.grupos.len() {
        if sistema.grupos[i].gid == gid {
            return Some(i);
        }
    }
    None
}

fn buscar_diretorio(sistema: &Sistema, nome: &str) -> Option<usize> {
    for i in 0..sistema.diretorios.len() {
        if sistema.diretorios[i].nome == nome {
            return Some(i);
        }
    }
    None
}

fn criar_usuario(sistema: &mut Sistema) {
    let nome = input_nome("Nome do usuário: ");
    for usuario in &sistema.usuarios {
        if usuario.nome == nome {
            println!("Já existe um usuário com esse nome.");
            return;
        }
    }

    let uid = input_int("UID do usuário: ");
    match buscar_usuario(sistema, uid) {
        Some(_) => {
            println!("Já existe um usuário com esse UID.");
            return;
        }
        None => {}
    }

    sistema.usuarios.push(Usuario::new(nome, uid));
    println!("Usuário criado.");
}

fn criar_grupo(sistema: &mut Sistema) {
    let nome = input_nome("Nome do grupo: ");
    for grupo in &sistema.grupos {
        if grupo.nome == nome {
            println!("Já existe um grupo com esse nome.");
            return;
        }
    }

    let gid = input_int("GID do grupo: ");
    match buscar_grupo(sistema, gid) {
        Some(_) => {
            println!("Já existe um grupo com esse GID.");
            return;
        }
        None => {}
    }

    sistema.grupos.push(Grupo::new(nome, gid));
    println!("Grupo criado.");
}

fn criar_diretorio(sistema: &mut Sistema) {
    if sistema.usuarios.is_empty() {
        println!("Cadastre um usuário antes de criar um diretório.");
        return;
    }

    let nome = input_nome("Nome do diretório: ");
    match buscar_diretorio(sistema, &nome) {
        Some(_) => {
            println!("Já existe um diretório com esse nome.");
            return;
        }
        None => {}
    }

    let uid_dono = input_int("UID do dono: ");
    match buscar_usuario(sistema, uid_dono) {
        Some(_) => {}
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    }

    let permissoes = ler_permissoes();
    
    sistema.diretorios.push(Diretorio::new(nome, permissoes, uid_dono));
    println!("Diretório criado.");
}

fn adicionar_arquivo(sistema: &mut Sistema) {
    if sistema.diretorios.is_empty() {
        println!("Crie um diretório antes de adicionar um arquivo.");
        return;
    }
    if sistema.grupos.is_empty() {
        println!("Cadastre um grupo antes de adicionar um arquivo.");
        return;
    }

    let nome_diretorio = input("Em qual diretório deseja adicionar o arquivo? ");
    let indice = match buscar_diretorio(sistema, &nome_diretorio) {
        Some(i) => i,
        None => {
            println!("Diretório não encontrado.");
            return;
        }
    };

    let nome = input_nome("Nome do arquivo: ");
    match sistema.diretorios[indice].buscar_arquivo(&nome) {
        Some(_) => {
            println!("Já existe um arquivo com esse nome neste diretório.");
            return;
        }
        None => {}
    }

    let tamanho = input_tamanho();
    let uid = input_int("UID do dono: ");
    match buscar_usuario(sistema, uid) {
        Some(_) => {}
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    }

    let gid = input_int("GID do grupo: ");
    match buscar_grupo(sistema, gid) {
        Some(_) => {}
        None => {
            println!("Grupo não encontrado.");
            return;
        }
    }

    let arquivo = Arquivo::new(nome, tamanho, uid, gid);
    sistema.diretorios[indice].adiciona_arquivo(arquivo);
    println!("Arquivo adicionado ao diretório '{}'.", nome_diretorio);
}

fn remover_arquivo(sistema: &mut Sistema) {
    if sistema.diretorios.is_empty() {
        println!("Não há diretórios cadastrados.");
        return;
    }

    let nome_diretorio = input("Em qual diretório está o arquivo? ");
    let indice = match buscar_diretorio(sistema, &nome_diretorio) {
        Some(i) => i,
        None => {
            println!("Diretório não encontrado.");
            return;
        }
    };
    let nome_arquivo = input("Nome do arquivo que deseja remover: ");
    sistema.diretorios[indice].remove_arquivo(&nome_arquivo);
}

fn listar_arquivos(sistema: &Sistema) {
    if sistema.diretorios.is_empty() {
        println!("Nenhum diretório cadastrado.");
        return;
    }

    for diretorio in &sistema.diretorios {
        println!("\nDiretório: {}", diretorio.nome);
        println!("UID do dono: {}", diretorio.uid_dono);
        mostrar_permissoes(&diretorio.permissoes);
        diretorio.listar_conteudo();
    }
}

fn alterar_permissoes_arquivo(sistema: &mut Sistema) {
    if sistema.diretorios.is_empty() {
        println!("Nenhum diretório cadastrado.");
        return;
    }

    let nome_diretorio = input("Em qual diretório está o arquivo? ");
    let indice_diretorio = match buscar_diretorio(sistema, &nome_diretorio) {
        Some(i) => i,
        None => {
            println!("Diretório não encontrado.");
            return;
        }
    };

    let nome_arquivo = input("Nome do arquivo que deseja alterar: ");
    let indice_arquivo = match sistema.diretorios[indice_diretorio].buscar_arquivo(&nome_arquivo) {
        Some(i) => i,
        None => {
            println!("Arquivo não encontrado.");
            return;
        }
    };

    let permissoes = ler_permissoes();
    sistema.diretorios[indice_diretorio].arquivos[indice_arquivo].alterar_permissao(permissoes);
    println!("Permissões do arquivo '{}' atualizadas.", nome_arquivo);
}

fn adicionar_usuario_grupo(sistema: &mut Sistema) {
    let uid = input_int("UID do usuário: ");
    let indice_usuario = match buscar_usuario(sistema, uid) {
        Some(i) => i,
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    };

    let gid = input_int("GID do grupo: ");
    let indice_grupo = match buscar_grupo(sistema, gid) {
        Some(i) => i,
        None => {
            println!("Grupo não encontrado.");
            return;
        }
    };

    for grupo_id in &sistema.usuarios[indice_usuario].grupos_ids {
        if *grupo_id == gid {
            println!("O usuário já pertence a esse grupo.");
            return;
        }
    }

    sistema.usuarios[indice_usuario].adiciona_grupo(gid);
    sistema.grupos[indice_grupo].adiciona_membro(uid);
    println!(
        "Usuário '{}' adicionado ao grupo '{}'.",
        sistema.usuarios[indice_usuario].nome, sistema.grupos[indice_grupo].nome
    );
}

fn remover_usuario_grupo(sistema: &mut Sistema) {
    let uid = input_int("UID do usuário: ");
    let indice_usuario = match buscar_usuario(sistema, uid) {
        Some(i) => i,
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    };

    let gid = input_int("GID do grupo: ");
    let indice_grupo = match buscar_grupo(sistema, gid) {
        Some(i) => i,
        None => {
            println!("Grupo não encontrado.");
            return;
        }
    };

    let mut pertence = false;
    for grupo_id in &sistema.usuarios[indice_usuario].grupos_ids {
        if *grupo_id == gid {
            pertence = true;
            break;
        }
    }

    if !pertence {
        println!("O usuário não pertence a esse grupo.");
        return;
    }

    sistema.usuarios[indice_usuario].remove_grupo(gid);
    sistema.grupos[indice_grupo].remove_membro(uid);

    println!(
        "Usuário '{}' removido do grupo '{}'.",
        sistema.usuarios[indice_usuario].nome, sistema.grupos[indice_grupo].nome
    );
}

fn listar_grupos_usuario(sistema: &Sistema) {
    let uid = input_int("UID do usuário: ");
    let indice_usuario = match buscar_usuario(sistema, uid) {
        Some(i) => i,
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    };

    sistema.usuarios[indice_usuario].listar_grupos();
}

fn listar_membros_grupo(sistema: &Sistema) {
    let gid = input_int("GID do grupo: ");
    match buscar_grupo(sistema, gid) {
        Some(i) => sistema.grupos[i].listar_membros(),
        None => println!("Grupo não encontrado."),
    }
}

fn listar_usuarios(sistema: &Sistema) {
    if sistema.usuarios.is_empty() {
        println!("Nenhum usuário cadastrado.");
    }
    for usuario in &sistema.usuarios {
        println!("UID: {} | Nome: {}", usuario.uid, usuario.nome);
    }
}

fn editar_usuario(sistema: &mut Sistema) {
    let uid = input_int("UID do usuário que deseja editar: ");
    let indice = match buscar_usuario(sistema, uid) {
        Some(i) => i,
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    };
    let nome = input_nome("Novo nome do usuário: ");
    for usuario in &sistema.usuarios {
        if usuario.nome == nome && usuario.uid != uid {
            println!("Já existe um usuário com esse nome.");
            return;
        }
    }
    sistema.usuarios[indice].nome = nome;
    println!("Usuário atualizado.");
}

fn remover_usuario(sistema: &mut Sistema) {
    let uid = input_int("UID do usuário que deseja remover: ");
    let indice = match buscar_usuario(sistema, uid) {
        Some(i) => i,
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    };

    for diretorio in &sistema.diretorios {
        if diretorio.uid_dono == uid {
            println!(
                "O usuário possui diretórios. Altere o dono ou remova os diretórios primeiro."
            );
            return;
        }
        for arquivo in &diretorio.arquivos {
            if arquivo.uid_dono == uid {
                println!(
                    "O usuário possui arquivos. Altere o dono ou remova os arquivos primeiro."
                );
                return;
            }
        }
    }

    for grupo in &mut sistema.grupos {
        grupo.remove_membro(uid);
    }
    sistema.usuarios.remove(indice);
    println!("Usuário removido.");
}

fn listar_grupos(sistema: &Sistema) {
    if sistema.grupos.is_empty() {
        println!("Nenhum grupo cadastrado.");
    }
    for grupo in &sistema.grupos {
        println!("GID: {} | Nome: {}", grupo.gid, grupo.nome);
    }
}

fn editar_grupo(sistema: &mut Sistema) {
    let gid = input_int("GID do grupo que deseja editar: ");
    let indice = match buscar_grupo(sistema, gid) {
        Some(i) => i,
        None => {
            println!("Grupo não encontrado.");
            return;
        }
    };
    let nome = input_nome("Novo nome do grupo: ");
    for grupo in &sistema.grupos {
        if grupo.nome == nome && grupo.gid != gid {
            println!("Já existe um grupo com esse nome.");
            return;
        }
    }
    sistema.grupos[indice].nome = nome;
    println!("Grupo atualizado.");
}

fn remover_grupo(sistema: &mut Sistema) {
    let gid = input_int("GID do grupo que deseja remover: ");
    let indice = match buscar_grupo(sistema, gid) {
        Some(i) => i,
        None => {
            println!("Grupo não encontrado.");
            return;
        }
    };

    for diretorio in &sistema.diretorios {
        for arquivo in &diretorio.arquivos {
            if arquivo.gid_grupo == gid {
                println!("O grupo está associado a arquivos. Altere o grupo dos arquivos ou remova-os primeiro.");
                return;
            }
        }
    }

    for usuario in &mut sistema.usuarios {
        usuario.remove_grupo(gid);
    }
    sistema.grupos.remove(indice);
    println!("Grupo removido.");
}

fn listar_diretorios(sistema: &Sistema) {
    if sistema.diretorios.is_empty() {
        println!("Nenhum diretório cadastrado.");
    }
    for diretorio in &sistema.diretorios {
        println!("Diretório: {}", diretorio.nome);
        println!("UID do dono: {}", diretorio.uid_dono);
        println!("Quantidade de arquivos: {}", diretorio.arquivos.len());
        mostrar_permissoes(&diretorio.permissoes);
    }
}

fn editar_diretorio(sistema: &mut Sistema) {
    let nome_atual = input("Nome do diretório que deseja editar: ");
    let indice = match buscar_diretorio(sistema, &nome_atual) {
        Some(i) => i,
        None => {
            println!("Diretório não encontrado.");
            return;
        }
    };

    let nome = input_nome("Novo nome do diretório: ");
    for i in 0..sistema.diretorios.len() {
        if sistema.diretorios[i].nome == nome && i != indice {
            println!("Já existe um diretório com esse nome.");
            return;
        }
    }
    let uid = input_int("UID do dono: ");
    match buscar_usuario(sistema, uid) {
        Some(_) => {}
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    }
    let permissoes = ler_permissoes();
    let diretorio = &mut sistema.diretorios[indice];
    diretorio.nome = nome;
    diretorio.uid_dono = uid;
    diretorio.permissoes = permissoes;
    println!("Diretório atualizado.");
}

fn remover_diretorio(sistema: &mut Sistema) {
    let nome = input("Nome do diretório que deseja remover: ");
    let indice = match buscar_diretorio(sistema, &nome) {
        Some(i) => i,
        None => {
            println!("Diretório não encontrado.");
            return;
        }
    };

    if !sistema.diretorios[indice].arquivos.is_empty() {
        println!("Remova os arquivos do diretório antes de removê-lo.");
        return;
    }
    sistema.diretorios.remove(indice);
    println!("Diretório removido.");
}

fn editar_arquivo(sistema: &mut Sistema) {
    let nome_diretorio = input("Em qual diretório está o arquivo? ");
    let indice_diretorio = match buscar_diretorio(sistema, &nome_diretorio) {
        Some(i) => i,
        None => {
            println!("Diretório não encontrado.");
            return;
        }
    };
    let nome_atual = input("Nome do arquivo que deseja editar: ");
    let indice_arquivo = match sistema.diretorios[indice_diretorio].buscar_arquivo(&nome_atual) {
        Some(i) => i,
        None => {
            println!("Arquivo não encontrado.");
            return;
        }
    };

    let nome = input_nome("Novo nome do arquivo: ");
    for i in 0..sistema.diretorios[indice_diretorio].arquivos.len() {
        if sistema.diretorios[indice_diretorio].arquivos[i].nome == nome && i != indice_arquivo {
            println!("Já existe um arquivo com esse nome neste diretório.");
            return;
        }
    }
    let tamanho = input_tamanho();
    let uid = input_int("UID do dono: ");
    match buscar_usuario(sistema, uid) {
        Some(_) => {}
        None => {
            println!("Usuário não encontrado.");
            return;
        }
    }
    let gid = input_int("GID do grupo: ");
    match buscar_grupo(sistema, gid) {
        Some(_) => {}
        None => {
            println!("Grupo não encontrado.");
            return;
        }
    }
    let permissoes = ler_permissoes();
    let arquivo = &mut sistema.diretorios[indice_diretorio].arquivos[indice_arquivo];
    arquivo.nome = nome;
    arquivo.tamanho = tamanho;
    arquivo.uid_dono = uid;
    arquivo.gid_grupo = gid;
    arquivo.alterar_permissao(permissoes);
    println!("Arquivo atualizado.");
}

fn main() {
    let mut sistema = Sistema::new();

    loop {
        println!("\n1. Criar diretório");
        println!("2. Criar usuário");
        println!("3. Criar grupo");
        println!("4. Adicionar arquivo");
        println!("5. Remover arquivo");
        println!("6. Listar arquivos");
        println!("7. Alterar permissões de um arquivo");
        println!("8. Adicionar usuário a grupo");
        println!("9. Remover usuário do grupo");
        println!("10. Listar grupos do usuário");
        println!("11. Listar membros de um grupo");
        println!("12. Listar usuários");
        println!("13. Editar usuário");
        println!("14. Remover usuário");
        println!("15. Listar grupos");
        println!("16. Editar grupo");
        println!("17. Remover grupo");
        println!("18. Listar diretórios");
        println!("19. Editar diretório");
        println!("20. Remover diretório");
        println!("21. Editar arquivo");
        println!("22. Sair");

        let opcao = input("Escolha uma opção: ");
        match opcao.as_str() {
            "1" => criar_diretorio(&mut sistema),
            "2" => criar_usuario(&mut sistema),
            "3" => criar_grupo(&mut sistema),
            "4" => adicionar_arquivo(&mut sistema),
            "5" => remover_arquivo(&mut sistema),
            "6" => listar_arquivos(&sistema),
            "7" => alterar_permissoes_arquivo(&mut sistema),
            "8" => adicionar_usuario_grupo(&mut sistema),
            "9" => remover_usuario_grupo(&mut sistema),
            "10" => listar_grupos_usuario(&sistema),
            "11" => listar_membros_grupo(&sistema),
            "12" => listar_usuarios(&sistema),
            "13" => editar_usuario(&mut sistema),
            "14" => remover_usuario(&mut sistema),
            "15" => listar_grupos(&sistema),
            "16" => editar_grupo(&mut sistema),
            "17" => remover_grupo(&mut sistema),
            "18" => listar_diretorios(&sistema),
            "19" => editar_diretorio(&mut sistema),
            "20" => remover_diretorio(&mut sistema),
            "21" => editar_arquivo(&mut sistema),
            "22" => {
                println!("Programa encerrado.");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}
