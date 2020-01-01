use std::env;

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

pub fn run() -> () {
    let args = parse_args();

    println!("user: {}", args.user);
    println!("project: {}", args.project);
    println!("count: {}", args.count)
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    // env::argsの最初はプログラム名だった。
    match &args[1..] {
        [user, project, count] if count.parse::<usize>().is_ok() => Args::new(
            user.to_string(),
            project.to_string(),
            count.parse::<usize>().unwrap(),
        ),
        [user, project] => Args::new_with_default_count(user.to_string(), project.to_string()),
        _ => panic!("\nPlease use\n\tgithub-issue [USER] [PROJECT] [COUNT(default 5)]\n"),
    }
}
