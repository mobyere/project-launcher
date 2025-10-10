# Cross-Platform Build Guide

## Building for Windows and macOS from Linux

Since Tauri requires building on the target platform, you have several options:

## Option 1: GitHub Actions (Recommended - Free & Automated)

This is the easiest and most reliable method. GitHub will build for all platforms automatically.

### Setup Steps:

1. **Create a GitHub repository** (if you haven't already):
   ```bash
   cd /home/moby/Workspace/projects-launcher
   git init
   git add .
   git commit -m "Initial commit"
   git remote add origin https://github.com/YOUR_USERNAME/project-launcher.git
   git push -u origin main
   ```

2. **Create a release tag** to trigger the build:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

3. **GitHub Actions will automatically**:
   - Build for Linux (AppImage, .deb, .rpm)
   - Build for macOS (.dmg, .app)
   - Build for Windows (.msi, .exe)
   - Create a GitHub Release with all binaries attached

4. **Find your builds**:
   - Go to your repository on GitHub
   - Click "Actions" tab to see build progress
   - Once complete, go to "Releases" to download all binaries

### Manual Trigger (Optional):

You can also trigger builds manually without creating a tag:
1. Go to GitHub → Your Repository → Actions
2. Select "Build Multi-Platform" workflow
3. Click "Run workflow"

## Option 2: Use Virtual Machines

If you want to build locally:

### For Windows:
1. Set up a Windows VM (VirtualBox, VMware, or cloud VM)
2. Install prerequisites:
   - Node.js: https://nodejs.org/
   - Rust: https://rustup.rs/
   - Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/
3. Clone your repository
4. Run: `npm install && npm run tauri build`
5. Binaries in: `src-tauri/target/release/bundle/`

### For macOS:
1. Set up a macOS VM or use a Mac computer
2. Install prerequisites:
   - Xcode Command Line Tools: `xcode-select --install`
   - Node.js: https://nodejs.org/
   - Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Clone your repository
4. Run: `npm install && npm run tauri build`
5. Binaries in: `src-tauri/target/release/bundle/`

## Option 3: Cloud Build Services

### Using GitLab CI/CD (Alternative to GitHub):
Similar to GitHub Actions, GitLab also provides free CI/CD runners for all platforms.

### Using CircleCI:
Offers macOS and Windows build environments.

### Using AppVeyor:
Free for open-source projects, supports Windows builds.

## Option 4: Cross-Compilation (Advanced, Limited Support)

Cross-compilation is technically possible but not officially supported by Tauri and can be very complex. **Not recommended** for beginners.

## Recommended Workflow

For the simplest deployment to Windows and macOS users:

1. **Use GitHub Actions** (Option 1) - This is by far the easiest
2. Push your code to GitHub
3. Create a release tag: `git tag v0.1.0 && git push origin v0.1.0`
4. Wait 10-15 minutes for builds to complete
5. Download all platform binaries from the GitHub Release page
6. Distribute to your users

## Output Files by Platform

### Linux:
- `Project Launcher_0.1.0_amd64.AppImage` ← Most portable
- `Project Launcher_0.1.0_amd64.deb` ← Debian/Ubuntu
- `Project Launcher-0.1.0-1.x86_64.rpm` ← Fedora/RHEL

### macOS:
- `Project Launcher_0.1.0_x64.dmg` ← Intel Macs
- `Project Launcher_0.1.0_aarch64.dmg` ← Apple Silicon (M1/M2/M3)

### Windows:
- `Project Launcher_0.1.0_x64-setup.exe` ← NSIS installer
- `Project Launcher_0.1.0_x64_en-US.msi` ← MSI installer

## Testing

Before distributing:
1. Test each platform binary on the target OS
2. Verify all features work correctly
3. Check that projects start/stop properly
4. Ensure ports are released when stopping

## Troubleshooting

### GitHub Actions fails on macOS:
- Check if you need to sign the macOS app (not required for internal distribution)
- Add `CSC_IDENTITY_AUTO_DISCOVERY: false` to environment variables

### GitHub Actions fails on Windows:
- Ensure all paths use forward slashes in the workflow file
- Check Windows-specific dependencies are installed

### Build takes too long:
- GitHub Actions has a 6-hour timeout per job (plenty of time)
- Consider caching dependencies to speed up builds

## Alternative: Request Team Members to Build

If you have team members with Windows or macOS:
1. Share the repository with them
2. Have them run: `npm install && npm run tauri build`
3. They send you the built binaries
4. You distribute to end users
