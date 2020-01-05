use crate::formatter::print_as_table;
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
    process(args)?;
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<Args, &'static str> {
    // env::argsの最初はプログラム名だった。
    match &args[1..] {
        [user, project, count]
            if count.parse::<usize>().is_ok() && count.parse::<usize>().unwrap() <= 30 =>
        {
            Ok(Args::new(
                user.to_string(),
                project.to_string(),
                count.parse::<usize>().unwrap(),
            ))
        }
        [user, project] => Ok(Args::new_with_default_count(
            user.to_string(),
            project.to_string(),
        )),
        _ => Err("\nPlease use\n\tgithub-issue [USER] [PROJECT] [COUNT(default 5 Max 30)]\n"),
    }
}

fn process(args: Args) -> Result<(), Box<dyn Error>> {
    let (user, project) = (args.user, args.project);
    let responce = fetch(&user, &project)?;
    let mut values = handle_response(responce)?.sort_ascending_order("created_at");
    let mut v = Vec::new();
    for _ in 0..args.count {
        v.push(values.as_array_mut().unwrap().pop().unwrap());
    }

    print_as_table(
        serde_json::to_value(v)?,
        vec!["number", "title", "created_at"],
    );
    Ok(())
}
