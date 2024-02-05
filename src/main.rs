fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }
    let type_arg = &args[1];
    let name_arg = if args.len() > 2 {
        args[2..].join(" ")
    } else {
        String::from("")
    };
    
    match type_arg.as_str() {
        "angular-component" => angular_component(&name_arg),
        "angular-service" => angular_service(&name_arg),
        "css" => css(&name_arg),
        "html" => html(&name_arg),
        "java" => java(&name_arg),
        "javascript" => javascript(&name_arg),
        "js" => javascript(&name_arg),
        "json" => json(&name_arg),
        "markdown" => markdown(&name_arg),
        "md" => markdown(&name_arg),
        "python" => python(&name_arg),
        "py" => python(&name_arg),
        "readme" => markdown(&name_arg),
        "react-component" => react_component(&name_arg),
        "typescript" => typescript(&name_arg),
        "ts" => typescript(&name_arg),
        "vue-component" => vue_component(&name_arg),
        "yaml" => yaml(&name_arg),

        "1" => angular_component(&name_arg),
        "2" => angular_service(&name_arg),
        "3" => css(&name_arg),
        "4" => html(&name_arg),
        "5" => java(&name_arg),
        "6" => javascript(&name_arg),
        "7" => json(&name_arg),
        "8" => markdown(&name_arg),
        "9" => python(&name_arg),
        "10" => react_component(&name_arg),
        "11" => typescript(&name_arg),
        "12" => vue_component(&name_arg),
        "13" => yaml(&name_arg),

        _ => {
            println!("Unsupported template: {}", type_arg);
        }
    }
}

fn help() {
    println!("Usage:");
    println!("> np <type> [name]  Create template with specific type and optional name");
    println!("");
    println!("Types:");
    println!("[1] angular-component");
    println!("[2] angular-service");
    println!("[3] css");
    println!("[4] html");
    println!("[5] java");
    println!("[6] javascript");
    println!("[7] json");
    println!("[8] markdown");
    println!("[9] python");
    println!("[10] react-component");
    println!("[11] typescript");
    println!("[12] vue-component");
    println!("[13] yaml");

}

fn angular_component(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.component.ts")
    } else if filename.ends_with(".component.ts") {
        String::from(filename)
    } else {
        format!("{}.component.ts", filename)
    };

    let content = 
r#"import { Component } from '@angular/core';

@Component({
  selector: 'app-my-component',
  template: `
    <div>
      <h1>{{ title }}</h1>
    </div>
  `,
  styles: [`
    h1 {
      color: green;
    }
  `]
})
export class MyComponent {
  title = 'Hello, Angular!';
}"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("Angular Service template created successfully.");
}

fn angular_service(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.service.ts")
    } else if filename.ends_with(".service.ts") {
        String::from(filename)
    } else {
        format!("{}.service.ts", filename)
    };

    let content = 
r#"import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class MyService {

  constructor() { }

  // Example method
  public myMethod(): void {
    // Logic here
  }
}"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("Angular Service template created successfully.");
}

fn css(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.css")
    } else if filename.ends_with(".css") {
        String::from(filename)
    } else {
        format!("{}.css", filename)
    };

    let content = 
r#"body {
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 0;
    background-color: #f4f4f4;
}"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("CSS template created successfully.");
}

fn html(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.html")
    } else if filename.ends_with(".html") {
        String::from(filename)
    } else {
        format!("{}.html", filename)
    };

    let content = 
r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Website</title>
    <link rel="stylesheet" href="./style.css">
    <link rel="icon" href="./favicon.ico" type="image/x-icon">
  </head>
  <body>

	<script src="index.js"></script>
  </body>
</html>"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("HTML template created successfully.");
}

fn java(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.java")
    } else if filename.ends_with(".java") {
        String::from(filename)
    } else {
        format!("{}.java", filename)
    };

    let content = 
r#"public class MyClass {

    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }

    // Example method
    public void myMethod() {
        // Logic here
    }
}"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("Java template created successfully.");
}

fn javascript(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.js")
    } else if filename.ends_with(".js") {
        String::from(filename)
    } else {
        format!("{}.js", filename)
    };

    let content = 
r#""#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("JavaScript template created successfully.");
}

fn json(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.json")
    } else if filename.ends_with(".json") {
        String::from(filename)
    } else {
        format!("{}.json", filename)
    };

    let content = 
r#"{
    "string": "value",
    "number": 123,
    "boolean": true,
    "null": null,
    "object": {
        "key": "value"
    },
    "array": ["item1", "item2", "item3"]
}"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("JSON template created successfully.");
}

fn markdown(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("markdown.md")
    } else if filename.ends_with(".md") {
        String::from(filename)
    } else {
        format!("{}.md", filename)
    };

    let content = 
r#"# My Project

## Introduction

This is a simple project description.

## Features

- Feature 1
- Feature 2
- Feature 3

## Usage

Explain how to use your project.

## Contributing

Instructions for how to contribute.

## License

[MIT](LICENSE)"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("Markdown template created successfully.");
}

fn python(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.py")
    } else if filename.ends_with(".py") {
        String::from(filename)
    } else {
        format!("{}.py", filename)
    };

    let content = 
r#"def main():
    print("Hello, World!")

if __name__ == "__main__":
    main()"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("Python template created successfully.");
}

fn react_component(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.js")
    } else if filename.ends_with(".js") {
        String::from(filename)
    } else {
        format!("{}.js", filename)
    };

    let content = 
r#"import React from 'react';

const MyComponent = (props) => {
    return (
        <div>
            <h1>Hello, React!</h1>
            {/* Add more JSX here */}
        </div>
    );
};

export default MyComponent;"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("React Component template created successfully.");
}

fn typescript(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.ts")
    } else if filename.ends_with(".ts") {
        String::from(filename)
    } else {
        format!("{}.ts", filename)
    };

    let content = 
r#""#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("TypeScript template created successfully.");
}

fn vue_component(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.vue")
    } else if filename.ends_with(".vue") {
        String::from(filename)
    } else {
        format!("{}.vue", filename)
    };

    let content = 
r#"<template>
<div>
  <h1>{{ message }}</h1>
</div>
</template>

<script>
export default {
name: 'MyComponent',
data() {
  return {
    message: 'Hello, Vue!'
  };
}
};
</script>

<style scoped>
h1 {
color: blue;
}
</style>"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("Vue template created successfully.");
}

fn yaml(filename: &str) {
    let filename = if filename.is_empty() {
        String::from("template.yaml")
    } else if filename.ends_with(".yaml") {
        String::from(filename)
    } else {
        format!("{}.yaml", filename)
    };

    let content = 
r#"stringKey: value
numberKey: 123
booleanKey: true
nullKey: null
objectItem:
  key: value
array:
  - item1
  - item2
  - item3"#;

    std::fs::write(filename, content).expect("Unable to write file");
    println!("YAML template created successfully.");
}