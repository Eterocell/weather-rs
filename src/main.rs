use exitfailure::ExitFailure;
use structopt::StructOpt;
use weather_rs::Response;
use weather_rs::{print_response, Input};

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    let response = Response::get(&input.city).await?;
    print_response(response);

    Ok(())
}
