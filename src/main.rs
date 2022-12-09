extern crate clap;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{thread, time};

use html2text::from_read;
use reqwest::blocking::multipart;
use reqwest::header::{HeaderMap, CACHE_CONTROL, CONTENT_TYPE};
use std::env;
use std::error::Error;
use std::fmt;
mod cli;
mod tags;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

fn get_auth_token(username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Result
    let res = reqwest::blocking::Client::new()
        .get("https://api.splunk.com/2.0/rest/login/splunk")
        .basic_auth(username, Some(password))
        .send()?
        .json()?;

    // Convert to JSON response
    let json_response: serde_json::Value = res;

    // Check status
    if json_response["status_code"] == "401" {
        return Err(
            "Unauthorized! Please check your credentials before attempting to authenticate again."
                .into(),
        );
    }

    // Return Result<String>
    Ok(json_response["data"]["token"].to_string())
}

fn submit_app(
    token: &str,
    file_path: &str,
    included_tags: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers_map = HeaderMap::new();

    headers_map.insert(CACHE_CONTROL, "no_cache".parse().unwrap());

    let client = reqwest::blocking::Client::new();

    let mut form = multipart::Form::new();
    let included_tags_vec = included_tags.split(',').collect::<Vec<&str>>();

    form = form.text("mode", "precert".to_string());

    for included_tag in included_tags_vec.iter() {
        form = form.text("included_tags", included_tag.to_string());
    }

    let final_form = form.file("app_package", &file_path)?;

    let request_build = client
        .post("https://appinspect.splunk.com/v1/app/validate")
        .multipart(final_form)
        .headers(headers_map)
        .bearer_auth(token)
        .send()?
        .json()?;

    // Converts the Future type to a Result<Response, Error>
    let res: serde_json::Value = request_build;

    if res["message"]
        == "File type not allowed.  Files must be [\'gz\', \'tgz\', \'zip\', \'spl\', \'tar\']"
    {
        return Err(Box::new(CustomError(res["message"].to_string())));
    }

    let response_id = serde_json::to_string(&res["request_id"])
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>);

    Ok(response_id.unwrap_or_else(|_| "0".to_string()))
}

fn get_submission_status(
    token: &str,
    request_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = "https://appinspect.splunk.com/v1/app/validate/status/".to_string();

    url.push_str(request_id);

    let client = reqwest::blocking::Client::new();
    let request_build = client.get(&url).bearer_auth(token).send()?.json()?;

    let res: serde_json::Value = request_build;

    Ok(res["status"].to_string())
}

pub fn get_report_results(
    token: &str,
    request_id: &str,
    html: &str,
    generate_file: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = "https://appinspect.splunk.com/v1/app/report/".to_string();
    url.push_str(request_id);

    let mut headers_map = HeaderMap::new();

    if html == "true" || generate_file == "false" {
        headers_map.insert(CONTENT_TYPE, "text/html".parse().unwrap());
    }

    let client = reqwest::blocking::Client::new();

    let request_build = client
        .get(&url)
        .headers(headers_map)
        .bearer_auth(token)
        .send()?;

    if html == "true" || generate_file == "false" {
        let html_request = request_build.text()?;
        let res: String = html_request;
        Ok(res)
    } else {
        let json_request = request_build.json()?;
        let res: serde_json::Value = json_request;
        Ok(res.to_string())
    }
}

pub fn create_report_file(
    report_data: String,
    file: &str,
    html: &str,
    report_path: &str,
) -> std::io::Result<String> {
    if report_path != "./" {
        create_dir_all(report_path)?;
    }
    let file = Path::new(&file);
    let mut path = PathBuf::new();
    let mut file_stem = match file.file_stem() {
        Some(stem) => OsString::from(stem),
        None => OsString::from("_"),
    };

    path.push(report_path);

    if html == "true" {
        file_stem.push(".html");
        path.push(file_stem);
        let mut file = File::create(path)?;

        for line in report_data.lines() {
            let current_line = line.to_string().replace('\n', "");
            file.write_all(current_line.as_bytes())
                .expect("Unable to write data to report.");
        }
    } else {
        file_stem.push(".json");
        path.push(file_stem);
        let file = File::create(path)?;
        ::serde_json::to_writer(&file, &report_data)?;
    }

    Ok("Your report is now ready.".to_string())
}

fn write_color(text: String, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match color {
        Color::Green => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?,
        Color::Red => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?,
        Color::Yellow => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?,
        Color::Black => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)))?,
        Color::Blue => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?,
        Color::Cyan => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?,
        Color::Magenta => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?,
        Color::White => stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?,
        Color::Ansi256(value) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(value))))?
        }
        Color::Rgb(r, g, b) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(r, g, b))))?
        }
        Color::__Nonexhaustive => stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?,
    };

    write!(&mut stdout, "{}", text)
}

pub fn output_report_to_cli(report_data: String) {
    let mut current_color = "";
    let mut begin_printing = false;
    for line in report_data.lines() {
        let current_line = line.to_string().replace('\n', "");

        if current_line.trim().starts_with("<h1>") {
            begin_printing = true;
        }

        if (!current_line.is_empty() || current_line != "\n") && begin_printing {
            let line = from_read(current_line.as_bytes(), 1000);

            if line.trim().starts_with("[ ") || line.trim().is_empty() {
                current_color = "";
            }

            if line.contains("Failures") {
                write_color(line, Color::Red).expect("Could not apply red.");
                current_color = "red";
            } else if line.contains("Warnings") {
                write_color(line, Color::Rgb(240, 135, 22)).expect("Could not apply rgb color.");
                current_color = "orange";
            } else if line.contains("Errors") {
                write_color(line, Color::Rgb(219, 29, 199)).expect("Could not apply rgb color.");
                current_color = "purple";
            } else if line.contains("Not Applicable") {
                write_color(line, Color::Rgb(230, 235, 233)).expect("Could not apply rgb color.");
                current_color = "gray";
            } else if line.contains("Manual Checks") {
                write_color(line, Color::Blue).expect("Could not apply blue.");
                current_color = "blue";
            } else if line.contains("Skipped") {
                write_color(line, Color::Rgb(0, 217, 23)).expect("Could not apply rgb color.");
                current_color = "light_blue";
            } else if line.contains("Successes") {
                write_color(line, Color::Rgb(4, 233, 32)).expect("Could not apply rgb color.");
                current_color = "green";
            } else if line.contains("[ success ]") || current_color == "green" {
                write_color(line, Color::Rgb(4, 233, 32)).expect("Could not apply rgb color.");
                current_color = "green"
            } else if line.contains("[ failure ]")
                || line.contains("[ Failure Summary ]")
                || current_color == "red"
            {
                write_color(line, Color::Red).expect("Could not apply red.");
                current_color = "red";
            } else if line.contains("[ not_applicable ]") || current_color == "gray" {
                write_color(line, Color::Rgb(230, 235, 233)).expect("Could not apply rgb color.");
                current_color = "gray";
            } else if line.contains("[ manual_check ]") || current_color == "blue" {
                write_color(line, Color::Blue).expect("Could not apply blue.");
                current_color = "blue";
            } else if line.contains("[ skipped ]") || current_color == "light_blue" {
                write_color(line, Color::Rgb(0, 217, 235)).expect("Could not apply rgb color.");
                current_color = "light_blue";
            } else if line.contains("[ errors ]") || current_color == "purple" {
                write_color(line, Color::Rgb(219, 29, 199)).expect("Could not apply rgb color.");
                current_color = "purple";
            } else if line.contains("[ warning ]")
                || line.contains("[ Warning Summary ]")
                || current_color == "orange"
            {
                write_color(line, Color::Rgb(240, 135, 22)).expect("Could not apply rgb color.");
                current_color = "orange";
            } else if !line.trim().is_empty() {
                print!("{}", line);
                current_color = "";
            }
            io::stdout().flush().unwrap();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn check_status(
    status_request: Result<String, Box<dyn std::error::Error>>,
    token: String,
    request_id: String,
    file: &str,
    html: &str,
    report_path: &str,
    generate_file: &str,
    timeout_time: &i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut status: String = serde_json::from_str(&status_request.unwrap()).unwrap_or_else(|e| {
        println!("Error obtaining report status. Reason {:?}", e);
        ::std::process::exit(1);
    });

    let mut total_time: i32 = 0;

    while status == "PREPARING" || status == "PROCESSING" {
        if &total_time >= timeout_time {
            return Err(
                "Generating the report has timed out. Please try again later, or try increasing the default timeout time.".into()
            );
        }

        println!("Report is processing. Will wait 30 seconds and check again.");

        let delay = time::Duration::from_secs(30);
        total_time += 30;

        thread::sleep(delay);

        let status_request = get_submission_status(&token, &request_id);

        status = serde_json::from_str(&status_request.unwrap().to_string()).unwrap_or_else(|e| {
            println!("Error obtaining report status. Reason {:?}", e);
            ::std::process::exit(1);
        });
    }

    if status == "SUCCESS" {
        let report_results_response: Result<String, Box<dyn std::error::Error>> =
            get_report_results(&token, &request_id, html, generate_file);

        match report_results_response {
            Ok(result) => {
                if generate_file == "true" {
                    let report = create_report_file(result, file, html, report_path);
                    match report {
                        Ok(_) => println!(
                            "Your report has been created in the following location: {:?}",
                            report_path
                        ),
                        Err(err) => {
                            let error =
                                format!(r#"Could not generate your report. Reason: {:?}"#, err);
                            return Err(error.replace('\"', "'").replace('\'', "").into());
                        }
                    }
                } else {
                    output_report_to_cli(result);
                }
            }
            Err(result) => {
                let error = format!(
                    r#"Could not obtain report results. Reasson: {:?}"#,
                    result.to_string()
                );
                return Err(error.replace('\"', "'").replace('\'', "").into());
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = String::new();
    let mut password = String::new();
    let mut env_splunk_pwd_exists = false;
    let mut env_splunk_username_exists = false;
    let mut env_splunk_timeout = false;
    let mut env_timeout: i32 = 0;

    if let Ok(password_env) = env::var("SPLUNK_PASSWORD") {
        env_splunk_pwd_exists = true;
        password = password_env;
    };

    if let Ok(username_env) = env::var("SPLUNK_USERNAME") {
        env_splunk_username_exists = true;
        username = username_env;
    }

    if let Ok(timeout_env) = env::var("SPLUNK_REPORT_TIMEOUT") {
        env_splunk_timeout = true;
        env_timeout = timeout_env.parse().unwrap_or(300);
        if env_timeout < 30 {
            env_timeout = 300
        }
    }

    let matches = cli::build_cli().get_matches();
    let mut file_args_present: bool = false;
    if matches.get_one::<String>("file").is_some() {
        file_args_present = true;
    }

    let mut timeout_time: i32 = 300;
    let mut generate_file = "false";
    let mut has_password = false;
    let mut has_username = false;
    if matches.get_one::<String>("username").is_some() {
        has_username = true
    }
    if matches.get_one::<String>("password").is_some() {
        has_password = true
    }
    let has_arg_creds = has_username && has_password;
    let has_env_creds = env_splunk_username_exists && env_splunk_pwd_exists;
    if let Some(timeout_time_some) = matches.get_one::<i32>("timeout") {
        timeout_time = *timeout_time_some;
    }

    if has_arg_creds || has_env_creds {
        if !file_args_present {
            return Err(
                "A file to inspect was not provided. Provide one with --file or -f flags."
                    .to_string()
                    .into(),
            );
        }

        #[allow(clippy::needless_late_init)]
        let mut file: String;
        if let Some(some_file) = matches.get_one::<String>("file") {
            file = some_file.to_string();
        } else {
            let error = r#"You must provide a file that you want to inspect."#.to_string();
            return Err(error.replace('\"', "'").replace('\'', "").into());
        }

        if let Some(generate_file_some) = matches.get_one::<String>("generate_file") {
            generate_file = generate_file_some;
        }

        if env_splunk_timeout && timeout_time == 300 {
            timeout_time = env_timeout;
        }

        if generate_file != "true" && generate_file != "false" {
            let error = format!(
                r#"The generate_file flag must be 'true' or 'false', not {:?}"#,
                generate_file
            );
            return Err(error.replace('\"', "'").replace('\'', "").into());
        }

        if has_arg_creds {
            if let Some(username_some) = matches.get_one::<String>("username") {
                username = username_some.to_string();
            }
            if let Some(password_some) = matches.get_one::<String>("password") {
                password = password_some.to_string();
            }
        }

        let existing_tags = tags::tags();

        let included_tags_vec: Vec<String> = matches
            .get_raw("included_tags")
            .expect("included_tags expected")
            .into_iter()
            .map(|osi| osi.to_str().unwrap().into())
            .collect::<Vec<String>>();

        if !included_tags_vec.is_empty() {
            for provided_tag in included_tags_vec.iter() {
                if !existing_tags.contains(provided_tag) {
                    let error = format!(r#"{:?} is not a known tag."#, provided_tag);
                    return Err(error.replace('\"', "'").replace('\'', "").into());
                }
            }
        } else {
            let error = r#"You must provide at least one tag i.e. -t cloud"#.to_string();
            return Err(error.replace('\"', "'").replace('\'', "").into());
        }

        let included_tags = included_tags_vec.join(",");
        let mut html: String = String::from("true");
        let mut report_path: String = String::from("./");

        if let Some(html_some) = matches.get_one::<String>("html") {
            html = html_some.to_string();
        }

        if let Ok(report_path_env) = env::var("REPORT_PATH") {
            report_path = report_path_env;
        } else if let Some(report_path_some) = matches.get_one::<String>("report_path") {
            report_path = report_path_some.to_string();
        }

        if file.starts_with('~') {
            file.replace_range(0..1, dirs::home_dir().unwrap().to_str().unwrap());
        }

        if report_path.starts_with('~') {
            report_path.replace_range(0..1, dirs::home_dir().unwrap().to_str().unwrap());
        }

        let auth_token_result: Result<String, Box<dyn std::error::Error>> =
            get_auth_token(&username, &password);

        let token_str: String = match auth_token_result {
            Ok(tok) => tok,
            Err(err) => {
                let error = format!(r#"Could not obtain auth_token. Reason: {:?}"#, err);
                return Err(error.replace('\"', "'").replace('\'', "").into());
            }
        };

        let token: String = match serde_json::from_str(&token_str) {
            Ok(val) => val,
            Err(_) => {
                let error = String::from("The AppInspect API may have had trouble parsing your password. Check that it doesn't contain invalid characters such as a newline.");
                return Err(error.into());
            }
        };

        let submit_app_response: Result<String, Box<dyn std::error::Error>> =
            submit_app(&token, &file, &included_tags);

        if let Err(request_id_str) = &submit_app_response {
            let error = format!(r#"Error: {:?}"#, &request_id_str.to_string());
            return Err(error.replace('\"', "'").replace('\'', "").into());
        }

        let request_id: String = match serde_json::from_str(&submit_app_response.unwrap()) {
            Ok(res) => res,
            Err(err) => {
                let error = format!(r#"Could not obtain the request_id: {:?}"#, err);
                return Err(error.replace('\"', "'").replace('\'', "").into());
            }
        };

        let status_request: Result<String, Box<dyn std::error::Error>> =
            get_submission_status(&token, &request_id);
        if let Err(status) = check_status(
            status_request,
            token,
            request_id,
            &file,
            &html,
            &report_path,
            generate_file,
            &timeout_time,
        ) {
            return Err(status.to_string().into());
        }
    } else {
        return Err("You must provide your username, password. These can be passed as arguments or set as ENV variables. Run appinspect --help for more information.".to_string().into());
    }

    Ok(())
}
