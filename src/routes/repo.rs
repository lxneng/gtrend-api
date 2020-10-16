use gtrend::repos;
use gtrend::Since;
use rocket_contrib::json::Json;
use serde_json::{json, Value};
use std::error::Error;

fn to_json(repos: Vec<repos::Repository>) -> Json<Value> {
    let x: Vec<_> = repos
        .into_iter()
        .map(|x| {
            json!({
                "author": x.author,
                "name": x.name,
                "avatar": x.avatar,
                "description": x.description,
                "url": x.url,
                "language": x.programming_language,
                "languageColor": x.lang_color,
                "stars": x.stars,
                "forks": x.forks,
                "currentPeriodStars": x.current_star,
                "builtBy": x.built_by
            })
        })
        .collect();

    Json(Value::Array(x))
}

#[get("/")]
pub fn repo_index() -> Result<Json<Value>, Box<dyn Error>> {
    let data = repos::builder().get_data();

    match data {
        Ok(val) => Ok(to_json(val)),
        Err(e) => Err(e),
    }
}

#[get("/repositories?<language>&<since>&<spoken_language_code>")]
pub fn repo_repositories(
    language: Option<String>,
    since: Option<String>,
    spoken_language_code: Option<String>,
) -> Result<Json<Value>, Box<dyn Error>> {
    let s = since.map(|x| Since::from_str(&x));
    let lang: Option<String> = language.map(|x| x.to_lowercase());
    let s_lang: Option<String> = spoken_language_code.map(|x| x.to_lowercase());

    let builder = match s {
        Some(val) => repos::builder().since(val),
        _ => repos::builder().since(Since::Daily),
    };

    let data = match (lang, s_lang) {
        (Some(l), Some(sl)) => builder
            .programming_language(&l)
            .spoken_language(&sl)
            .get_data(),
        (Some(l), None) => builder.programming_language(&l).get_data(),
        (None, Some(sl)) => builder.spoken_language(&sl).get_data(),
        _ => builder.get_data(),
    };

    match data {
        Ok(val) => Ok(to_json(val)),
        Err(e) => Err(e),
    }
}
