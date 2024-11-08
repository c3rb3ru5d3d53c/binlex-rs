pub struct Symbols;

impl Symbols {
    #[allow(dead_code)]
    pub fn demangle_msvc_symbol(mangled_name: &str) -> String {
        if !mangled_name.starts_with('?') {
            return mangled_name.to_string();
        }
        let parts: Vec<&str> = mangled_name.trim_start_matches('?').split('@').collect();
        let function_name = parts.get(0).unwrap_or(&mangled_name);
        let mut namespaces: Vec<&str> = parts.iter().skip(1).take_while(|&&s| s != "").map(|&s| s).collect();
        namespaces.reverse();
        format!(
            "{}::{}",
            namespaces.join("::"),
            function_name
        )
    }
}