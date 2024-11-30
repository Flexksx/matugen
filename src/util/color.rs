use material_colors::color::Argb;
use owo_colors::{OwoColorize, Style};

use prettytable::{format, Cell, Row, Table};
use material_colors::theme::Palettes;
use material_colors::palette::TonalPalette;

use colorsys::Rgb;

use crate::Schemes;

#[cfg(feature = "dump-json")]
use super::arguments::Format;

use matugen::color::format::rgb_from_argb;

pub fn show_color(schemes: &Schemes, source_color: &Argb) {
    let mut table: Table = generate_table_format();

    for ((field, color_light), (_, color_dark)) in std::iter::zip(&schemes.light, &schemes.dark) {
        let color_light: Rgb = rgb_from_argb(*color_light);
        let color_dark: Rgb = rgb_from_argb(*color_dark);

        generate_table_rows(&mut table, field, color_light, color_dark);
    }

    generate_table_rows(
        &mut table,
        "source_color",
        rgb_from_argb(*source_color),
        rgb_from_argb(*source_color),
    );

    table.printstd();
}

#[cfg(feature = "dump-json")]
pub fn dump_json(schemes: &Schemes, source_color: &Argb, format: &Format, palettes: Palettes) {
    use std::collections::HashMap;

    let mut colors_normal_light: HashMap<&str, String> = HashMap::new();
    let mut colors_normal_dark: HashMap<&str, String> = HashMap::new();

    for ((field, color_light), (_, color_dark)) in std::iter::zip(&schemes.light, &schemes.dark) {
        let color_light: Rgb = rgb_from_argb(*color_light);
        let color_dark: Rgb = rgb_from_argb(*color_dark);

        colors_normal_light.insert(field, format_single_color(color_light, format));
        colors_normal_dark.insert(field, format_single_color(color_dark, format));
    }

    colors_normal_light.insert("source_color", format_single_color(rgb_from_argb(*source_color), format));

    println!(
        "{}",
        serde_json::json!({
            "colors": {
                "light": colors_normal_light,
                "dark": colors_normal_dark,
            },
            "palettes": format_palettes(palettes, format),
        })
    );
}

#[cfg(feature = "dump-json")]
fn format_palettes(palettes: Palettes, format: &Format) -> serde_json::Value {
    let primary = format_single_palette(palettes.primary, format);
    let secondary = format_single_palette(palettes.secondary, format);
    let tertiary = format_single_palette(palettes.tertiary, format);  
    let neutral = format_single_palette(palettes.neutral, format); 
    let neutral_variant = format_single_palette(palettes.neutral_variant, format);
    let error = format_single_palette(palettes.error, format);
    serde_json::json!({
        "primary": primary,
        "secondary": secondary,
        "tertiary": tertiary,
        "neutral": neutral,
        "neutral_variant": neutral_variant,
        "error": error,
    })
}

#[cfg(feature = "dump-json")]
fn format_single_palette(palette: TonalPalette, format: &Format) -> serde_json::Value {
    serde_json::json!({
        "0": format_single_color(rgb_from_argb(palette.tone(0)), format),
        "5": format_single_color(rgb_from_argb(palette.tone(5)), format),
        "10": format_single_color(rgb_from_argb(palette.tone(10)), format),
        "15": format_single_color(rgb_from_argb(palette.tone(15)), format),
        "20": format_single_color(rgb_from_argb(palette.tone(20)), format),
        "25": format_single_color(rgb_from_argb(palette.tone(25)), format),
        "30": format_single_color(rgb_from_argb(palette.tone(30)), format),
        "35": format_single_color(rgb_from_argb(palette.tone(35)), format),
        "40": format_single_color(rgb_from_argb(palette.tone(40)), format),
        "45": format_single_color(rgb_from_argb(palette.tone(45)), format),
        "50": format_single_color(rgb_from_argb(palette.tone(50)), format),
        "55": format_single_color(rgb_from_argb(palette.tone(55)), format),
        "60": format_single_color(rgb_from_argb(palette.tone(60)), format),
        "65": format_single_color(rgb_from_argb(palette.tone(65)), format),
        "70": format_single_color(rgb_from_argb(palette.tone(70)), format),
        "75": format_single_color(rgb_from_argb(palette.tone(75)), format),
        "80": format_single_color(rgb_from_argb(palette.tone(80)), format),
        "85": format_single_color(rgb_from_argb(palette.tone(85)), format),
        "90": format_single_color(rgb_from_argb(palette.tone(90)), format),
        "95": format_single_color(rgb_from_argb(palette.tone(95)), format),
        "100": format_single_color(rgb_from_argb(palette.tone(100)), format),
    })
}

#[cfg(feature = "dump-json")]
fn format_single_color(color: Rgb, format: &Format) -> String {
    use matugen::color::format::{format_hex, format_hex_stripped, format_hsl, format_hsla, format_rgb, format_rgba, hsl_from_rgb};

    let fmt = match format {
        Format::Rgb => |c: Rgb| format_rgb(&c),
        Format::Rgba => |c: Rgb| format_rgba(&c, true),
        Format::Hsl => |c: Rgb| format_hsl(&hsl_from_rgb(c)),
        Format::Hsla => |c: Rgb| format_hsla(&hsl_from_rgb(c), true),
        Format::Hex => |c: Rgb| format_hex(&c),
        Format::Strip => |c: Rgb| format_hex_stripped(&c),
    };
    fmt(color)
}

fn generate_table_format() -> Table {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separators(
            &[format::LinePosition::Title],
            format::LineSeparator::new('─', '┼', '├', '┤'),
        )
        .separators(
            &[format::LinePosition::Top],
            format::LineSeparator::new('─', '┬', '╭', '╮'),
        )
        .separators(
            &[format::LinePosition::Bottom],
            format::LineSeparator::new('─', '┴', '╰', '╯'),
        )
        .padding(1, 1)
        .build();

    table.set_format(format);

    table.set_titles(Row::new(vec![
        Cell::new("NAME").style_spec("c"),
        Cell::new("LIGHT").style_spec("c"),
        Cell::new("LIGHT").style_spec("c"),
        Cell::new("DARK").style_spec("c"),
        Cell::new("DARK").style_spec("c"),
    ]));
    table
}

fn generate_table_rows(table: &mut Table, field: &str, color_light: Rgb, color_dark: Rgb) {
    let formatstr = "  ";

    table.add_row(Row::new(vec![
        // Color names
        Cell::new(field).style_spec(""),
        // Light scheme
        Cell::new(color_light.to_hex_string().to_uppercase().as_str()).style_spec("c"),
        Cell::new(format!("{}", formatstr.style(generate_style(&color_light))).as_str())
            .style_spec("c"),
        // Dark scheme
        Cell::new(color_dark.to_hex_string().to_uppercase().as_str()).style_spec("c"),
        Cell::new(format!("{}", formatstr.style(generate_style(&color_dark))).as_str())
            .style_spec("c"),
    ]));
}

fn generate_style(color: &Rgb) -> Style {
    let luma = color.red() as u16 + color.blue() as u16 + color.green() as u16;

    let owo_color: owo_colors::Rgb =
        owo_colors::Rgb(color.red() as u8, color.green() as u8, color.blue() as u8);

    if luma > 500 {
        Style::new().black().on_color(owo_color)
    } else {
        Style::new().white().on_color(owo_color)
    }
}
