#[derive(Clone, Copy)]
pub struct Config {
    /// The separator used when parsing and formatting multiple artists in metadata formats that does not explicitly support
    /// multiple artists (i.e. artist is a single string separated by the separator)
    pub sep_artist: &'static str,
    /// Parse multiple artists from a single string using the separator specified above
    pub parse_multiple_artists: bool,
    /// language of the comments. Defaults to "XXX"
    pub lang: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sep_artist: ";",
            parse_multiple_artists: true,
            lang: "XXX".to_string(),
        }
    }
}
impl Config {
    pub fn sep_artist(mut self, sep: &'static str) -> Self {
        self.sep_artist = sep;
        self
    }
    pub fn parse_multiple_artists(mut self, parse_multiple_artists: bool) -> Self {
        self.parse_multiple_artists = parse_multiple_artists;
        self
    }
    pub fn language(mut self, lang: &'static str) -> Self {
        self.lang = lang.to_owned();
        self
    }
}
