use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_struct_table::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[derive(TableRow, Clone)]
#[table(impl_vec_data_provider)]
pub struct TestStruct {
    pub test_field: String,
}

#[server]
pub async fn load_resource() -> Result<Vec<i32>, ServerFnError> {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    Ok(vec![])
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let _resource = Resource::new(move || {}, move |_| load_resource());

    let rows: Vec<TestStruct> = vec![TestStruct {
        test_field: "test content".to_string(),
    }];

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-struct-table-bug.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        <div>
            <table>
                // <TableContent
                //     rows
                //     scroll_container="html"
                // />
            </table>
        </div>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div>
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
