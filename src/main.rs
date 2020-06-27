mod cli;

fn main() -> std::io::Result<()> {
    let project = cli::Project::new();
    project.build_structure()?;

    Ok(())
}
