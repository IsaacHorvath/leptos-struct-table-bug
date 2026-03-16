use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Title};
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
        // sets the document title
        <Title text="Welcome to Leptos"/>
        <div>
            <table>
                <TableContent
                    rows
                    scroll_container="html"
                />
            </table>
        </div>
    }
}
