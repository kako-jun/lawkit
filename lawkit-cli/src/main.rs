use clap::{command, Command};

mod colors;
mod common_options;
mod run;
mod subcommands;

const VERSION: &str = "2.2.0";

fn main() {
    let matches = command!()
        .name("lawkit")
        .about("Statistical law analysis toolkit")
        .version(VERSION)
        .subcommand(common_options::add_benf_options(
            common_options::add_common_options(common_options::add_input_arg(
                Command::new("benf").about("Benford's law analysis"),
            )),
        ))
        .subcommand(common_options::add_pareto_options(
            common_options::add_common_options(common_options::add_input_arg(
                Command::new("pareto").about("Pareto principle (80/20 rule) analysis"),
            )),
        ))
        .subcommand(common_options::add_zipf_options(
            common_options::add_common_options(common_options::add_input_arg(
                Command::new("zipf").about("Zipf's law analysis"),
            )),
        ))
        .subcommand(common_options::add_normal_options(
            common_options::add_common_options(common_options::add_input_arg(
                Command::new("normal").about("Normal distribution analysis"),
            )),
        ))
        .subcommand(common_options::add_poisson_options(
            common_options::add_common_options(common_options::add_input_arg(
                Command::new("poisson").about("Poisson distribution analysis"),
            )),
        ))
        .subcommand(subcommands::analyze::command())
        .subcommand(subcommands::validate::command())
        .subcommand(subcommands::diagnose::command())
        .subcommand(
            Command::new("generate")
                .about("Generate sample data following statistical laws")
                .subcommand(common_options::add_generate_benf_options(
                    common_options::add_generate_options(common_options::add_common_options(
                        Command::new("benf").about("Generate Benford's law sample data"),
                    )),
                ))
                .subcommand(common_options::add_generate_pareto_options(
                    common_options::add_generate_options(common_options::add_common_options(
                        Command::new("pareto").about("Generate Pareto distribution sample data"),
                    )),
                ))
                .subcommand(common_options::add_generate_zipf_options(
                    common_options::add_generate_options(common_options::add_common_options(
                        Command::new("zipf").about("Generate Zipf's law sample data"),
                    )),
                ))
                .subcommand(common_options::add_generate_normal_options(
                    common_options::add_generate_options(common_options::add_common_options(
                        Command::new("normal").about("Generate normal distribution sample data"),
                    )),
                ))
                .subcommand(common_options::add_generate_poisson_options(
                    common_options::add_generate_options(common_options::add_common_options(
                        Command::new("poisson").about("Generate Poisson distribution sample data"),
                    )),
                )),
        )
        .subcommand(common_options::add_common_options(
            Command::new("list").about("List available statistical laws"),
        ))
        .subcommand(common_options::add_common_options(
            Command::new("selftest").about("Run self-test for all laws using generated data"),
        ))
        .get_matches();

    let result = match matches.subcommand() {
        Some(("benf", sub_matches)) => subcommands::benf::run(sub_matches),
        Some(("pareto", sub_matches)) => subcommands::pareto::run(sub_matches),
        Some(("zipf", sub_matches)) => subcommands::zipf::run(sub_matches),
        Some(("normal", sub_matches)) => subcommands::normal::run(sub_matches),
        Some(("poisson", sub_matches)) => subcommands::poisson::run(sub_matches),
        Some(("analyze", sub_matches)) => subcommands::analyze::run(sub_matches),
        Some(("validate", sub_matches)) => subcommands::validate::run(sub_matches),
        Some(("diagnose", sub_matches)) => subcommands::diagnose::run(sub_matches),
        Some(("generate", sub_matches)) => run::handle_generate_command(sub_matches),
        Some(("list", sub_matches)) => run::list_laws(sub_matches),
        Some(("selftest", sub_matches)) => run::run_selftest(sub_matches),
        _ => {
            run::show_help();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
