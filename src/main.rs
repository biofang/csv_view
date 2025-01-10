use anyhow::{Error,Ok};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *}; // Table
use std::path::PathBuf;
// use std::time::Instant;
use csv::ReaderBuilder;
use clap::{value_parser, Parser};

mod error;
mod utils;
// mod loger;
use crate::utils::*;

fn view_csv(
    delimiter: u8,
    table_width: Option<u16>,
    cell_height: Option<usize>,
    alignment: &str,
    header: bool,
    column_index:bool,
    csv: Option<PathBuf>,
) -> Result<(),Error> {
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(file_reader(csv.as_ref())?);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth);

    // set whole table width
    if let Some(width) = table_width {
        table.set_width(width);  // 设置整个表格的宽度
    } else {
        table.width(); // 获取表格的预期宽度
    }
// -------------------
    if column_index {
        let mut tmp_csv_reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(delimiter)
        .from_reader(file_reader(csv.as_ref())?);

        let col_num =  tmp_csv_reader.records().next().unwrap()?.len();

        let mut col_row = Row::new();

        if let Some(height) = cell_height {
            col_row.max_height(height);
        }
        let col_vec: Vec<usize> = (0..col_num).map(|x| x + 1).collect();
        for index in col_vec{
            let cell: Cell = match alignment {
                "left" => Cell::new(index).set_alignment(CellAlignment::Left).add_attribute(Attribute::Bold).fg(Color::Green),
                "center" => Cell::new(index).set_alignment(CellAlignment::Center).add_attribute(Attribute::Bold).fg(Color::Green),
                "right" => Cell::new(index).set_alignment(CellAlignment::Right).add_attribute(Attribute::Bold).fg(Color::Green),
                _ => Cell::new(index),
            };
            col_row.add_cell(cell);
        }
        table.add_row(col_row);   
    }

// ----------------------
    let mut n = 0usize;
    for rec in csv_reader.records().flatten() {
        n += 1;

        let mut row = Row::new();
        // set cell max height
        if let Some(height) = cell_height {
            row.max_height(height);
        }

        for each in rec.iter() {

            let cell: Cell = match alignment {
                "left" => Cell::new(each).set_alignment(CellAlignment::Left),
                "center" => Cell::new(each).set_alignment(CellAlignment::Center),
                "right" => Cell::new(each).set_alignment(CellAlignment::Right),
                _ => Cell::new(each),
            };
            row.add_cell(cell);
        }

        //csv has header
        // header为true ,column_index为false
        if header && !column_index && n == 1 {
            table.set_header(row);
            continue;
        }
        table.add_row(row);
    }

    println!("{}", table);

    Ok(())
}

// Pretty display of CSV table 

#[derive(Debug, Parser)]
#[command(
    author = "fangj",
    version = "version 0.0.3",
    next_line_help = false,
    about = "Pretty display of CSV table ",
    long_about = None,
)]
#[command(help_template = "{about}\n\nVersion: {version}\
    \nAuthors: {author}\
    \n{before-help}
{usage-heading} {usage}\n\n{all-args}\n\nUse \"{name} -h\" for more information")]

struct Cli {
    /// Set the whole table width, Optional
    #[arg(short = 'w', long = "table_width", value_name = "INT", value_parser = value_parser!(u16).range(0..=65535))]
    table_width: Option<u16>,

    /// If set, truncate content of cells which occupies more than INT lines of space, Optional
    #[arg(short = 'c', long = "cell_height", value_name = "INT")]
    cell_height: Option<usize>,
    
    /// Set the alignment of content for each cell, possible values: {left, center, right}
    #[arg(short ='a', long = "aln", value_name = "STR", default_value_t = String::from("center"))]
    aln: String,
    
    /// Show header in different style, Optional
    #[arg(short = 'H',long = "header",value_name = "Bool")]
    header: bool,

    /// Set delimiter, e.g., in linux -d $'\t' for tab, in powershell -d `t for tab
    #[arg(short = 'd', long = "delimiter", default_value_t = '\t', value_name = "CHAR",)]
    delimiter: char,

    /// If set, display the column numbers on the first line, Optional. If -n is set, -H will not work.
    #[arg(short = 'n', long = "column_num",default_value_t=false, value_name = "Bool",)]
    column_index: bool,

    /// Input CSV file name, if file not specified read data from stdin, [Required]
    #[arg(value_name = "CSV")]
    input: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    println!("{}",args.delimiter as u8);

    view_csv(args.delimiter as u8, args.table_width, args.cell_height, &args.aln, args.header,args.column_index, args.input)?;
    
    Ok(())
}
