use perseus::{ErrorPages, Html};
use sycamore::view;

pub fn get_error_pages<G: Html>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(|_, _, _, _| {
        view! {
            p { "Another error occurred." }
        }
    });
    error_pages.add_page(404, |_, _, err, _| {
        view! {
            p { "Page not found." }
            p { (err) }
        }
    });
    error_pages.add_page(400, |_, _, _, _| {
        view! {
            p { "Client error occurred..." }
        }
    });

    error_pages
}
