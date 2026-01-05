#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use enter_text_lars::app::*; 
    use scylla::SessionBuilder;
    use std::sync::Arc;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    // 1. ScyllaDB Connection
    let session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
        .expect("❌ Failed to connect to ScyllaDB");
    
    let session = Arc::new(session);

    // 2. Automatic schema initialization
    initialize_schema(&session).await;

    println!("✅ ScyllaDB is ready");

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();
        let s_leptos = session.clone();

        App::new()
            .app_data(web::Data::new(session.clone()))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", &site_root))
            .service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                let s_context = s_leptos.clone();
                move || {
                    provide_context(s_context.clone());
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body><App/></body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)?
    .run()
    .await
}

// Function for table creation
#[cfg(feature = "ssr")]
async fn initialize_schema(session: &scylla::Session) {
    println!("🧪 CRYSTAL: Checking data schema...");

    // Create table if it doesn't exist
    session.query_unpaged(
        "CREATE TABLE IF NOT EXISTS prototype.data (
            id uuid PRIMARY KEY,
            content text,
            created_at timestamp
        )",
        ()
    ).await.expect("❌ Error creating 'data' table");

    println!("✨ CRYSTAL: Table 'data' verified/created successfully");
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(opts: actix_web::web::Data<leptos::config::LeptosOptions>) -> actix_web::Result<actix_files::NamedFile> {
    let site_root = &opts.get_ref().site_root;
    Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.ico"))?)
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}