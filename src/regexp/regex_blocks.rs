pub struct BlocksRegExp;

impl BlocksRegExp {
    
    pub const GET_README_BLOCK: [&'static str; 2] = [
        r"(?i)readme\s*\{",
        r"\}",
    ];

    pub const GET_PATH_VAR: &'static str = r#"(?i)path\s*"([^"]+)""#;

    pub const GET_OPEN_VAR: &'static str = r#"(?i)open\s*"([^"]+)""#;
    
    pub const GET_STYLE_VAR: &'static str = r#"(?i)style\s*"([^"]+)""#;
    
    pub const GET_PRINT_VAR: &'static str = r#"(?i)print\s*"([^"]+)""#;
    
    pub const GET_README_VAR: &'static str = r#"(?i)readme\s*"([^"]+)""#;
    
    pub const GET_COVERS_VAR: &'static str = r#"(?i)covers\s*"([^"]+)""#;
    
    pub const GET_QRCODE_VAR: &'static str = r#"(?i)qrcode\s*"([^"]+)""#;

    pub const GET_COMPRESS_VAR: &'static str = r#"(?i)compress\s*"([^"]+)""#;

    pub const GET_MATH_VAR: &'static str = r#"math\s+['"]([^'"]+)['"]\s*>\s*(\S+)"#;

    pub const GET_PATTERNS_MONLIB_VARS: [&'static str; 7] = [
        r#"(?m)^\s*@name\s+"[^"]+""#,
        r#"(?m)^\s*@version\s+"[^"]+""#,
        r#"(?m)^\s*@description\s+"[^"]+""#,
        r#"(?m)^\s*@author\s+"[^"]+""#,
        r#"(?m)^\s*@license\s+"[^"]+""#,
        r#"(?m)^\s*@privacy\s+"[^"]+""#,
        r#"(?m)^\s*@homepage\s+"[^"]+""#,
    ];
     
}
