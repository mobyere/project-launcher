import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

let projects = [];

// DOM Elements
const addProjectBtn = document.getElementById('addProjectBtn');
const addProjectModal = document.getElementById('addProjectModal');
const addProjectForm = document.getElementById('addProjectForm');
const cancelBtn = document.getElementById('cancelBtn');
const projectsList = document.getElementById('projectsList');
const browseBtn = document.getElementById('browseBtn');
const projectTypeSelect = document.getElementById('projectType');
const commandGroup = document.getElementById('commandGroup');
const startCommandInput = document.getElementById('startCommand');

// Load projects on startup
window.addEventListener('DOMContentLoaded', async () => {
    await loadProjects();
    renderProjects();
});

// Load projects from storage
async function loadProjects() {
    try {
        const stored = await invoke('get_projects');
        projects = stored || [];
    } catch (error) {
        console.error('Error loading projects:', error);
        projects = [];
    }
}

// Save projects to storage
async function saveProjects() {
    try {
        await invoke('save_projects', { projects });
    } catch (error) {
        console.error('Error saving projects:', error);
    }
}

// Render projects list
function renderProjects() {
    if (projects.length === 0) {
        projectsList.innerHTML = `
            <div class="empty-state">
                <h2>No Projects Yet</h2>
                <p>Click "Add Project" to get started</p>
            </div>
        `;
        return;
    }

    projectsList.innerHTML = projects.map((project, index) => `
        <div class="project-card" data-index="${index}">
            <div class="project-header">
                <div class="project-info">
                    <h3>${project.name}</h3>
                    <span class="project-type ${project.type}">${project.type}</span>
                </div>
                <button class="delete-btn" onclick="deleteProject(${index})">×</button>
            </div>
            <div class="project-path">${project.path}</div>
            <div class="project-status">
                <span class="status-indicator ${project.running ? 'running' : 'stopped'}"></span>
                <span>${project.running ? 'Running' : 'Stopped'}</span>
            </div>
            <div class="project-actions">
                ${project.running
                    ? `<button class="btn btn-danger" onclick="stopProject(${index})">Stop</button>`
                    : `<button class="btn btn-success" onclick="startProject(${index})">Start</button>`
                }
            </div>
            ${project.output ? `
                <div class="output-section">
                    <div class="output-header">
                        <span>Output:</span>
                        <button class="clear-output" onclick="clearOutput(${index})">Clear</button>
                    </div>
                    <pre>${project.output}</pre>
                </div>
            ` : ''}
        </div>
    `).join('');
}

// Show/hide modal
addProjectBtn.addEventListener('click', () => {
    addProjectModal.classList.remove('hidden');
});

cancelBtn.addEventListener('click', () => {
    addProjectModal.classList.add('hidden');
    addProjectForm.reset();
});

// Browse for directory
browseBtn.addEventListener('click', async () => {
    try {
        const selected = await open({
            directory: true,
            multiple: false,
        });

        if (selected) {
            document.getElementById('projectPath').value = selected;
        }
    } catch (error) {
        console.error('Error selecting directory:', error);
    }
});

// Update start command placeholder based on project type
projectTypeSelect.addEventListener('change', (e) => {
    const type = e.target.value;
    if (type === 'java') {
        startCommandInput.placeholder = 'e.g., mvn spring-boot:run';
        startCommandInput.value = startCommandInput.value || 'mvn spring-boot:run';
    } else if (type === 'angular') {
        startCommandInput.placeholder = 'e.g., npm start';
        startCommandInput.value = startCommandInput.value || 'npm start';
    }
});

// Add new project
addProjectForm.addEventListener('submit', async (e) => {
    e.preventDefault();

    const newProject = {
        name: document.getElementById('projectName').value,
        type: document.getElementById('projectType').value,
        path: document.getElementById('projectPath').value,
        command: document.getElementById('startCommand').value,
        running: false,
        output: ''
    };

    projects.push(newProject);
    await saveProjects();
    renderProjects();

    addProjectModal.classList.add('hidden');
    addProjectForm.reset();
});

// Start project
window.startProject = async (index) => {
    const project = projects[index];

    try {
        await invoke('start_project', {
            path: project.path,
            command: project.command,
            index: index
        });

        project.running = true;
        project.output = 'Starting project...\n';
        renderProjects();
        await saveProjects();

        // Start listening for output
        listenToOutput(index);
    } catch (error) {
        console.error('Error starting project:', error);
        project.output = `Error: ${error}\n`;
        renderProjects();
    }
};

// Stop project
window.stopProject = async (index) => {
    const project = projects[index];

    try {
        await invoke('stop_project', { index: index });
        project.running = false;
        project.output += '\nProject stopped.\n';
        renderProjects();
        await saveProjects();
    } catch (error) {
        console.error('Error stopping project:', error);
        project.output += `\nError stopping: ${error}\n`;
        renderProjects();
    }
};

// Delete project
window.deleteProject = async (index) => {
    if (projects[index].running) {
        await stopProject(index);
    }

    projects.splice(index, 1);
    await saveProjects();
    renderProjects();
};

// Clear output
window.clearOutput = async (index) => {
    projects[index].output = '';
    renderProjects();
    await saveProjects();
};

// Listen to project output
async function listenToOutput(index) {
    // This would need a proper event listener implementation in Tauri
    // For now, we'll poll for output
    const interval = setInterval(async () => {
        if (!projects[index].running) {
            clearInterval(interval);
            return;
        }

        try {
            const output = await invoke('get_project_output', { index: index });
            if (output) {
                projects[index].output += output;
                renderProjects();
            }
        } catch (error) {
            console.error('Error getting output:', error);
        }
    }, 1000);
}
