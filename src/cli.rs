use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("AppInspect CLI for API")
        .version("0.2.1")
        .author("Hurricane Labs")
        .about("A little CLI wrapper around the AppInspect API. Always up-to-date.")
        .trailing_var_arg(true)
        .allow_hyphen_values(true)
        .arg(Arg::new("username")
            .short('u')
            .long("username")
            .value_name("SPLUNK_USER NAME")
            .help("Provide your splunk.com / SplunkAnswers / Splunkbase username. Can also be set as an env var SPLUNK_USERNAME.")
        )
        .arg(Arg::new("password")
            .short('p')
            .long("password")
            .value_name("SPLUNK_PASSWORD")
            .help("Provide your splunk.com / SplunkAnswers / Splunkbase password. Can also be set as an env var SPLUNK_PASSWORD.")
        )
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("SPLUNK_APP.TAR.GZ | SPLUNK_APP.SPL")
            .help("Provide the path to compressed Splunk app you want to upload.")
        )
        .arg(Arg::new("included_tags")
            .short('t')
            .num_args(1..)
            .long("included_tags")
            .required(true)
            .use_value_delimiter(true)
            .help("Multiple tags allowed i.e. -t=foo,bar etc. All tags provided here https://dev.splunk.com/enterprise/reference/appinspect/appinspecttagreference/ can now be passed in as options i.e. -t jquery -t cloud.")
        )
        .arg(Arg::new("generate_file")
            .long("generate_file")
            .required(false)
            .value_name("true|false")
            .default_value("false")
            .help("If set to true, this will generate a file. By default this is false and will output html to the CLI.")
        )
        .arg(Arg::new("html")
            .long("html")
            .required(false)
            .value_name("true|false")
            .default_value("true")
            .help("By default this will generat an HTML file. If set to false then the report will be generated as JSON. Only applicable is 'generate_file' flag is set to true.")
        )
        .arg(Arg::new("report_path")
            .short('r')
            .long("report_path")
            .required(false)
            .value_name("/full/path/to/report/output/")
            .default_value("./")
            .help("Set the full path to where you want the reports to be output. By default this is in the same directory in which appinspect is installed. Can also be set as an env variable REPORT_PATH. IMPORTANT: You must set the generate_file flag to true in order for this to output the report.")
        )
        .arg(Arg::new("timeout")
            .long("timeout")
            .required(false)
            .default_value("300")
            .value_name("SPLUNK_REPORT_TIMEOUT")
            .value_parser(clap::value_parser!(i32))
            .help("By default, the report request will timeout after 300 seconds. You can override this by submitting a time in seconds. You can alternatively set an env var SPLUNK_REPORT_TIMEOUT.")
        )
}
