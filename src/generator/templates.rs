pub struct Templates;

impl Templates {

    pub fn chat_gpt(&self, html_content: &str) -> String {
        format!(r#"
            <html>
            <head>
                <meta charset="utf-8">
                <style>
                    body {{ font-family: 'Helvetica', serif; }}
                </style>
            </head>
            <body>{}</body>
            </html>
        "#, html_content)
    }

}