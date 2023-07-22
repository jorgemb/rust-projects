use conway_life::application::{App, ApplicationError};

fn main() -> Result<(), ApplicationError> {
    let mut app = App::default();
    app.run()
}
