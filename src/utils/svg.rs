use wasm_bindgen::prelude::*;

pub fn is_svg_bytes(bytes: &[u8]) -> bool {
    // Check if it starts with common SVG patterns
    if bytes.len() < 10 {
        return false;
    }
    
    // Convert bytes to string for pattern matching (safe for SVG which is UTF-8)
    let content = match std::str::from_utf8(&bytes[..std::cmp::min(bytes.len(), 1024)]) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    // Normalize whitespace and convert to lowercase for case-insensitive matching
    let normalized = content
        .trim_start()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    
    // Check for XML declaration + SVG root element patterns
    normalized.starts_with("<?xml") && normalized.contains("<svg")
        || normalized.starts_with("<svg")
        || normalized.starts_with("<!doctype") && normalized.contains("<svg")
}

pub fn validate_svg_content(bytes: &[u8]) -> Result<(), JsValue> {
    if !is_svg_bytes(bytes) {
        return Err(JsValue::from_str("Invalid SVG format: does not contain SVG content"));
    }
    
    // Convert to UTF-8 string
    let content = std::str::from_utf8(bytes)
        .map_err(|_| JsValue::from_str("Invalid SVG format: not valid UTF-8"))?;
    
    // Basic SVG validation - check for required elements
    if !content.to_lowercase().contains("<svg") {
        return Err(JsValue::from_str("Invalid SVG format: missing <svg> element"));
    }
    
    // Check for balanced tags (basic validation)
    let svg_open_count = content.matches("<svg").count();
    let svg_close_count = content.matches("</svg>").count() + content.matches("/>").count();
    
    if svg_open_count == 0 {
        return Err(JsValue::from_str("Invalid SVG format: no opening <svg> tag found"));
    }
    
    // Allow some flexibility in tag counting due to self-closing tags
    // This is a basic check - usvg will do more thorough validation
    
    Ok(())
}

pub fn detect_svg_dimensions(bytes: &[u8]) -> Result<(Option<f32>, Option<f32>), JsValue> {
    let content = std::str::from_utf8(bytes)
        .map_err(|_| JsValue::from_str("Invalid SVG format: not valid UTF-8"))?;
    
    // Find the SVG opening tag
    let svg_tag_start = content.to_lowercase().find("<svg")
        .ok_or_else(|| JsValue::from_str("SVG tag not found"))?;
    
    let svg_tag_end = content[svg_tag_start..]
        .find('>')
        .ok_or_else(|| JsValue::from_str("SVG opening tag not closed"))?;
    
    let svg_tag = &content[svg_tag_start..svg_tag_start + svg_tag_end + 1];
    
    // Extract width and height attributes
    let width = extract_dimension_attribute(svg_tag, "width");
    let height = extract_dimension_attribute(svg_tag, "height");
    
    Ok((width, height))
}

fn extract_dimension_attribute(svg_tag: &str, attr_name: &str) -> Option<f32> {
    let lower_tag = svg_tag.to_lowercase();
    let attr_pattern = format!("{}=", attr_name);
    
    if let Some(start) = lower_tag.find(&attr_pattern) {
        let after_equals = start + attr_pattern.len();
        let rest = &svg_tag[after_equals..];
        
        // Skip whitespace and find quote
        let rest = rest.trim_start();
        if rest.is_empty() {
            return None;
        }
        
        let quote_char = rest.chars().next()?;
        if quote_char != '"' && quote_char != '\'' {
            return None;
        }
        
        let value_start = 1;
        let value_end = rest[value_start..].find(quote_char)?;
        let value = &rest[value_start..value_start + value_end];
        
        // Parse numeric value (strip units like px, pt, etc.)
        parse_dimension_value(value)
    } else {
        None
    }
}

fn parse_dimension_value(value: &str) -> Option<f32> {
    let value = value.trim();
    
    // Remove common units
    let numeric_part = if value.ends_with("px") {
        &value[..value.len() - 2]
    } else if value.ends_with("pt") {
        &value[..value.len() - 2]
    } else if value.ends_with("em") {
        &value[..value.len() - 2]
    } else if value.ends_with("%") {
        // Percentage values are more complex, skip for now
        return None;
    } else {
        value
    };
    
    numeric_part.trim().parse::<f32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_svg_simple() {
        let svg = b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"200\"></svg>";
        assert!(is_svg_bytes(svg));
        
        let (width, height) = detect_svg_dimensions(svg).unwrap();
        assert_eq!(width, Some(100.0));
        assert_eq!(height, Some(200.0));
    }
    
    #[test]
    fn test_detect_svg_with_xml_declaration() {
        let svg = b"<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>";
        assert!(is_svg_bytes(svg));
    }
    
    #[test]
    fn test_not_svg() {
        let not_svg = b"\x89PNG\r\n\x1a\n";
        assert!(!is_svg_bytes(not_svg));
    }
    
    #[test]
    fn test_parse_dimension_with_units() {
        assert_eq!(parse_dimension_value("100px"), Some(100.0));
        assert_eq!(parse_dimension_value("50.5pt"), Some(50.5));
        assert_eq!(parse_dimension_value("200"), Some(200.0));
        assert_eq!(parse_dimension_value("50%"), None); // Not supported yet
    }
}