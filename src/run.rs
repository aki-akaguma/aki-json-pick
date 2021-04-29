use crate::conf::CmdOptConf;
use crate::util::err::BrokenPipeError;
use crate::util::OptColorWhen;
use colored_json::{ColorMode, ColoredFormatter, CompactFormatter, PrettyFormatter};
use jql::walker;
use runnel::RunnelIoe;
use serde_json::{Deserializer, Value};
use std::fmt::Write as FmtWrite;
use std::io::{BufRead, Write};

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let r = run_0(sioe, conf);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn run_0(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let mut json_content = String::new();
    //
    // input
    for line in sioe.pin().lock().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        //let line_len: usize = line_ss.len();
        writeln!(json_content, "{}", line_ss)?;
    }
    //
    // output
    let color_mode = if conf.opt_color == OptColorWhen::Auto {
        ColorMode::On
    } else {
        ColorMode::Off
    };
    let selectors = Some(conf.opt_select.as_str());
    for value in Deserializer::from_str(&json_content).into_iter::<Value>() {
        match value {
            Ok(valid_json) => match walker(&valid_json, selectors) {
                Ok(selection) => {
                    write_selection(sioe, conf, selection, color_mode)?;
                }
                Err(err) => {
                    return Err(anyhow!("Error select JSON: {}", err));
                }
            },
            Err(err) => {
                return Err(anyhow!("Invalid JSON: {}", err));
            }
        }
    }
    //
    Ok(())
}

fn write_selection(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    selection: Value,
    color_mode: ColorMode,
) -> anyhow::Result<()> {
    let mut o = sioe.pout().lock();
    o.write_fmt(format_args!(
        "{}\n",
        (if conf.flg_raw_output && selection.is_string() {
            String::from(selection.as_str().unwrap())
        } else if conf.flg_pretty {
            ColoredFormatter::new(PrettyFormatter::new())
                .to_colored_json(&selection, color_mode)
                .unwrap()
        } else {
            ColoredFormatter::new(CompactFormatter {})
                .to_colored_json(&selection, color_mode)
                .unwrap()
        })
    ))?;
    o.flush()?;
    //
    Ok(())
}
