use crate::github_client::*;
use std::error::Error;

struct Args {
    user: String,
    project: String,
    count: usize,
}

impl Args {
    const DEFAULT_COUNT: usize = 5;

    fn new(user: String, project: String, count: usize) -> Self {
        Args {
            user,
            project,
            count,
        }
    }

    fn new_with_default_count(user: String, project: String) -> Self {
        Args {
            user,
            project,
            count: Args::DEFAULT_COUNT,
        }
    }
}

// TODO トレイトで雑に実装したがジェネリクスでもできるかもしれない
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let args = parse_args(args)?;

    println!("user: {}", args.user);
    println!("project: {}", args.project);
    println!("count: {}", args.count);
    process(args);
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<Args, &'static str> {
    // env::argsの最初はプログラム名だった。
    match &args[1..] {
        [user, project, count] if count.parse::<usize>().is_ok() => Ok(Args::new(
            user.to_string(),
            project.to_string(),
            count.parse::<usize>().unwrap(),
        )),
        [user, project] => Ok(Args::new_with_default_count(
            user.to_string(),
            project.to_string(),
        )),
        _ => Err("\nPlease use\n\tgithub-issue [USER] [PROJECT] [COUNT(default 5)]\n"),
    }
}

fn process(args: Args) -> () {
    let (user, project) = (args.user, args.project);
    fetch(&user, &project);
}
