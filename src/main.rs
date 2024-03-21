use orca::pipeline::simple::LLMPipeline;
use orca::pipeline::Pipeline;
use orca::llm::quantized::Quantized;
use orca::llm::quantized::Model;
use orca::prompt::context::Context;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
}

// Change function signature
pub async fn get_response(message: String) -> anyhow::Result<String> {
    let instance = Quantized::new();
    let model_variant = Model::L7bChat;
    let model = instance.load_model(model_variant).await?;
    let built_model = model.build_model()?;

    // Adjust the prompt for a single country
    let prompt = r#"
        [INST] <<SYS>>
        You are a helpful assistant that answers any questions the user has.
        <</SYS>>
        {{message}}
        [/INST]
    "#;
    
    let pipeline = LLMPipeline::new(&built_model)
        .load_template("capitals", prompt)?
        .load_context(&Context::new(Data { message })?)?;
    let res = pipeline.execute("capitals").await?.content();

    // Return the result instead of printing and asserting
    Ok(res)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let message = "hi, how are you?".to_string();
    let response = get_response(message).await?;
    println!("{}", response);
    Ok(())
}
