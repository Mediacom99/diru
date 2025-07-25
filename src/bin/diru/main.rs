use diru::{Args, Parser, Result, calculate_usage};

fn main() -> Result<()> {
    let args = Args::parse();

    let usage_info = calculate_usage(&args.path, &args)?;

    if args.summarize && !args.all {
        if let Some(total) = usage_info.last() {
            println!("{}", total.format(args.format));
        }
    } else {
        usage_info.iter().for_each(|usage| {
            println!("{}", usage.format(args.format));
        });
    }

    Ok(())
}
