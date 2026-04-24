use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut out: BufWriter<io::Stdout> = BufWriter::new(stdout);
    writeln!(out, "Hello")?;
    writeln!(out, "My name is ")?;
    out.flush()?;
    Ok(())
}
