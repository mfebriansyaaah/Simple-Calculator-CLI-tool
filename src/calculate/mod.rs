pub fn calculate(num1: f64, operator: &str, num2: f64) -> Result<f64, String> {
	match operator {
		"+" => Ok(num1 + num2),
		"-" => Ok(num1 - num2),
		"x" => Ok(num1 * num2),
		"/" => {
			if num2 == 0.0 {
				Err("Cannot divide by zero!".to_string())
			} else {
				Ok(num1 / num2)
			}
		}
		_ => Err(format!("Unknown operator: {}", operator)),
	}
}