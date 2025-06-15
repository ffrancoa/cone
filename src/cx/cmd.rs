use clap::{Parser, Subcommand, Args};
use shlex::split;


#[derive(Subcommand, Debug)]
enum Commands {
    /// Carga un archivo o directorio
    Load(LoadCmd),
    /// Previsualiza datos cargados
    Preview(PreviewCmd),
    /// Filtra resultados
    Filter(FilterCmd),
    /// Sale del REPL
    Exit,  // comando "exit" sin argumentos
}

/// Estructura principal del REPL CLI
#[derive(Parser, Debug)]
#[command(multicall = true, disable_help_flag = true)]
struct ReplCli {
    /// Campo que representa el comando introducido (subcomando)
    #[command(subcommand)]
    command: Commands,
}

pub fn parse_input_line(line: &str) {
    // Intentar dividir la línea en tokens estilo shell
    if let Some(args) = split(line) {
        // Intentar parsear la línea como nuestro CLI:
        match ReplCli::try_parse_from(args) {
            Ok(cli) => {
                // Tenemos un ReplCli parseado correctamente
                match cli.command {
                    Commands::Load(cmd) => {
                        println!("You choose the LOAD command. ({:?})", cmd)
                     },
                    Commands::Preview(cmd) => {
                        println!("You choosed the PREVIEW command. ({:?})", cmd)
                     },
                    Commands::Filter(cmd)  => {
                        println!("You choosed the LOAD command. ({:?})", cmd)
                     },
                    Commands::Exit         => {
                        println!("You left me.")
                     },
                }
            },
            Err(error_or_help_msg) => { error_or_help_msg.print().unwrap() }
        }
    }
}

#[derive(Args, Debug)]
struct LoadCmd {
    #[arg(short = 'f', long = "file", value_name = "FILE", 
          help = "Ruta de archivo a cargar")]
    file: Option<String>,

    #[arg(long = "dir", value_name = "DIR", 
          help = "Ruta de directorio a cargar")]
    dir: Option<String>,

    #[arg(long = "ignore-case", short = 'i', 
          help = "Ignorar mayúsculas/minúsculas en la operación")]
    ignore_case: bool,
}

#[derive(Args, Debug)]
struct PreviewCmd {
    // Ejemplo: si se quisiera una opción --lines (-n) para número de líneas:
    // #[arg(short = 'n', long = "lines", default_value = "10", help = "Número de líneas a mostrar")]
    // pub lines: usize,
}

#[derive(Args, Debug)]
struct FilterCmd {
    #[arg(name = "PATTERN", help = "Patrón de filtro a aplicar")]
    pattern: String,

    #[arg(long = "ignore-case", short = 'i', help = "Filtro sin distinguir mayúsculas")]
    ignore_case: bool,
}