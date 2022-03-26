//! automation_tasks_rs with_lib

use cargo_auto_lib::*;

/// automation_tasks_rs with_lib
fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion. Take care to keep them in sync with the changes.

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("Running automation task: {}", &task);
                if &task == "build" || &task == "b" {
                    task_build();
                } else if &task == "release" || &task == "r" {
                    task_release();
                } else if &task == "docs" || &task == "doc" || &task == "d" {
                    task_docs();
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt
cargo auto release - builds the crate in release mode, version from date, fmt
Run the web server in a separate terminal: 'cd ~/rustprojects/sey_currency_converter_pwa/web_server_folder/;basic-http-server'
Run the web app in your browser: 'http://127.0.0.1:4000/sey_currency_converter_pwa/'
    
cargo auto docs - builds the docs, copy to docs directory
cargo auto publish_to_web - publish to web, git tag
"#
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "publish_to_web"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["with_lib"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion.

// region: tasks

/// example how to call a list of shell commands
fn task_build() {
    #[rustfmt::skip]
    let shell_commands = [
        "cargo fmt", 
        "cargo build"];
    run_shell_commands(shell_commands.to_vec());
    println!(
        r#"
After `cargo auto build`, run the tests and the code. If ok, then `cargo auto release`
"#
    );
}

/// example how to call one shell command and combine with rust code
fn task_release() {
    auto_version_from_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("wasm-pack build --target web --release");
    // copy to local web_server_folder
    run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/sey_currency_converter_pwa/pkg/");
    // run typescript compiler with tsconfig.json
    run_shell_command("tsc -b --verbose");
    println!(
        r#"
1. Run the web server in a separate terminal: 'cd ~/rustprojects/sey_currency_converter_pwa/web_server_folder/;basic-http-server'
2. Run the web app in your browser: 'http://127.0.0.1:4000/sey_currency_converter_pwa/'
After `cargo auto release`, run the tests and the code. If ok, then `cargo auto doc`
"#
    );
}

/// example how to call a list of shell commands and combine with rust code
fn task_docs() {
    auto_md_to_doc_comments();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items ",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    run_shell_command("xdg-open docs/index.html");
    // message to help user with next move
    println!(
        r#"
After `cargo auto doc`, check `docs/index.html`. If ok, then `git commit -am"message"` and `git push`,
then `cargo auto publish to web`
"#
    );
}

/// example how to publish to web, git tag
fn task_publish_to_web() {
    // git tag
    let shell_command = format!("git tag -f -a v{version} -m version_{version}", version = package_version());
    run_shell_command(&shell_command);
    // sync one way to local copy of the web server
    run_shell_command("rsync -a --info=progress2 --delete-after ~/rustprojects/sey_currency_converter_pwa/web_server_folder/sey_currency_converter_pwa/ ~/rustprojects/googlecloud/var/www/bestia.dev/sey_currency_converter_pwa/");
    // sync one-way local copy to web server
    run_shell_command("rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/googlecloud/var/www/bestia.dev/sey_currency_converter_pwa/ luciano_bestia@bestia.dev:/var/www/bestia.dev/sey_currency_converter_pwa/");
    println!(
        r#"
After `cargo auto task_publish_to_web', check the web page `https://bestia.dev/{package_name}/`.
"#,
        package_name = package_name(),
    );
}

// endregion: tasks
