# Project Launcher

A lightweight, cross-platform GUI application built with Tauri to easily launch and manage your Java and Angular projects.

## Features

- **Cross-Platform**: Works on Linux, macOS, and Windows
- **Lightweight**: Built with Tauri, resulting in small bundle sizes (compared to Electron)
- **Project Management**: Add, configure, and manage multiple projects
- **Easy Launch**: Start and stop projects with a single click
- **Output Monitoring**: View real-time console output from your projects
- **Persistent Storage**: Projects are saved and loaded automatically

## Prerequisites

- Node.js (v14 or higher)
- Rust (1.77.2 or higher)
- For Java projects: JDK and Maven/Gradle
- For Angular projects: Node.js and npm

## Installation

1. Clone or navigate to the project directory:
```bash
cd project-launcher
```

2. Install dependencies:
```bash
npm install
```

## Development

Run the application in development mode:

```bash
npm run dev
```

This will start the Tauri development server and open the application.

## Building

Build the application for production:

```bash
npm run build
```

The built application will be in `src-tauri/target/release/bundle/`

## Usage

### Adding a Project

1. Click the "+ Add Project" button
2. Fill in the project details:
   - **Project Name**: A friendly name for your project
   - **Project Type**: Select either Java or Angular
   - **Project Path**: Browse or enter the path to your project directory
   - **Start Command**: The command to run your project (e.g., `mvn spring-boot:run` for Java or `npm start` for Angular)
3. Click "Add Project"

### Starting a Project

- Click the "Start" button on any project card
- The project status will change to "Running"
- Output from the project will appear in the Output section

### Stopping a Project

- Click the "Stop" button on a running project
- The process will be terminated

### Deleting a Project

- Click the "×" button in the top-right corner of any project card
- If the project is running, it will be stopped first

## Project Structure

```
project-launcher/
├── dist/                   # Frontend files (HTML, CSS, JS)
│   ├── index.html
│   ├── styles.css
│   └── main.js
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Main Tauri application logic
│   │   └── main.rs
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── package.json
└── README.md
```

## Technology Stack

- **Frontend**: Vanilla HTML/CSS/JavaScript
- **Backend**: Rust
- **Framework**: Tauri 2.x
- **Plugins**:
  - tauri-plugin-dialog (for file browser)
  - tauri-plugin-fs (for file system operations)

## Customization

### Adding More Project Types

To add support for additional project types:

1. Update the `projectType` select options in `dist/index.html`
2. Add CSS styling for the new type in `dist/styles.css`
3. Update the command placeholder logic in `dist/main.js`

### Changing Window Size

Edit `src-tauri/tauri.conf.json`:

```json
"windows": [
  {
    "width": 1200,  // Change this
    "height": 800   // And this
  }
]
```

## Troubleshooting

### Projects won't start

- Verify the project path is correct
- Ensure the start command is valid for your project type
- Check that required tools (Java, Maven, Node.js, etc.) are installed and in your PATH

### Output not showing

- The output polling happens every second
- Some applications may buffer output - try adding flush commands to your projects

### Build errors

- Make sure all dependencies are installed: `npm install`
- Update Rust: `rustup update`
- Clear Rust cache: `cd src-tauri && cargo clean`

## License

ISC

## Contributing

Feel free to submit issues and enhancement requests!
