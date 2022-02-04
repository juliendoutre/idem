use std::error::Error;
use std::io::Write;

use clap::{App, Arg, ArgMatches};

use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry_datadog::{new_pipeline, ApiVersion};

use idem::formatting::Formattable;
use idem::interpretation::Interpreter;
use idem::lexing::Lexer;
use idem::parsing::Parser;
use idem::reading::read;
use idem::validation::validate;
use idem::visualization;

fn main() {
    let app = App::new("idem")
        .version("0.1.0")
        .author("Julien Doutre <jul.doutre@gmail.com>")
        .about("idem language toolchain")
        .subcommand(
            App::new("run")
                .version("0.1.0")
                .about("Runs an Idem program")
                .arg(
                    Arg::new("PATH")
                        .about("Path to an Idem source file")
                        .required(true),
                )
                .arg(
                    Arg::new("tracing")
                    .about("activate open telemetry tracing")
                    .takes_value(false)
                ),
        )
        .subcommand(
            App::new("format")
                .version("0.1.0")
                .about("Format an Idem program")
                .arg(
                    Arg::new("PATH")
                        .about("Path to an Idem source file")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("validate")
                .version("0.1.0")
                .about("Check an Idem program")
                .arg(
                    Arg::new("PATH")
                        .about("Path to an Idem source file")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("display")
                .version("0.1.0")
                .about("Displays visualizations for an idem program")
                .subcommand(
                    App::new("functions")
                        .version("0.1.0")
                        .about("Displays function calls in an Idem program")
                        .arg(
                            Arg::new("PATH")
                                .about("Path to an Idem source file")
                                .required(true),
                        )
                        .arg(
                            Arg::new("output")
                                .long("output")
                                .short('o')
                                .about("Path to a file to which save the graph representation. Default to stdin."),
                        )
                ),
        );

    print_if_error(match app.get_matches().subcommand() {
        Some(("run", matches)) => run_cmd(matches),
        Some(("format", matches)) => format_cmd(matches),
        Some(("validate", matches)) => validate_cmd(matches),
        Some(("display", matches)) => match matches.subcommand() {
            Some(("functions", sub_matches)) => display_functions_cmd(sub_matches),
            _ => Ok(()),
        },
        _ => Ok(()),
    })
}

fn print_if_error(result: Result<(), Box<dyn Error>>) {
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run_cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = matches.value_of_t::<String>("PATH")?;
    let mut lexer = Lexer::new(&path);
    let tokens = lexer.tokenize(read(&path)?.chars());
    let mut tokens_stream = tokens.iter();

    let mut parser = Parser::new(&mut tokens_stream);
    let ast = parser.parse()?;

    let reports = validate(&ast);
    if !reports.is_empty() {
        eprintln!("Issues were found in {}:", path);

        for report in reports {
            println!("{}", report);
        }

        std::process::exit(1);
    }

    let mut interpreter = Interpreter {};

    if matches.is_present("tracing") {
        let _tracer = new_pipeline()
            .with_service_name("")
            .with_version(ApiVersion::Version05)
            .install_simple()?;
    }

    interpreter.run(&ast)?;

    if matches.is_present("tracing") {
        shutdown_tracer_provider();
    }

    Ok(())
}

fn format_cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = matches.value_of_t::<String>("PATH")?;
    let mut lexer = Lexer::new(&path);
    let tokens = lexer.tokenize(read(&path)?.chars());
    let mut tokens_stream = tokens.iter();

    let mut parser = Parser::new(&mut tokens_stream);
    let ast = parser.parse()?;

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;

    f.write_all(format!("{}\n", ast.format()).as_bytes())?;
    f.flush()?;

    Ok(())
}

fn validate_cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = matches.value_of_t::<String>("PATH")?;
    let mut lexer = Lexer::new(&path);
    let tokens = lexer.tokenize(read(&path)?.chars());
    let mut tokens_stream = tokens.iter();

    let mut parser = Parser::new(&mut tokens_stream);
    let ast = parser.parse()?;

    let reports = validate(&ast);
    for report in reports {
        println!("{}", report);
    }

    Ok(())
}

fn display_functions_cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = matches.value_of_t::<String>("PATH")?;

    let mut lexer = Lexer::new(&path);
    let tokens = lexer.tokenize(read(&path)?.chars());
    let mut tokens_stream = tokens.iter();

    let mut parser = Parser::new(&mut tokens_stream);
    let ast = parser.parse()?;

    let graph = visualization::function_calls_graph(&path, &ast);

    if let Ok(output) = matches.value_of_t::<String>("output") {
        let mut file = std::fs::File::create(output)?;
        file.write_all(format!("{}", graph).as_bytes())?;
    } else {
        println!("{}", graph);
    }

    Ok(())
}
