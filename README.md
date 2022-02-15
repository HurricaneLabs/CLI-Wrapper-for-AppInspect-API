AppInspect CLI for API 0.1.9
Hurricane Labs

A little CLI wrapper around the AppInspect API. Always up-to-date.

## Install With Homebrew
```
brew tap hurricanelabs/CLI-Wrapper-for-AppInspect-API
brew install CLI-Wrapper-for-AppInspect-API
```
If already installed, then upgrade to the newest version:
```
brew upgrade CLI-Wrapper-for-AppInspect-API
```

## The Manual Install Option
**All you need to download is: target/release/appinspect**

Place it wherever makes sense for you, but something like /usr/local/bin/ might be good.

If you decided to install Appinspect manually you will also need export your path too wherever you decided too install Appinspect:
```
export PATH="$PATH:/usr/local/bin/appinspect"
```

## Set ENV Variables
Optionally, you can set appropriate environment variables in ~/.bash_profile etc. 
```
export SPLUNK_USERNAME="<your_splunkbase_username>"
export SPLUNK_PASSWORD="<your_splunkbase_password>"
export REPORT_PATH="~/Documents/appinspect_reports/"
export SPLUNK_REPORT_TIMEOUT=<int_value>
```

## Additional Notes
Currently, the CLI color output does not work in a Windows environment.

If you get a message about a missing DLL on Windows, you will need to install the Visual Studio redistributable 
package found here: https://www.microsoft.com/en-us/download/details.aspx?id=52685

## Options
```
USAGE:
    appinspect [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <SPLUNK_APP.TAR.GZ | SPLUNK_APP.SPL> Provide the path to compressed Splunk app you want to upload.
        --generate_file <true|false>
            If set to true, this will generate a file. By default this is false and will output html to the CLI.
    -h, --html <true|false>
            By default this will generat an HTML file. If set to false then the report will be generated as JSON. Only
            applicable is 'generate_file' flag is set to true.
    -t, --included_tags <included_tags>...
            Multiple tags allowed i.e. -t foo -t bar etc. All tags provided here
            https://dev.splunk.com/enterprise/reference/appinspect/appinspecttagreference/ can now be passed in as
            options i.e. -t jquery -t cloud.
    -p, --password <SPLUNK_PASSWORD>
            Provide your splunk.com / SplunkAnswers / Splunkbase password. Can also be set as an env var
            SPLUNK_PASSWORD.
    -r, --report_path </full/path/to/report/output/>
            Set the full path to where you want the reports to be output. By default this is in the same directory in
            which appinspect is installed. Can also be set as an env variable REPORT_PATH. IMPORTANT: You must set the
            generate_file flag to true in order for this to output the report.
        --timeout <SPLUNK_REPORT_TIMEOUT>
            By default, the report request will timeout after 300 seconds. You can override this by submitting a time in
            seconds. You can alternatively set an env var SPLUNK_REPORT_TIMEOUT.
    -u, --username <SPLUNK_USERNAME>
            Provide your splunk.com / SplunkAnswers / Splunkbase username. Can also be set as an env var
            SPLUNK_USERNAME.
```

Optionally, if you store your SPLUNK_PASSWORD, SPLUNK_USERNAME, and REPORT_PATH as env vars then you can bypass those arguments.

# Run Example
    appinspect -f /path/to/app.tar.gz -t cloud

# Updates
v 0.1.9
- Handle edge case where password contains a character such as a newline

v 0.1.8
- Tags are now required. Previously, not providing tags would end up with an unexpected panic.

v 0.1.7
- Bug fix for multiple tags provided before additional flags, which would prevent any flags passed in afterward to be bypassed. Multiple flags are now to be provided using -t <value_1> -t <value_2> so on and so forth.

Roadmap (to-do):
- Integrate tests
- Any other random requests
