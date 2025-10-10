# Project Launcher - Deployment Guide

## For Users (Simplest Method)

### Linux Users
1. Download the `.AppImage` or `.deb` file from the releases
2. Make it executable (if AppImage): `chmod +x Project-Launcher_*.AppImage`
3. Double-click to run or execute from terminal: `./Project-Launcher_*.AppImage`

### macOS Users
1. Download the `.dmg` file from the releases
2. Open the DMG and drag the app to Applications
3. Run from Applications folder

### Windows Users
1. Download the `.msi` or `.exe` installer from the releases
2. Run the installer
3. Launch from Start Menu or Desktop shortcut

## Configuration

When users first run the app, they'll need to:
1. Click "Add Project" to add their own projects, OR
2. Delete the default projects and add their own

The app stores project configurations in:
- **Linux**: `~/.local/share/com.projectlauncher.app/projects.json`
- **macOS**: `~/Library/Application Support/com.projectlauncher.app/projects.json`
- **Windows**: `%APPDATA%/com.projectlauncher.app/projects.json`

## Prerequisites for Users

Users need to have the following installed:
- **For Java Maven projects**: Maven and Java (via asdf or system-wide)
- **For Java Gradle projects**: Java (Gradle wrapper handles Gradle itself)
- **For Angular projects**: Node.js and npm

## For Developers (Building from Source)

### Prerequisites
- Node.js (v14+)
- Rust (1.77.2+)
- npm
- System dependencies (Linux only):
  ```bash
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev patchelf pkg-config libsoup-3.0-dev
  ```

### Build Steps
1. Clone the repository
2. Navigate to project-launcher directory: `cd project-launcher`
3. Install dependencies: `npm install`
4. Build for production: `npm run tauri build`
5. Find built binaries in: `src-tauri/target/release/bundle/`

### Build Outputs

After building, you'll find different formats:

**Linux**:
- `src-tauri/target/release/bundle/appimage/` - AppImage (most portable)
- `src-tauri/target/release/bundle/deb/` - Debian package
- `src-tauri/target/release/bundle/rpm/` - RPM package

**macOS**:
- `src-tauri/target/release/bundle/dmg/` - DMG installer
- `src-tauri/target/release/bundle/macos/` - App bundle

**Windows**:
- `src-tauri/target/release/bundle/msi/` - MSI installer
- `src-tauri/target/release/bundle/nsis/` - NSIS installer

## Distribution Methods

### Method 1: Direct File Sharing
- Upload the built binaries (AppImage, DMG, MSI) to a shared drive or cloud storage
- Share the download link with users

### Method 2: GitHub Releases (Recommended)
1. Create a GitHub repository
2. Push your code
3. Create a new release
4. Upload the built binaries as release assets
5. Users can download from the releases page

### Method 3: Internal Package Repository
- Set up a company-internal package repository
- Host the installers there
- Users can install via package manager

## Customizing Default Projects

Before building, edit `dist/index.html` around line 180 to change default projects:

```javascript
projects = [
    {
        name: 'your-project-name',
        type: 'angular', // or 'java-maven', 'java-gradle'
        path: '/path/to/project',
        command: 'your-start-command',
        running: false,
        output: ''
    }
];
```

Or remove the default projects initialization so users start with an empty list.

## Troubleshooting

### Linux: "Permission denied" when running AppImage
```bash
chmod +x Project-Launcher_*.AppImage
```

### macOS: "App can't be opened because it's from an unidentified developer"
- Right-click the app → Open → Click "Open" in the dialog

### Windows: "Windows protected your PC"
- Click "More info" → "Run anyway"

## Auto-Updates (Optional)

To implement auto-updates, you can use Tauri's updater feature:
1. Configure updater in `src-tauri/tauri.conf.json`
2. Host update manifests on a server
3. App will check for updates automatically
