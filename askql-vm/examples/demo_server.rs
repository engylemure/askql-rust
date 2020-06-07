#![deny(warnings)]
#[macro_use]
extern crate serde;
extern crate log;
use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use askql_parser::{AskCodeOrValue, Value};
use askql_vm::resources::*;
use askql_vm::run::{AskVm, RunOptions};

#[derive(Serialize, Deserialize)]
struct Payload {
    code: String,
}

fn vm() -> AskVm {
    let ask_resource = AskResource {};
    let call_resource = CallResource {};
    let get_resource = GetResource {};
    let sum_resource = SumResource {};
    let minus_resource = MinusResource {};
    let times_resource = TimesResource {};
    let concat_resource = ConcatResource {};
    let max_resource = MaxResource {};
    let list_resource = ListResource {};
    let node_resource = NodeResource {};
    let query_resource = QueryResource::new();
    let fragment_resource = FragmentResource {};
    let lowercase_resource = ToLowerCaseResource {};
    let uppercase_resource = ToUpperCaseResource {};
    let resources: Vec<Box<dyn askql_vm::resource::Resource>> = vec![
        Box::new(ask_resource),
        Box::new(call_resource),
        Box::new(get_resource),
        Box::new(sum_resource),
        Box::new(minus_resource),
        Box::new(times_resource),
        Box::new(concat_resource),
        Box::new(max_resource),
        Box::new(list_resource),
        Box::new(node_resource),
        Box::new(query_resource),
        Box::new(fragment_resource),
        Box::new(lowercase_resource),
        Box::new(uppercase_resource),
    ];
    let mut values = std::collections::HashMap::new();
    values.insert(
        "firstName".to_string(),
        AskCodeOrValue::new_value(Value::String("PrimeiroNome".to_string())),
    );
    values.insert(
        "lastName".to_string(),
        AskCodeOrValue::new_value(Value::String("SegundoNome".to_string())),
    );
    let mut friend0 = std::collections::BTreeMap::new();
    let mut friend1 = std::collections::BTreeMap::new();
    let mut friend2 = std::collections::BTreeMap::new();
    friend0.insert("id".to_string(), Value::Int(1));
    friend0.insert(
        "firstName".to_string(),
        Value::String("Friend 0".to_string()),
    );
    friend1.insert("id".to_string(), Value::Int(2));
    friend1.insert(
        "firstName".to_string(),
        Value::String("Friend 1".to_string()),
    );
    friend2.insert("id".to_string(), Value::Int(3));
    friend2.insert(
        "firstName".to_string(),
        Value::String("Friend 2".to_string()),
    );
    let friends = vec![
        Value::Object(friend0),
        Value::Object(friend1),
        Value::Object(friend2),
    ];
    values.insert(
        "friends".to_string(),
        AskCodeOrValue::new_value(Value::List(friends)),
    );
    AskVm::new(RunOptions::new(resources, values))
}

async fn ask(vm: web::Data<AskVm>, payload: web::Json<Payload>) -> Result<HttpResponse, Error> {
    let Payload { code } = payload.into_inner();
    let code = askql_parser::parse(code, false).unwrap();
    let result = vm.run(code, None, None).await;
    let (mut response, value) = match result {
        Ok(result) => (HttpResponse::Ok(), result),
        Err(_) => (HttpResponse::BadRequest(), Value::Null),
    };
    Ok(response
        .content_type("application/json")
        .body(serde_json::to_string(&value)?))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let ask_vm = web::Data::new(vm());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(ask_vm.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/ask").route(web::post().to(ask)))
        // .service(
        //     web::resource("/")
        //         .route(web::post().to(graphql))
        //         .route(web::get().to(graphql)),
        // )
        // .service(web::resource("/playground").route(web::get().to(playground_handler)))
        // .service(web::resource("/graphiql").route(web::get().to(graphiql_handler)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
