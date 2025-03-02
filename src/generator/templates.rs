pub struct Templates;

impl Templates {

    pub fn generic(&self, html_content: &str) -> String {
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

    pub fn markdown(&self, css_style: &str, html_content: &str) -> String {
        format!(
            "<!DOCTYPE html>
            <html lang='en'>
            <head>
                <meta charset='UTF-8'>
                <meta name='viewport' content='width=device-width, initial-scale=1.0'>
                <style>{}</style>
            </head>
            <body>
                <article class='markdown-body'>{}</article>
            </body>
            </html>",
            css_style, html_content
        )
    }

}