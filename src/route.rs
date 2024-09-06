pub mod character;

macro_rules! redirect {
    ($url: expr) => {
        HttpResponse::Found()
            .append_header(("Location", $url))
            .finish()
    };
}

pub(crate) use redirect;