use crate::web::ctx;

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    Render(#[from] handlebars::RenderError)
}

pub struct Renderer<'a>(handlebars::Handlebars<'a>)

impl<'a> Renderer<'a> {
    pub fn new(template_dit: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();
        renderer.register_template_directory(".hbs", &template_dit)
        .expect("failed to register handlebars templetes")
        Self(renderer)
    }

    fn convert_to_value<S>(serializable: &S) -> serde_json::Value
    where
        S: serde::Serialize + std::fmt::Debug 
        {
            serde_json::to_value(&serializable)
            .expect("filed to convert structur to value")
        }
}

