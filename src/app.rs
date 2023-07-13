use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet href="https://demo.productionready.io/main.css"/>
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let page = create_rw_signal(cx, 0);
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    let tag = create_rw_signal(cx, String::new());

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>

        <div class="col-md-3">
                        <div class="sidebar">
                            <h4>"Popular Tags"</h4>
                            <TagList tag=tag />
                        </div>
                    </div>
        <ul class="pagination">
            <Show
                when=move || {page.get() > 0}
                fallback=|_| ()
            >
                <li class="page-item">
                    <a class="btn btn-primary" on:click=move |_| page.update(|x| *x -= 1)>
                        "<< Previous page"
                    </a>
                </li>
            </Show>
            <Show
                when=move || {page.get() < 10}
                fallback=|_| ()
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

#[component]
fn TagList(cx: Scope, tag: RwSignal<String>) -> impl IntoView {
    let tag_list = create_resource(
        cx,
        || (),
        |_| async {
            if env!("PATH").is_empty() {
                return Err(ServerFnError::ServerError("Impossible".into()));
            }
            Ok(vec![
                String::from("this"),
                String::from("other"),
                String::from("thing"),
            ])
        },
    );

    let tag_view = move || {
        let tag_elected = tag.get();
        tag_list.with(cx, move |ts| {
            ts.clone().map(move |tags| {
                view! { cx,
                    <For
                        each=move || tags.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        view=move |cx, (_, t): (usize, String)| {
                            let class = if t == tag_elected {"tag-pill tag-default tag-primary"} else {"tag-pill tag-default"};
                            let t2 = t.to_string();
                            view!{cx, <a href="" class=class  on:click=move |_| {
                                tag.update(|current_tag| {
                                    tracing::debug!("current_tag={current_tag},new_tag={t}");
                                    *current_tag = if current_tag == &t {
                                        String::new()
                                    } else {
                                        t.to_string()
                                    }
                                })
                            }>{t2}</a>}
                        }
                    />
                }
            })
        })
    };

    view! { cx,
        <div class="tag-list">
            <Suspense fallback=move || view! {cx, <p>"Loading Tags"</p> }>
                <ErrorBoundary fallback=|cx, _| {
                    view! { cx,
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
