use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "install", &format!("packer@{}", version)])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn init(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "packer", "init", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn inspect(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "packer", "inspect", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn plugins(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "packer", "plugins", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn fmt(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "packer", "fmt", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn fix(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "packer", "fix", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn hcl2_upgrade(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "packer", "hcl2_upgrade", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn validate(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "packer", "validate", &args])?
        .stdout()?;
    Ok(stdout)
}
