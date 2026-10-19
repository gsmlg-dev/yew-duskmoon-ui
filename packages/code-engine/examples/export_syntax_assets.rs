fn main() -> std::io::Result<()> {
    let directory = std::env::args_os().nth(1).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "usage: export_syntax_assets <static-directory>",
        )
    })?;
    code_engine::write_syntax_assets(directory)
}
