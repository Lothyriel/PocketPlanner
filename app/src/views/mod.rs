pub async fn route(route: &str) -> Result<impl askama::Template, AppError> {
    match route {
        "/" => Ok(index().await),
        _ => Err(AppError {}),
    }
}

pub struct AppError {}

async fn index() -> IndexTemplate {
    IndexTemplate {
        test: String::from("sexo"),
    }
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    test: String,
}
