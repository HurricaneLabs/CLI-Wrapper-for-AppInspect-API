AppInspect CLI for API 1.0.6 
Ian Gillespie

[![pipeline status](https://code.hurricanelabs.net/ian/homebrew-appinspect-api-cli-wrapper/badges/master/pipeline.svg)](https://code.hurricanelabs.net/ian/homebrew-appinspect-api-cli-wrapper/-/commits/master) [![coverage report](https://code.hurricanelabs.net/ian/homebrew-appinspect-api-cli-wrapper/badges/master/coverage.svg)](https://code.hurricanelabs.net/ian/homebrew-appinspect-api-cli-wrapper/-/commits/master)[![Quality Gate Status](https://sonarqube.hurricanelabs.net/api/project_badges/measure?project=AppinspectAppCLI&metric=alert_status)](https://sonarqube.hurricanelabs.net/dashboard?id=AppinspectAppCLI)

A little CLI wrapper around the AppInspect API. Always up-to-date. This is an ALPHA version of the app, so bugs are to be expected.

**This has only been tested on Mac OS, so if you decide to try this on another OS and it explodes, I take no responsibility.**

## Install With Homebrew (Recommended)
Run `brew tap ian/homebrew-appinspect-api-cli-wrapper git@code.hurricanelabs.net:ian/homebrew-appinspect-api-cli-wrapper.git && brew install appinspect`

Then you should be able to run `appinspect <arguments>` on the CLI. See below for details on all the available options.

If for some reason you encounter a problem installing you can try to updating, upgrading, and clearing Homebrew cache by running `brew update && brew upgrade && brew cleanup`.

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
            If set to true, this will generate a file. By default this is false and will output the results to the CLI.
    -h, --html <true|false>
            By default this will generat an HTML file. If set to false then the report will be generated as JSON. Only
            applicable is 'generate_file' flag is set to true.
    -t, --included_tags <[opt1 opt2 ...]>
            All tags provided here https://dev.splunk.com/enterprise/reference/appinspect/appinspecttagreference/ can now be passed in as options i.e. 
            -t jquery cloud
    -p, --password <SPLUNK_PASSWORD>
            Provide your splunk.com / SplunkAnswers / Splunkbase password. Can also be set as an env var
            SPLUNK_PASSWORD.
    -r, --report_path </full/path/to/report/output/>
            Set the full path to where you want the reports to be output. By default this is in the same directory in
            which appinspect is installed. Can also be set as an env variable REPORT_PATH.
        --timeout <SPLUNK_REPORT_TIMEOUT>
            By default, the report request will timeout after 300 seconds. You can override this by submitting a time in
            seconds. You can alternatively set an env var SPLUNK_REPORT_TIMEOUT.
    -u, --username <SPLUNK_USERNAME>
            Provide your splunk.com / SplunkAnswers / Splunkbase username. Can also be set as an env var
            SPLUNK_USERNAME.
```

Optionally, if you store your SPLUNK_PASSWORD, SPLUNK_USERNAME, and REPORT_PATH as env vars then you can bypass those arguments.

# Updates
v 0.1.6
- All tags provided here https://dev.splunk.com/enterprise/reference/appinspect/appinspecttagreference/ can now be passed in as options when using the --tags (-t) option. Split multiple options by a space i.e. -t jquery cloud 

Roadmap (to-do):
- Integrate tests
- Any other random requests
