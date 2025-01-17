# Conversify

Conversify is a simple Rust program that allows users to convert between Celsius and Fahrenheit temperatures. It's a great example of basic user input handling and function calls in Rust.

## Features
- Convert Celsius to Fahrenheit.
- Convert Fahrenheit to Celsius.

## How It Works
The program prompts the user to:
1. Choose an action:
   - Convert Celsius to Fahrenheit.
   - Convert Fahrenheit to Celsius.
2. Input a temperature value.
3. The program performs the conversion and outputs the result.

## Usage
1. Clone this repository to your local machine:
   ```bash
   git clone <repository_url>
   ```

2. Navigate to the project directory:
   ```bash
   cd conversify
   ```

3. Run the program using Cargo:
   ```bash
   cargo run
   ```

## Sample Output
```text
Welcome to Conversify

Choose your action:
1.Celsius to Fahrenheit
2. Fahrenheit to Celsius

1
Input Celsius value:
25
Celsius to Fahrenheit is 77
```

```text
Welcome to Conversify

Choose your action:
1.Celsius to Fahrenheit
2. Fahrenheit to Celsius

2
Input Fahrenheit value:
77
Fahrenheit to Celsius is 25
```

## Program Explanation
- **Main Function**: Handles user interaction, prompting for choices and delegating conversion tasks to respective functions.
- **`to_fahrenheit` Function**: Converts a given Celsius value to Fahrenheit using the formula:
  
  ```
  Fahrenheit = Celsius * 9 / 5 + 32
  ```

- **`to_celcius` Function**: Converts a given Fahrenheit value to Celsius using the formula:
  
  ```
  Celsius = (Fahrenheit - 32) * 5 / 9
  ```

## Prerequisites
- Rust installed on your system. If you don't have Rust installed, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

## Contributing
Contributions are welcome! If you have ideas for new features or improvements, feel free to open an issue or create a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.

---
