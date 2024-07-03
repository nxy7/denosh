use eyre::{eyre, Result};
use std::io;
use url::Url;

use deno_core::*;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[op2]
fn op_sum(#[serde] nums: Vec<f64>) -> Result<f64, deno_core::error::AnyError> {
    // Sum inputs
    let sum = nums.iter().fold(0.0, |a, v| a + v);
    // return as a Result<f64, AnyError>
    Ok(sum)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = dotenv::dotenv();
    let default_filter = EnvFilter::from_default_env().add_directive("info".parse()?);
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(default_filter)
        .pretty()
        .with_file(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut runtime = init_runtime().await;
    loop {
        loop_tick(&mut runtime)?;
    }
}

async fn init_runtime() -> JsRuntime {
    const DECL: OpDecl = op_sum();
    let ext = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[DECL]),
        ..Default::default()
    };

    // Initialize a runtime instance
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });
    let url = Url::parse("https://deno.land/x/exec/mod.ts").unwrap();

    runtime.load_main_es_module(&url).await.unwrap();
    runtime
}

fn loop_tick(runtime: &mut JsRuntime) -> Result<()> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // check passed in syntax and modify it if needed
            // execute it on the runtime
            runtime
                .execute_script("", input)
                .map_err(|e| eyre!("{e}:?"))?;
        }
        Err(error) => println!("error: {error}"),
    }
    Ok(())
}

// Notes
// Should binary execution happen in deno or in rust binary?
// Rust binary uses tokio and is mulithreaded so it can be a better place to execute binaries.
// If that's how we do it then how will we pass state between deno and rust binary?
// Also, how will we differentiate between deno and rust syntax?
//
// Running everything inside deno has the benefit of being simpler approach, but it's more limited.
// How do we share Deno runtime between threads? With actor model? If that's the case then
// each binary could be ran as a separate actor and they can communicate with deno using messages.
