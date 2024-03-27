// automation_tasks_rs for sey_currency_converter_pwa

// region: library with basic automation tasks
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cargo_auto_lib::GREEN;
use cargo_auto_lib::RED;
use cargo_auto_lib::RESET;
use cargo_auto_lib::YELLOW;

// region: library with basic automation tasks

fn main() {
    cl::exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

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
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "test" {
                    task_test();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
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
cargo auto build - builds the crate in debug mode, fmt, increment version
cargo auto release - builds the crate in release mode, fmt, increment version
cargo auto doc - builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push "message" - commits with message and push with mandatory message
      (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
cargo auto publish_to_web - publish to web, git tag
      (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for the web server.)

Run the web server in a separate terminal: 'cd ~/rustprojects/sey_currency_converter_pwa/web_server_folder/;basic-http-server'
Run the web app in your browser: 'http://127.0.0.1:4000/sey_currency_converter_pwa/'
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd(){
/*
    println!(r#"run examples:
cargo run --example example1
"#);
*/
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push", "publish_to_web"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();

    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("wasm-pack build --target web --release");
    // copy to local web_server_folder
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/sey_currency_converter_pwa/pkg/");
    // run typescript compiler with tsconfig.json
    cl::run_shell_command("tsc -b --verbose");
    println!(
        r#"
1. Run the web server in a separate terminal: 'cd ~/rustprojects/{package_name}/web_server_folder/;basic-http-server'
2. Run the web app in your browser: 'http://127.0.0.1:4000/{package_name}/'

After `cargo auto build`, run the tests and the code. If ok then 
run `cargo auto release`
"#,
package_name = cargo_toml.package_name()
    );
    print_examples_cmd();
}

/// cargo build --release
fn task_release() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();

    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("wasm-pack build --target web --release");
    // copy to local web_server_folder
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/sey_currency_converter_pwa/pkg/");
    // run typescript compiler with tsconfig.json
    cl::run_shell_command("tsc -b --verbose");
    println!(
        r#"
1. Run the web server in a separate terminal: 'cd ~/rustprojects/{package_name}/web_server_folder/;basic-http-server'
2. Run the web app in your browser: 'http://127.0.0.1:4000/{package_name}/'
After `cargo auto release`, run the tests and the code. If ok then 
run `cargo auto doc`
"#,
package_name = cargo_toml.package_name()
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::auto_plantuml(&cargo_toml.package_repository().unwrap());
    cl::auto_playground_run_code();
    cl::auto_md_to_doc_comments();

    cl::run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    cl::run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    cl::run_shell_command(&format!(
        r#"printf "<meta http-equiv=\"refresh\" content=\"0; url={}/index.html\" />" > docs/index.html\n"#,
        cargo_toml.package_name().replace("-", "_")
    ));
    // pretty html
    cl::auto_doc_tidy_html().unwrap();
    cl::run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, check `docs/index.html`. If ok then test the documentation code examples{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
}

/// cargo test
fn task_test() {
    cl::run_shell_command("cargo test");
    println!(
r#"
    {YELLOW}After `cargo auto test`. If ok then {RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
    {YELLOW}with mandatory commit message{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    let Some(message) = arg_2 else {
        eprintln!("{RED}Error: Message for commit is mandatory. Exiting.{RESET}");
        // early exit
        return;
    };

    // init repository if needed. If it is not init then normal commit and push.
    if !cl::init_repository_if_needed(&message) {
        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#);
        }
        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::run_shell_command(&format!( r#"git add -A && git diff --staged --quiet || git commit -m "{message}" "#));
        cl::run_shell_command("git push");
        println!(
r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
        );
    }
}

/// publish to web, git tag
fn task_publish_to_web() {
    println!(r#"The ssh-agent is used to authenticate on the web server."#);
    let cargo_toml = cl::CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    cl::run_shell_command(&shell_command);
    // sync one-way local copy to web server
    cl::run_shell_command("rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/sey_currency_converter_pwa/web_server_folder/sey_currency_converter_pwa/ luciano_bestia@bestia.dev:/var/www/bestia.dev/sey_currency_converter_pwa/");
    println!(
        r#"
After `cargo auto publish_to_web', 
check the web page `https://bestia.dev/{package_name}/`
"#,
        package_name = cargo_toml.package_name(),
    );
}

// endregion: tasks
