use proc_macro2::TokenStream;
use quote::quote;

pub(crate) enum FourXs {
    False,
    True,
}

pub(crate) fn handle_options_and_keywords(
    serde_rename: &mut Option<TokenStream>,
    field_name: &mut String,
    option: &mut bool,
) -> () {
    if field_name.starts_with("Option<") {
        *field_name = field_name
            .trim_end_matches(">")
            .trim_start_matches("Option<")
            .to_string();
        *option = true;
    }
    if crate::utils::RESERVED_KEYWORDS.contains(&field_name.as_str()) {
        *serde_rename = Some(
            format!("#[serde(rename = \"{}\")]", &field_name)
                .parse()
                .unwrap(),
        );
        field_name.push_str("_field");
    }
}

pub(crate) fn handle_option(name_hint: &mut String, option: &mut bool) -> () {
    if name_hint.starts_with("Option<") {
        *name_hint = name_hint
            .trim_end_matches(">")
            .trim_start_matches("Option<")
            .to_string();
        *option = true;
    }
}

pub(crate) fn add_pub_keywords(tokens: &mut Vec<TokenStream>) {
    *tokens = tokens
        .into_iter()
        .map(|ts| match ts.clone().into_iter().next() {
            None | Some(proc_macro2::TokenTree::Punct(_)) => ts.clone(),
            _ => quote!(pub #ts),
        })
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn handle_options_and_keywords_non_optional_non_keyword() {
        let mut observed_serde_rename = None;
        let mut observed_field_name = "fooople".to_string();
        let mut observed_option = false;
        handle_options_and_keywords(
            &mut observed_serde_rename,
            &mut observed_field_name,
            &mut observed_option,
        );
        assert!(observed_serde_rename.is_none());
        assert_eq!(observed_field_name, "fooople".to_string());
        assert_eq!(observed_option, false);
    }
    #[test]
    fn handle_options_and_keywords_optional_keyword() {
        let mut observed_serde_rename = None;
        let mut observed_field_name = "Option<yield>".to_string();
        let mut observed_option = false;
        handle_options_and_keywords(
            &mut observed_serde_rename,
            &mut observed_field_name,
            &mut observed_option,
        );
        assert!(observed_serde_rename.is_some());
        assert_eq!(observed_field_name, "yield_field".to_string());
        assert_eq!(observed_option, true);
    }
}
