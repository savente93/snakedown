use color_eyre::Result;
use dialoguer::Input;
use snakedown::config::ExternalIndex;
use snakedown::config::{ConfigBuilder, predefined_externals};
use snakedown::render::SSG;
use std::collections::HashMap;
use std::path::PathBuf;

use console;

use console::Style;
use dialoguer::{Confirm, MultiSelect, Select, theme::ColorfulTheme};

pub fn wizard() -> Result<ConfigBuilder> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    println!("Welcome to Snakedown!");
    println!("Please answer a few questions to get started quickly.");
    println!(
        "Any choices made can be changed by modifying the `snakedown.toml` or `pyproject.toml` file later.",
    );

    let mut out = ConfigBuilder::default();

    let suggestion = {
        let current_dir = std::env::current_dir().unwrap_or_default();
        let dirname = current_dir
            .components()
            .next_back()
            .and_then(|c| c.as_os_str().to_str())
            .map(|s| s.to_string());
        dirname.unwrap_or_default()
    };

    let pkg_path_input: String = Input::with_theme(&theme)
        .with_prompt("Where is the package you want to document?")
        .default(suggestion)
        .interact()?;

    let pkg_path = PathBuf::from(pkg_path_input);

    out = out.with_pkg_path(Some(pkg_path));

    let site_root_input: String = Input::with_theme(&theme)
        .with_prompt("Where is the root of the site you want to add the output to?")
        .default("docs/".to_string())
        .interact()?;

    let site_root = PathBuf::from(site_root_input);

    out = out.with_site_root(Some(site_root));

    let api_content_path_input: String = Input::with_theme(&theme)
        .with_prompt(
            "What is the path from the site content folder to where the output will be placed?",
        )
        .default("api/".to_string())
        .interact()?;

    let api_content_path = PathBuf::from(api_content_path_input);

    out = out.with_api_content_path(Some(api_content_path));

    let skip_private_input = Confirm::with_theme(&theme)
        .with_prompt("Do you want to skip private objects?")
        .interact()?;

    if skip_private_input {
        out = out.with_skip_private(Some(true));
    }

    let skip_undoc_choice = Confirm::with_theme(&theme)
        .with_prompt("Do you want to skip undocumented objects?")
        .interact()?;

    if skip_undoc_choice {
        out = out.with_skip_undoc(Some(true));
    }

    let ssg_choices = [SSG::Markdown, SSG::Zola];
    let ssg_choice_index = Select::with_theme(&theme)
        .with_prompt("What SSG would you like to use?")
        .default(0)
        .item(SSG::Zola)
        .item(SSG::Markdown)
        .interact()?;

    // we only allow options that we knwo so this get is guaranteed to
    // return Some
    #[allow(clippy::unwrap_used)]
    let ssg_choice = ssg_choices.get(ssg_choice_index).unwrap();

    out = out.with_ssg(Some(*ssg_choice));

    let mut pre_defined_externals = predefined_externals();
    let mut key_list = Vec::new();
    let mut selector = MultiSelect::with_theme(&theme);
    selector = selector.with_prompt("Would you like to reference any of these libraries?");

    for (key, val) in pre_defined_externals.iter() {
        key_list.push(key);
        selector = selector.item_checked(
            val.name.clone().unwrap_or_else(|| key.to_string()),
            key == "builtins",
        );
    }

    let selections = selector.interact()?;
    let chosen_keys: Vec<String> = selections.iter().map(|c| key_list[*c].clone()).collect();
    let chosen_externals: HashMap<String, ExternalIndex> = pre_defined_externals
        .extract_if(|k, _v| chosen_keys.contains(k))
        .collect();

    out = out.with_externals(Some(chosen_externals));

    Ok(out)
}
