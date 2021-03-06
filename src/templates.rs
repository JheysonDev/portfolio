use actix_web::{HttpResponse, dev::HttpResponseBuilder, http::StatusCode};
use crate::{
    languages::HeaderTexts,
    utils::get_current_directory
};
use std::fs;

pub fn render_scss(name: &str) -> HttpResponse {
    let scss_rute: String = format!("{}templates/scss/{}.scss", get_current_directory(), name);

    match grass::from_path(
        &scss_rute,
        &grass::Options::default()
            .style(grass::OutputStyle::Compressed)
    ) {
        Ok(css_content) => HttpResponseBuilder::new(StatusCode::OK).content_type("text/css").body(css_content),
        Err(e) => {
            println!("Template SCSS ({}): {} ", name, e);
            HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body(format!("Cannot parse SCSS file: {}", scss_rute))
        },
    }
}

fn render_template(template_name: &str) -> String {
    let template_rute: String = format!("{}templates/html/{}.html", get_current_directory(), template_name);

    match fs::read_to_string(&template_rute) {
        Ok(template_content) => template_content,
        Err(_) => format!("Cannot read template: {}", template_rute),
    }
}

pub type TemplateData<'a> = Vec<(&'a str, &'a str)>;

fn render_template_with_data(template_name: &str, data: TemplateData) -> String {
    let mut template_content: String = render_template(template_name);

    if !template_content.starts_with("Cannot read template:") {
        for (key, value) in data {
            template_content = template_content.replace(format!("{{@{}}}", key).as_str(), value);
        }
    }

    template_content
}

pub struct HeaderData<'a> {
    pub page_id: &'a str,

    pub page_lang: String,
    pub page_url: String,

    pub page_title: String,
    pub page_description: String,

    pub header_texts: HeaderTexts,

    pub page_keywords: &'a str,
    pub page_image: &'a str,
}

pub fn render_html(
    name: &str,
    header: HeaderData,
    data: TemplateData,
) -> HttpResponse {
    let mut template_data: TemplateData = data;

    let mut header_data: TemplateData = TemplateData::new();
    header_data.push(("page_id", header.page_id));

    header_data.push(("page_lang", &header.page_lang));
    header_data.push(("page_url", &header.page_url));

    header_data.push(("page_title", &header.page_title));
    header_data.push(("page_description", &header.page_description));

    header_data.push(("nav_home", &header.header_texts.nav_home));
    header_data.push(("nav_projects", &header.header_texts.nav_projects));
    header_data.push(("nav_contact", &header.header_texts.nav_contact));
    header_data.push(("nav_blog", &header.header_texts.nav_blog));

    header_data.push(("page_keywords", header.page_keywords));
    header_data.push(("page_image", header.page_image));

    let template_header: String = render_template_with_data("header/header", header_data);
    template_data.push(("header", template_header.as_str()));

    let template_footer: String = render_template_with_data("header/footer", TemplateData::new());
    template_data.push(("footer", template_footer.as_str()));

    let mut template_response: HttpResponseBuilder = HttpResponse::Ok();

    let template_content = render_template_with_data(name, template_data);
    if template_content.starts_with("Cannot read template:") {
        template_response = HttpResponseBuilder::new(StatusCode::BAD_REQUEST);
    }

    template_response.body(template_content)
}
