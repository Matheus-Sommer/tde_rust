
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
struct Sistema 
{
    usuarios: Vec<Usuario>,
    grupos: Vec<Grupo>,
    diretorios: Vec<Diretorio>,
}

impl Sistema
{
    fn new () -> Sistema 
    {
        Sistema
        {
            usuarios: Vec::new(),
            grupos: Vec::new(),
            diretorios: Vec::new(),
        }

    }
}

struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>
}
impl Usuario
{
    fn new(nome: String, uid: u16) -> Usuario 
    {
        Usuario 
        {
            nome,
            uid,
            grupos_ids: Vec::new()
        }
    }
}


struct Grupo 
{
    nome: String,
    gid: u16,
    usuarios_ids: Vec<u16>
}

impl Grupo
{
    fn new(nome: String, gid: u16) -> Grupo 
    {
        Grupo 
        {
            nome,
            gid,
            usuarios_ids: Vec::new()
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
    fn new(
        nome: String,
        permissoes: (Permissao, Permissao, Permissao),
        uid_dono: u16,
    ) -> Self {
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
    let posicao = self.arquivos
        .iter()
        .position(|arquivo| arquivo.nome == nome_arquivo);

    match posicao {
        Some(indice) => {
            self.arquivos.remove(indice);
            println!("Arquivo '{}' apagado.", nome_arquivo);
        }
        None => println!("Arquivo '{}' não encontrado.", nome_arquivo),
    }
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
    fn new(
        nome: String,
        tamanho: u64,
        uid_dono: u16,
        gid_grupo: u16,
    ) -> Self {
        Self {
            nome,
            tamanho,
            permissoes: (
                Permissao::new(true, true, false),
                Permissao::new(true, false, false),
                Permissao::new(true, false, false),
            ),
            uid_dono,
            gid_grupo,
        }
    }

    fn alterar_permissao(
        &mut self,
        permissoes: (Permissao, Permissao, Permissao),
    ) {
        self.permissoes = permissoes;
    }
    fn stat(&self) {
    let (dono, grupo, outros) = &self.permissoes;
    let mut simbolos = String::new();

    for permissao in [dono, grupo, outros] {
        simbolos.push(if permissao.leitura  { 'r' } else { '-' });
        simbolos.push(if permissao.escrita  { 'w' } else { '-' });
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

struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}

impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Self 
    {
        Self {
            leitura,
            escrita,
            execucao,
        }
    }

    fn octal(&self) -> u8 
    {
        (self.leitura as u8) * 4 
        + (self.escrita as u8) * 2
        + (self.execucao as u8)
    }

    fn ler_permissao(para_quem: &str) -> Permissao 
    {
    loop {
        println!("Permissão para {para_quem}");
        let valor = input_int("Digite um número de 0 a 7: ");

        if valor <= 7 {
            let leitura = (valor & 4) != 0;
            let escrita = (valor & 2) != 0;
            let execucao = (valor & 1) != 0;

            return Permissao::new(leitura, escrita, execucao);
        }

        println!("Permissão inválida. Tente novamente.");
    }
}
}

fn criar_diretorio(sistema: &mut Sistema) 
{
    let nome = input("Nome do diretório: ");
    let uid_dono = input_int("UID do dono: ");

    let dono = Permissao::ler_permissao("dono");
    let grupo = Permissao::ler_permissao("grupo");
    let outros = Permissao::ler_permissao("outros");
    let permissoes = (dono, grupo, outros);

    let diretorio = Diretorio::new(nome, permissoes, uid_dono);
    sistema.diretorios.push(diretorio);

    println!("Diretório criado.");
}

fn criar_arquivo(sistema: &mut Sistema) 
{
    if sistema.diretorios.is_empty() 
    {
        println!("Crie um diretório antes de criar um arquivo.");
        return;
    }

    let nome_diretorio = input("Em qual diretório deseja criar o arquivo? ");

    let Some(diretorio) = sistema.diretorios
        .iter_mut()
        .find(|d| d.nome == nome_diretorio)
    else 
    {
        println!("Diretório não encontrado.");
        return;
    };

    let nome = input("Nome do arquivo: ");

    let tamanho = loop 
    {
        let texto = input("Tamanho em bytes: ");
        match texto.parse::<u64>() {
            Ok(valor) => break valor,
            Err(_) => println!("Digite um tamanho válido em bytes."),
        }
    };

    let uid = input_int("UID do dono: ");
    let gid = input_int("GID do grupo: ");

    let arquivo = Arquivo::new(nome, tamanho, uid, gid);
    diretorio.adiciona_arquivo(arquivo);

    println!("Arquivo criado no diretório '{}'.", diretorio.nome);
}

fn apagar_arquivo(sistema: &mut Sistema) {
    if sistema.diretorios.is_empty() {
        println!("Não há diretórios cadastrados.");
        return;
    }

    let nome_diretorio = input("Em qual diretório está o arquivo? ");

    let Some(diretorio) = sistema.diretorios
        .iter_mut()
        .find(|d| d.nome == nome_diretorio)
    else {
        println!("Diretório não encontrado.");
        return;
    };

    let nome_arquivo = input("Nome do arquivo que deseja apagar: ");
    diretorio.remove_arquivo(&nome_arquivo);
}
fn listar_arquivos(sistema: &Sistema) {
    if sistema.diretorios.is_empty() {
        println!("Nenhum diretório cadastrado.");
        return;
    }

    for diretorio in &sistema.diretorios {
        println!("\nDiretório: {}", diretorio.nome);
        diretorio.listar_conteudo();
    }
}

fn editar_arquivo(sistema: &mut Sistema)
{
    if sistema.diretorios.is_empty() {
        println!("Nenhum diretório cadastrado.");
        return;
    }

    let nome_diretorio = input("Em qual diretório está o arquivo? ");

    let Some(diretorio) = sistema.diretorios
        .iter_mut()
        .find(|d| d.nome == nome_diretorio)
    else {
        println!("Diretório não encontrado.");
        return;
    };

    let nome_arquivo = input("Nome do arquivo que deseja editar: ");

    let Some(arquivo) = diretorio.arquivos
        .iter_mut()
        .find(|a| a.nome == nome_arquivo)
    else {
        println!("Arquivo não encontrado.");
        return;
    };

    let dono = Permissao::ler_permissao("dono");
    let grupo = Permissao::ler_permissao("grupo");
    let outros = Permissao::ler_permissao("outros");

    arquivo.alterar_permissao((dono, grupo, outros));
    println!("Permissões do arquivo '{}' atualizadas.", arquivo.nome);
}

fn main() {
    let mut sistema = Sistema::new();

    loop {
        println!("\n1. Criar diretório");
        println!("2. Criar arquivo");
        println!("3. Editar arquivo");
        println!("4. Apagar arquivo");
        println!("5. Listar arquivos");
        println!("6. Sair");

        let opcao = input("Escolha uma opção: ");

        match opcao.as_str() {
            "1" => criar_diretorio(&mut sistema),
            "2" => criar_arquivo(&mut sistema),
            "3" => editar_arquivo(&mut sistema),
            "4" => apagar_arquivo(&mut sistema),
            "5" => listar_arquivos(&sistema),
            "6" => break,
            _ => println!("Opção inválida."),
        }
    }   
}