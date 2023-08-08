use color_eyre::{eyre::Result, Report};

use regex::Regex;
use serde::{Deserialize, Serialize};

use std::str;

use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::util::arguments::Commands;
use crate::util::color::SchemeExt;
use crate::Scheme;

use super::arguments::Cli;
use super::config::ConfigFile;
use material_color_utilities_rs::util::color::format_argb_as_rgb;
use resolve_path::PathResolveExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

struct ColorPattern {
    pattern: Regex,
    replacements: ColorReplacements,
}

struct ImagePattern<'a> {
    pattern: Regex,
    replacement: Option<&'a String>,
}

struct ColorReplacements {
    hex: String,
    hex_stripped: String,
    rgb: String,
    rgba: String,
}

struct Patterns<'a> {
    colors: Vec<ColorPattern>,
    image: ImagePattern<'a>,
}

use super::color::Color;

impl Template {
    pub fn generate(
        colors: &Vec<&str>,
        scheme: Scheme,
        config: &ConfigFile,
        args: &Cli,
    ) -> Result<(), Report> {
        let default_prefix = "@".to_string();

        let prefix: &String = match &config.config.prefix {
            Some(prefix) => prefix,
            None => &default_prefix,
        };

        info!("Loaded {} templates.", &config.templates.len());

        let image = match &args.source {
            Commands::Image { path } => Some(path),
            Commands::Color { .. } => None,
        };

        // TODO Use only one regex and use a for loop with matches?
        let regexvec: Patterns = generate_patterns(colors, scheme, prefix, image)?;

        // println!("{}", imageregex.is_match("@{image}"));

        for (name, template) in &config.templates {
            let input_path_absolute = template.input_path.try_resolve()?;
            let output_path_absolute = template.output_path.try_resolve()?;

            if !input_path_absolute.exists() {
                warn!("<d>The <yellow><b>{}</><d> template in <u>{}</><d> doesnt exist, skipping...</>", name, input_path_absolute.display());
                continue;
            }

            let mut data = read_to_string(&input_path_absolute)?;

            replace_matches(&regexvec, &mut data);

            let mut output_file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&output_path_absolute)?;

            output_file.write_all(data.as_bytes())?;
            success!(
                "Exported the <b><green>{}</> template to <d><u>{}</>",
                name,
                output_path_absolute.display()
            );
        }
        Ok(())
    }
}

fn replace_matches(regexvec: &Patterns, data: &mut String) {
    for regex in &regexvec.colors {
        let captures = regex.pattern.captures(data);

        let format = if let Some(caps) = captures {
            caps.get(1)
        } else {
            continue;
        };

        if format.is_some() {
            match format.unwrap().as_str() {
                ".hex" => {
                    *data = regex
                        .pattern
                        .replace_all(data, &regex.replacements.hex)
                        .to_string()
                }
                ".strip" => {
                    *data = regex
                        .pattern
                        .replace_all(data, &regex.replacements.hex_stripped)
                        .to_string()
                }
                ".rgb" => {
                    *data = regex
                        .pattern
                        .replace_all(data, &regex.replacements.rgb)
                        .to_string()
                }
                ".rgba" => {
                    *data = regex
                        .pattern
                        .replace_all(data, &regex.replacements.rgba)
                        .to_string()
                }
                _ => continue,
            }
        } else {
            *data = regex
                .pattern
                .replace_all(data, &regex.replacements.hex)
                .to_string()
        }
    }

    if let Some(image) = regexvec.image.replacement {
        *data = regexvec
            .image
            .pattern
            .replace_all(&*data, image)
            .to_string();
    }
}

fn generate_patterns<'a>(
    colors: &'a Vec<&'a str>,
    scheme: Scheme,
    prefix: &'a String,
    image: Option<&'a String>,
) -> Result<Patterns<'a>, Report> {
    let mut regexvec: Vec<ColorPattern> = vec![];
    for field in colors {
        let color: Color = Color::new(*Scheme::get_value(&scheme, field));

        regexvec.push(ColorPattern {
            pattern: Regex::new(
                &format!(r#"\{prefix}\{{{field}(\.hex|\.rgb|\.rgba|\.strip)?}}"#).to_string(),
            )?,
            replacements: ColorReplacements {
                hex: format_argb_as_rgb([color.alpha, color.red, color.green, color.blue]),
                hex_stripped: format_argb_as_rgb([color.alpha, color.red, color.green, color.blue])
                    [1..]
                    .to_string(),
                rgb: format!("rgb({:?}, {:?}, {:?})", color.red, color.green, color.blue),
                rgba: format!(
                    "rgba({:?}, {:?}, {:?}, {:?})",
                    color.red, color.green, color.blue, color.alpha
                ),
            },
        });
    }
    Ok(Patterns {
        colors: regexvec,
        image: ImagePattern {
            pattern: Regex::new(&format!(r#"\{prefix}\{{image}}"#))?,
            replacement: image,
        },
    })
}
