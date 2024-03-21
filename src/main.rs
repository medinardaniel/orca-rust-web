use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use orca::pipeline::simple::LLMPipeline;
use orca::pipeline::Pipeline;
use orca::llm::quantized::Quantized;
use orca::llm::quantized::Model;
use orca::prompt::context::Context;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)] // Add Deserialize here
pub struct Data {
    message: String,
}


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

#[post("/get_response")]
async fn get_response_api(info: web::Json<Data>) -> impl Responder {
    let response = get_response(info.message.clone()).await;
    match response {
        Ok(res) => HttpResponse::Ok().body(res),
        Err(_) => HttpResponse::InternalServerError().body("Error generating response"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_response_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
