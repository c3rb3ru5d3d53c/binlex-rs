use std::process;
use binlex::models::controlflow::function::FunctionQueueJson;
use clap::Parser;
use pdb::FallibleIterator;
use std::fs::File;
use binlex::models::terminal::Terminal;
use binlex::models::symbols::Symbols;
use binlex::models::config::VERSION;
use binlex::models::config::AUTHOR;

#[derive(Parser, Debug)]
#[command(
    name = "blpdb",
    version = VERSION,
    about = "A Binlex PDB Parsing Utility",
    author = AUTHOR,
)]
struct Cli {
    #[arg(short, long, required = true)]
    input: String,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(long, default_value_t = false)]
    demangle_msvc_names: bool
}

fn main() -> pdb::Result<()> {
    let cli = Cli::parse();

    let file = File::open(cli.input)?;
    let mut pdb = pdb::PDB::open(file)?;

    let symbol_table = pdb.global_symbols()?;
    let address_map = pdb.address_map()?;

    let mut results = Vec::<FunctionQueueJson>::new();
    let mut symbols = symbol_table.iter();
    while let Some(symbol) = symbols.next()? {
        match symbol.parse() {
            Ok(pdb::SymbolData::Public(data)) if data.function => {
                let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                let mut name = data.name.to_string().into_owned();
                if cli.demangle_msvc_names {
                    name = Symbols::demangle_msvc_symbol(&name);
                }
                results.push(FunctionQueueJson{
                    type_: "function".to_string(),
                    name: name,
                    offset: None,
                    relative_virtual_address: Some(rva.0 as u64),
                    virtual_address: None,
                });
            }
            _ => {}
        }
    }

    if let Err(error) = Terminal::STDIN.passthrough() {
        eprintln!("{}", error);
        process::exit(1);
    }

    for result in results {
        if let Ok(json_string) = serde_json::to_string(&result){
            if let Err(error) = Terminal::STDOUT.print(json_string) {
                eprintln!("{}", error);
                process::exit(1);
            }
        }
    }

    process::exit(0);
}