use std::collections::BTreeSet;
use std::process;
use binlex::models::controlflow::function::FunctionSymbolJson;
use clap::Parser;
use pdb::FallibleIterator;
use std::fs::File;
use binlex::models::terminal::io::Stdin;
use binlex::models::terminal::io::Stdout;
use binlex::models::symbols::Symbols;
use binlex::models::terminal::args::VERSION;
use binlex::models::terminal::args::AUTHOR;

#[derive(Parser, Debug)]
#[command(
    name = "blpdb",
    version = VERSION,
    about =  format!("A Binlex PDB Parsing Tool\n\nVersion: {}", VERSION),
    after_help = format!("Author: {}", AUTHOR),
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

    let mut results = Vec::<FunctionSymbolJson>::new();
    let mut symbols = symbol_table.iter();
    while let Some(symbol) = symbols.next()? {
        match symbol.parse() {
            Ok(pdb::SymbolData::Public(data)) if data.function => {
                let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                let mut name = data.name.to_string().into_owned();
                if cli.demangle_msvc_names {
                    name = Symbols::demangle_msvc_symbol(&name);
                }
                let mut names = BTreeSet::<String>::new();
                names.insert(name);
                results.push(FunctionSymbolJson{
                    type_: "function".to_string(),
                    names: names,
                    offset: None,
                    relative_virtual_address: Some(rva.0 as u64),
                    virtual_address: None,
                });
            }
            _ => {}
        }
    }

    Stdin::passthrough();

    for result in results {
        if let Ok(json_string) = serde_json::to_string(&result){
            Stdout.print(json_string);
        }
    }

    process::exit(0);
}