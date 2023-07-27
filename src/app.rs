use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet href="https://demo.productionready.io/main.css"/>
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let page = create_rw_signal(0);
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>

        <div class="col-md-3">
            <div class="sidebar">
                <h4>"Popular Tags"</h4>
                <TagList />
            </div>
        </div>
        <ul class="pagination">
            <Show
                when=move || {page.get() > 0}
                fallback=|| ()
            >
                <li class="page-item">
                    <a class="btn btn-primary" on:click=move |_| page.update(|x| *x -= 1)>
                        "<< Previous page"
                    </a>
                </li>
            </Show>
            <Show
                when=move || {page.get() < 10}
                fallback=|| ()
            >
                <li class="page-item">
                    <a class="btn btn-primary" on:click=move |_| page.update(|x| *x += 1)>
                        "Next page >>"
                    </a>
                </li>
            </Show>
        </ul>
    }
}

#[server(TagListAction, "/api")]
async fn tag_list_action() -> Result<Vec<String>, ServerFnError> {
    Ok(vec![
        String::from("Hello"),
        String::from("brave"),
        String::from("and"),
        String::from("courageous"),
        String::from("world"),
    ])
}

#[component]
fn TagList() -> impl IntoView {
    let tag_list = create_resource(|| (), |_| async { tag_list_action().await });
    let pagination = use_query_map();

    let tag_view = move || {
        let tag_elected =
            pagination.with(|x| x.get("tag").map(ToString::to_string).unwrap_or_default());
        tag_list.with(move |ts| {
            ts.clone().map(move |tags| {
                view! {
                    <For
                        each=move || tags.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        view=move |(_, t): (usize, String)| {
                            let class = if t == tag_elected {"tag-pill tag-default tag-primary"} else {"tag-pill tag-default"};
                            let t2 = t.to_string();
                            let href = format!("/?tag={}", if tag_elected == t {
                                String::new()
                            } else {
                                t.to_string()
                            });
                            view!{<a href=href class=class>{t2}</a>}
                        }
                    />
                }
            })
        })
    };

    view! {
        <div class="tag-list">
            <Suspense fallback=move || view! {<p>"Loading Tags"</p> }>
                <ErrorBoundary fallback=|_| {
                    view! {
                        <ul class="error-messages">
                            <li>"Something went wrong."</li>
                        </ul>
                    }
                }>
                    {tag_view}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
