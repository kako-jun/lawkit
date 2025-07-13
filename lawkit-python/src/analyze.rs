use crate::common_options;
use crate::subcommands::integration_common::{
    get_dataset_name, get_numbers_from_input, output_integration_result,
};
use clap::{ArgMatches, Command};
use lawkit_core::common::output::{create_output_writer, OutputConfig};
use lawkit_core::error::Result;
use lawkit_core::laws::integration::{
    analyze_all_laws, analyze_selected_laws, apply_focus_analysis, compare_laws,
};

pub fn command() -> Command {
    common_options::add_integration_options(common_options::add_common_options(
        common_options::add_input_arg(
            Command::new("analyze").about("複数の統計法則による基本分析"),
        ),
    ))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let dataset_name = get_dataset_name(matches);

    let result = if let Some(laws_str) = matches.get_one::<String>("laws") {
        let selected_laws: Vec<String> =
            laws_str.split(',').map(|s| s.trim().to_string()).collect();
        let mut result = analyze_selected_laws(&numbers, &dataset_name, &selected_laws)?;

        // Apply focus if provided
        if let Some(focus) = matches.get_one::<String>("focus") {
            apply_focus_analysis(&mut result, focus);
        }

        result
    } else if let Some(focus) = matches.get_one::<String>("focus") {
        compare_laws(&numbers, &dataset_name, Some(focus))?
    } else {
        analyze_all_laws(&numbers, &dataset_name)?
    };

    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);

    output_integration_result(&mut writer, &result, &output_config)?;

    std::process::exit(result.risk_level.exit_code());
}
