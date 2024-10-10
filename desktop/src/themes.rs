use std::collections::HashMap;
use crate::config;
pub struct ThemeManager {
    pub themes: Vec<HashMap<String, String>>,
    pub theme_index: u32,
    pub num_themes: u32,
}

impl ThemeManager {
    pub fn new() -> Self {
        ThemeManager {
            themes: vec![HashMap::from([("".to_string(),"".to_string())])],
            theme_index: 0,
            num_themes: 1,
        }
    }
    
    pub fn parse_themes(&mut self, config: &config::Config) {
        // input: Vec<Vec3<String>>

        let mut final_themes: Vec<HashMap<String, String>> = Vec::new();

        for theme in &config.themes {
            final_themes.push(
                HashMap::from([
                    ("BG".to_string(), theme[0].to_string()),
                    ("FG".to_string(), theme[1].to_string()),
                    ("TEXT".to_string(), theme[2].to_string())
                ])
            );
        }

        self.themes = final_themes;
        self.num_themes = self.themes.len() as u32;
    }
}