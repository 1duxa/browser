use anyhow::Result;
use rune::runtime::RuntimeContext;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Source, Sources, Vm};
use std::sync::Arc;

pub fn build() -> Result<()> {
    let context = Context::with_default_modules()?;

    let mut sources = Sources::new();
    _ = sources.insert(Source::memory("pub fn add(a, b) { a + b }")?);

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }
    let mut vm = Vm::new(Arc::new(RuntimeContext::default()), Arc::new(result?));

    let output = vm.call(["add"], (10i64, 20i64))?;
    let output: i64 = rune::from_value(output)?;

    println!("{}", output);
    Ok(())
}

#[test]
pub fn figure_out() {
    _ = build();
}
