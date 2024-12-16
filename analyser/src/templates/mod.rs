use sailfish::Template;

pub mod nav;

#[derive(Template)]
#[template(path = "../src/templates/bytecode.stpl")]
pub struct Bytecode<'a> {
    pub title: String,
    pub rendered_breadcrumbs: String,
    pub rendered_hierarchy: &'a str,
    pub code: crate::dis::DisCode<'a>,
}
