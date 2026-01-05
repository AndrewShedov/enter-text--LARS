use leptos::prelude::*;
use leptos_meta::{ provide_meta_context, Stylesheet, Title };
use leptos_router::{ components::{ Route, Router, Routes }, StaticSegment, SsrMode };

#[cfg(feature = "ssr")]
const PROTO_ID: &str = "11111111-1111-1111-1111-111111111111";

// --- SERVER FUNCTIONS ---

#[server(GetContent, "/api")]
pub async fn get_content() -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use scylla::Session;
        use std::sync::Arc;
        use uuid::Uuid;
        use leptos_actix::extract;
        use actix_web::web::Data;

        let session_data: Data<Arc<Session>> = extract().await?;
        let session = session_data.get_ref();
        let id = Uuid::parse_str(PROTO_ID).unwrap();

        // Querying the 'content' field in the 'data' table
        let query = "SELECT content FROM prototype.data WHERE id = ? LIMIT 1";
        let res = session
            .query_unpaged(query, (id,)).await
            .map_err(|e| ServerFnError::new(format!("ScyllaDB error: {}", e)))?;

        if let Some(row) = res
                .maybe_first_row_typed::<(String,)>()
                .map_err(|e| ServerFnError::new(format!("Type error: {}", e)))?
        {
            return Ok(row.0);
        }
        Ok("Database is empty".to_string())
    }
    #[cfg(not(feature = "ssr"))]
    { Err(ServerFnError::new("Internal Error")) }
}

#[server(SaveContent, "/api")]
pub async fn save_content(content: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use scylla::Session;
        use std::sync::Arc;
        use uuid::Uuid;
        use leptos_actix::extract;
        use actix_web::web::Data;

        if content.trim().is_empty() {
            return Err(ServerFnError::new("Empty"));
        }

        let session_data: Data<Arc<Session>> = extract().await?;
        let session = session_data.get_ref();
        let id = Uuid::parse_str(PROTO_ID).unwrap();

        // Inserting into the 'content' field
        let query = "INSERT INTO prototype.data (id, content, created_at) VALUES (?, ?, toTimestamp(now()))";
        session
            .query_unpaged(query, (id, content.clone())).await
            .map_err(|e| ServerFnError::new(format!("Write error: {}", e)))?;

        Ok(content)
    }
    #[cfg(not(feature = "ssr"))]
    { Err(ServerFnError::new("Internal Error")) }
}

#[server(DeleteContent, "/api")]
pub async fn delete_content() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use scylla::Session;
        use std::sync::Arc;
        use uuid::Uuid;
        use leptos_actix::extract;
        use actix_web::web::Data;

        let session_data: Data<Arc<Session>> = extract().await?;
        let session = session_data.get_ref();
        let id = Uuid::parse_str(PROTO_ID).unwrap();

        let query = "DELETE FROM prototype.data WHERE id = ?";
        session
            .query_unpaged(query, (id,)).await
            .map_err(|e| ServerFnError::new(format!("Delete error: {}", e)))?;

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    { Err(ServerFnError::new("Internal Error")) }
}

// /--- SERVER FUNCTIONS ---

// --- COMPONENTS ---

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/enter_text_lars.css"/>
        <Title text="Enter Text (LARS)"/>
        <Router>
            <main>
                <Routes fallback=move || "404">
                    <Route path=StaticSegment("") view=HomePage ssr=SsrMode::PartiallyBlocked />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let save_content_action = ServerAction::<SaveContent>::new();
    let delete_content_action = ServerAction::<DeleteContent>::new();
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let content_resource = Resource::new_blocking(
        move || (save_content_action.version().get(), delete_content_action.version().get()),
        |_| async move { get_content().await.unwrap_or_else(|_| "Database is empty".to_string()) }
    );

    Effect::new(move |_| {
        let _ = save_content_action.version().get();
        let _ = delete_content_action.version().get();
        if let Some(input) = input_ref.get() {
            input.set_value("");
        }
    });

    view! {
        <div class="container">
            <h1 class="title">"Enter Text (LARS)"</h1>
            
            <div class="card">
                <p class="card-label">"Text from ScyllaDB:"</p>
                <h2 class="db-text">
                    <Suspense fallback=move || view! { "Loading..." }>
                        {move || content_resource.get()}
                    </Suspense>
                </h2>

                <hr class="separator"/>

                <div class="controls">
                    <ActionForm action=save_content_action>
                        <div class="input-row">
                            <input 
                                type="text" 
                                name="content"
                                node_ref=input_ref
                                placeholder="Enter text..." 
                                class="input-field"
                            />
                            
                            <button type="submit" class="btn btn-black">
                                <Transition fallback=move || "Add">
                                    {move || content_resource.get().map(|val| {
                                        if val == "Database is empty" { "Add" } else { "Update" }
                                    }).unwrap_or("Add")}
                                </Transition>
                            </button>
                        </div>
                    </ActionForm>

                    <Transition>
                        {move || content_resource.get().map(|val| {
                            if val != "Database is empty" {
                                view! {
                                    <ActionForm action=delete_content_action>
                                        <button type="submit" class="btn btn-red">
                                            "Delete"
                                        </button>
                                    </ActionForm>
                                }.into_any()
                            } else {
                                view! { <div/> }.into_any()
                            }
                        })}
                    </Transition>
                </div>
            </div>
        </div>
    }
}
// /--- COMPONENTS ---