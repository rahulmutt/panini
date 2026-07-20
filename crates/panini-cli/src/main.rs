#![forbid(unsafe_code)]
use clap::{Parser, Subcommand, ValueEnum};
use panini::{Panini, Verdict, render};
use panini_lipi::Scheme;

#[derive(Parser)]
#[command(name = "panini", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Validate a single word and show its derivation.
    Check {
        word: String,
        #[arg(long, value_enum, default_value = "auto")]
        r#in: InScheme,
        #[arg(long, value_enum, default_value = "iast")]
        out: OutScheme,
        #[arg(long)]
        trace: bool,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum InScheme {
    Auto,
    Slp1,
    Iast,
    Hk,
    Deva,
}
#[derive(Clone, Copy, ValueEnum)]
enum OutScheme {
    Slp1,
    Iast,
    Hk,
    Deva,
}

fn out_scheme(o: OutScheme) -> Scheme {
    match o {
        OutScheme::Slp1 => Scheme::Slp1,
        OutScheme::Iast => Scheme::Iast,
        OutScheme::Hk => Scheme::Hk,
        OutScheme::Deva => Scheme::Devanagari,
    }
}

/// Map an explicit `--in` choice to a `panini_lipi::Scheme`. Returns `None`
/// for `Auto`, whose caller should keep relying on `Panini::check`'s
/// built-in auto-detection instead of forcing a scheme.
fn in_scheme(i: InScheme) -> Option<Scheme> {
    match i {
        InScheme::Auto => None,
        InScheme::Slp1 => Some(Scheme::Slp1),
        InScheme::Iast => Some(Scheme::Iast),
        InScheme::Hk => Some(Scheme::Hk),
        InScheme::Deva => Some(Scheme::Devanagari),
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Check {
            word,
            r#in,
            out,
            trace,
            json,
        } => {
            let engine = Panini::new();
            // When the caller declares an explicit input scheme, that scheme
            // is authoritative: transliterate to SLP1 ourselves before
            // checking, rather than trusting auto-detection (which `--in`
            // exists to override). `--in auto` (the default) keeps the
            // original behavior of letting `check` auto-detect/normalize.
            let result = match in_scheme(r#in) {
                Some(scheme) => {
                    let slp1_word = panini_lipi::to_slp1(&word, scheme);
                    engine.check(&slp1_word)
                }
                None => engine.check(&word),
            };
            let scheme = out_scheme(out);
            if json {
                let obj = serde_json::json!({
                    "verdict": matches!(result.verdict, Verdict::Valid),
                    "input_slp1": result.input_slp1,
                    "analyses": result.analyses.iter().map(|a| serde_json::json!({
                        "dhatu": a.dhatu,
                        "form": render(&a.form_slp1, scheme),
                        "trace": a.trace.iter().map(|s| serde_json::json!({"sutra": s.sutra, "name": s.name, "after": s.after})).collect::<Vec<_>>(),
                    })).collect::<Vec<_>>(),
                });
                println!("{}", serde_json::to_string_pretty(&obj).unwrap());
            } else if matches!(result.verdict, Verdict::Valid) {
                let a = &result.analyses[0];
                println!(
                    "VALID \u{2713}  {} ({})",
                    render(&a.form_slp1, scheme),
                    a.dhatu
                );
                if trace {
                    for step in &a.trace {
                        println!("  {} {} -> {}", step.sutra, step.name, step.after);
                    }
                }
            } else {
                println!("INVALID (not derivable within the covered v1 grammar)");
            }
            std::process::exit(if matches!(result.verdict, Verdict::Valid) {
                0
            } else {
                1
            });
        }
    }
}
