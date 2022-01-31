use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("AppInspect CLI for API")
        .version("0.1.7")
        .author("Hurricane Labs (Ian Gillespie)")
        .about("A little CLI wrapper around the AppInspect API. Always up-to-date.")
        .setting(clap::AppSettings::TrailingVarArg)
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::with_name("username")
            .short("u")
            .long("username")
            .value_name("SPLUNK_USERNAME")
            .help("Provide your splunk.com / SplunkAnswers / Splunkbase username. Can also be set as an env var SPLUNK_USERNAME.")
            .takes_value(true)
        )
        .arg(Arg::with_name("password")
            .short("p")
            .long("password")
            .value_name("SPLUNK_PASSWORD")
            .help("Provide your splunk.com / SplunkAnswers / Splunkbase password. Can also be set as an env var SPLUNK_PASSWORD.")
            .takes_value(true)
        )
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("SPLUNK_APP.TAR.GZ | SPLUNK_APP.SPL")
            .help("Provide the path to compressed Splunk app you want to upload.")
            .takes_value(true)
        )
        .arg(Arg::with_name("included_tags")
            .short("t")
            .long("included_tags")
            .required(false)
            .help("Multiple tags allowed i.e. -t foo -t bar etc. All tags provided here https://dev.splunk.com/enterprise/reference/appinspect/appinspecttagreference/ can now be passed in as options i.e. -t jquery -t cloud.")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1)
        )
        .arg(Arg::with_name("generate_file")
            .long("generate_file")
            .required(false)
            .value_name("true|false")
            .help("If set to true, this will generate a file. By default this is false and will output html to the CLI.")
            .takes_value(true)
        )
        .arg(Arg::with_name("html")
            .short("h")
            .long("html")
            .required(false)
            .value_name("true|false")
            .help("By default this will generat an HTML file. If set to false then the report will be generated as JSON. Only applicable is 'generate_file' flag is set to true.")
            .takes_value(true)
        )
        .arg(Arg::with_name("report_path")
            .short("r")
            .long("report_path")
            .required(false)
            .value_name("/full/path/to/report/output/")
            .help("Set the full path to where you want the reports to be output. By default this is in the same directory in which appinspect is installed. Can also be set as an env variable REPORT_PATH.")
            .takes_value(true)
        )
        .arg(Arg::with_name("timeout")
            .long("timeout")
            .required(false)
            .value_name("SPLUNK_REPORT_TIMEOUT")
            .help("By default, the report request will timeout after 300 seconds. You can override this by submitting a time in seconds. You can alternatively set an env var SPLUNK_REPORT_TIMEOUT.")
            .takes_value(true)
        )
}
