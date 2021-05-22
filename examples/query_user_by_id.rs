use redmine::Redmine;
use redmine::queries::UserById;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    match (args.next(), args.next(), args.next(), args.next(), args.next()) {
        (_, Some(base_url), Some(api_key), Some(user_id), None) => {
            let redmine = Redmine::new(&base_url, api_key)?;

            let user = redmine.query(UserById(user_id.parse()?))?;

            println!("{:#?}", user);

            Ok(())
        }
        _ => {
            eprintln!("Usage: query_current_user <redmine base url> <redmine api key>");
            std::process::exit(1);
        }
    }

}
