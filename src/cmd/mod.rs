use clap::{App, Arg};

pub trait Registrable<'a, 'b> {
    fn register_option(self, name: &'a str, description: Option<&'a str>) -> Self;
}

impl<'a, 'b> Registrable<'a, 'b> for App<'a, 'b> {
    fn register_option(self, name: &'a str, description: Option<&'a str>) -> Self {
        self.arg(
            Arg::with_name(name)
                .long(name)
                .help(description.unwrap_or(""))
                .takes_value(false)
        )
    }
}

#[test]
fn test_register_option() {
    for input in vec![
        vec!["prog", "--foo"]
    ] {
        let app = App::new("Test");
        let matches = app.register_option("foo", None).get_matches_from(input);
        assert_eq!(matches.is_present("foo"), true);
    }
}
