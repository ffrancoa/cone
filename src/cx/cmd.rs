use clap::{Command, Arg};
use shlex::split;


pub fn parse_input_line(line: &str) {
    // Intentar dividir la línea en tokens estilo shell
    let args = match split(line) {
        Some(tokens) => tokens,
        None => {
            eprintln!("Error: comillas no balanceadas en la entrada");
            return;
        }
    };
    // Ahora 'args' es un Vec<String> con cada palabra/argumento separado.
    // Ejemplo: line = "load -f \"mi archivo.csv\"" -> args = ["load", "-f", "mi archivo.csv"]
    let parser = build_cmd_parser();
    let parse_result = parser.try_get_matches_from(&args);
    match parse_result {
        Ok(matches) => {
            // El parsing fue exitoso, ahora determinar qué comando se pidió
            if let Some((name, sub_matches)) = matches.subcommand() {
                match name {
                    "load" => { 
                        // Extraer opciones de sub_matches, ejecutar lógica del comando
                        let file = sub_matches.get_one::<String>("file").expect("required");
                        let dir = sub_matches.get_one::<String>("dir");
                        println!("Ejecutando load: file = {}, dir = {:?}", file, dir);
                        // Aquí iría la lógica para cargar el archivo...
                    },
                    "preview" => {
                        let num = sub_matches.get_one::<String>("num")
                                             .and_then(|s| s.parse::<usize>().ok())
                                             .unwrap_or(5);
                        println!("Ejecutando preview de {} líneas...", num);
                        // Llamar función de previsualización...
                    },
                    "filter" => {
                        let cond = sub_matches.get_one::<String>("condition").unwrap();
                        let ignore_case = sub_matches.get_flag("ignore-case");
                        println!("Ejecutando filter: cond='{}', ignore_case={}", cond, ignore_case);
                        // Llamar función de filtrado...
                    },
                    "exit" => {
                        println!("Saliendo del REPL...");
                        std::process::exit(0);
                    },
                    _ => unreachable!("Comando desconocido parseado")
                }
            }
        },
        Err(e) => {
            // Ocurrió un error durante el parseo
            println!("{}", e)
        }
    }
}

fn build_cmd_parser() -> Command {
    Command::new("")  // Nombre vacío: no lo usamos en la ayuda
        .no_binary_name(true)  // Indicar que el primer token es el comando directamente,
        .subcommand(cmd_load())     // agregar subcomando "load"
        .subcommand(cmd_preview())  // agregar subcomando "preview"
        .subcommand(cmd_filter())   // agregar subcomando "filter"
}

/// Definir el subcomando "load"
fn cmd_load() -> Command {
    Command::new("load")
        .about("Cargar un archivo CSV en la aplicación")  // Descripción breve del comando
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("ARCHIVO")
                .help("Ruta del archivo CSV a cargar") 
                .required(true)  // Obligatorio: Clap validará que se proporcione
        )
        .arg(
            Arg::new("dir")
                .short('d')
                .long("dir")
                .value_name("DIR")
                .help("Directorio base donde se encuentra el archivo (opcional)")
                .required(false)
        )
}
/// Definir el subcomando "preview"
fn cmd_preview() -> Command {
    Command::new("preview")
        .about("Previsualizar los datos cargados actualmente")
        .arg(
            Arg::new("num")
                .short('n')
                .long("num")
                .value_name("N")
                .help("Número de líneas a mostrar en la vista previa (por defecto 5)")
                .required(false)
        )
}
/// Definir el subcomando "filter"
fn cmd_filter() -> Command {
    Command::new("filter")
        .about("Filtrar los datos según una condición")
        .arg(
            Arg::new("condition")
                .help("Condición de filtrado (ejemplo: columna=valor)")
                .required(true)
        )
        .arg(
            Arg::new("ignore-case")
                .short('i')
                .long("ignore-case")
                .help("Ignorar mayúsculas/minúsculas al filtrar")
                .action(clap::ArgAction::SetTrue)  // true si se especifica
        )
}