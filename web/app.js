document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('scan-form');
    const btnText = document.querySelector('.btn-text');
    const loader = document.querySelector('.loader');
    const resultsContainer = document.getElementById('results-container');
    const resultsBody = document.getElementById('results-body');
    const errorMessage = document.getElementById('error-message');

    const scanStats = document.getElementById('scan-stats');
    const statHosts = document.getElementById('stat-hosts');
    const statPorts = document.getElementById('stat-ports');
    const statServices = document.getElementById('stat-services');
    const progressContainer = document.getElementById('scan-progress-container');
    const progressFill = document.getElementById('progress-fill');
    const progressPercent = document.getElementById('progress-percent');
    const progressText = document.getElementById('progress-text');
    const liveLogStatus = document.getElementById('live-log-status');
 
    const scanAnalytics = document.getElementById('scan-analytics');
    const serviceList = document.getElementById('service-distribution-list');
 
    let scanResults = [];
    let uniqueHosts = new Set();
    let uniqueServices = new Set();
    let totalTargets = 0;
    let completedTasks = 0;
    const searchInput = document.getElementById('search-input');
    const portsInput = document.getElementById('ports');
    const presetBtns = document.querySelectorAll('.preset-btn');

    presetBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            presetBtns.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            portsInput.value = btn.dataset.ports;
        });
    });

    const btnExportJson = document.getElementById('btn-export-json');
    const btnExportCsv = document.getElementById('btn-export-csv');
    const btnExportPdf = document.getElementById('btn-export-pdf');
 
    btnExportPdf.addEventListener('click', () => {
        if (scanResults.length === 0) return;
        window.print();
    });
 
    btnExportJson.addEventListener('click', () => {
        if (scanResults.length === 0) return;
        const blob = new Blob([JSON.stringify(scanResults, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `aetheris_scan_${new Date().toISOString().slice(0,10)}.json`;
        a.click();
    });
 
    btnExportCsv.addEventListener('click', () => {
        if (scanResults.length === 0) return;
        const headers = ['Target', 'Port', 'State', 'Service', 'Version'];
        const rows = scanResults.map(r => [r.target, r.port, r.state, r.service, r.version].join(','));
        const csvContent = [headers.join(','), ...rows].join('\n');
        const blob = new Blob([csvContent], { type: 'text/csv' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `aetheris_scan_${new Date().toISOString().slice(0,10)}.csv`;
        a.click();
    });
 
    searchInput.addEventListener('input', () => {
        const query = searchInput.value.toLowerCase();
        const filtered = scanResults.filter(item => 
            item.target.toLowerCase().includes(query) ||
            item.port.toString().includes(query) ||
            item.service.toLowerCase().includes(query) ||
            item.version.toLowerCase().includes(query)
        );
        renderTable(filtered);
    });
 
    function renderTable(data) {
        resultsBody.innerHTML = '';
        if (data.length === 0) {
            resultsBody.innerHTML = '<tr><td colspan="5" style="text-align: center;">No results match your search.</td></tr>';
        } else {
            data.forEach(item => {
                const tr = document.createElement('tr');
                const stateBadge = item.state === 'open' ? 'badge badge-open' : 'badge';
                tr.innerHTML = `
                    <td>${item.target}</td>
                    <td>${item.port}/tcp</td>
                    <td><span class="${stateBadge}">${item.state}</span></td>
                    <td style="font-weight: 600;">${item.service}</td>
                    <td style="color: #94a3b8;">${item.version}</td>
                `;
                resultsBody.appendChild(tr);
            });
        }
    }
 
    form.addEventListener('submit', async (e) => {
        e.preventDefault();
 
        const target = document.getElementById('target').value.trim();
        const ports = document.getElementById('ports').value.trim();
 
        // Validation
        let isValid = true;
        
        // Target: IP, CIDR or Hostname
        const targetRegex = /^([a-zA-Z0-9.-]+|(\d{1,3}\.){3}\d{1,3}(\/\d{1,2})?)$/;
        if (!targetRegex.test(target)) {
            showError("Invalid target format. Use IP, CIDR (e.g. /24) or Hostname.");
            document.getElementById('target').classList.add('input-error');
            isValid = false;
        } else {
            document.getElementById('target').classList.remove('input-error');
        }
 
        // Ports: 1-65535 comma separated
        const portRegex = /^(\d{1,5})(,\d{1,5})*$/;
        if (!portRegex.test(ports)) {
            showError("Invalid ports. Use comma separated numbers (e.g. 80,443).");
            document.getElementById('ports').classList.add('input-error');
            isValid = false;
        } else {
            const allInRange = ports.split(',').every(p => {
                const num = parseInt(p);
                return num >= 1 && num <= 65535;
            });
            if (!allInRange) {
                showError("Ports must be between 1 and 65535.");
                document.getElementById('ports').classList.add('input-error');
                isValid = false;
            } else {
                document.getElementById('ports').classList.remove('input-error');
            }
        }
 
        if (!isValid) return;
 
        // Reset UI
        resultsContainer.classList.add('hidden');
        errorMessage.classList.add('hidden');
        btnText.textContent = 'Scanning...';
        loader.classList.remove('hidden');
        form.querySelector('button').disabled = true;
        searchInput.value = ''; // Reset search
        
        scanResults = [];
        uniqueHosts.clear();
        uniqueServices.clear();
        statHosts.textContent = '0';
        statPorts.textContent = '0';
        statServices.textContent = '0';
        scanStats.classList.remove('hidden');
        scanAnalytics.classList.add('hidden');
        
        // Estimate total tasks for progress bar
        const portCount = ports.split(',').length;
        const isCidr = target.includes('/');
        let hostCount = 1;
        if (isCidr) {
            const mask = parseInt(target.split('/')[1]);
            hostCount = Math.pow(2, 32 - mask);
        }
        totalTargets = hostCount * portCount;
        completedTasks = 0;
        updateProgress(0);
        progressContainer.classList.remove('hidden');
        liveLogStatus.textContent = "Scanning targets...";
 
        renderTable(scanResults);
        resultsContainer.classList.remove('hidden');
 
        try {
            const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
            const wsUrl = `${protocol}//${window.location.host}/api/ws/fingerprint`;
            const socket = new WebSocket(wsUrl);
 
            socket.onopen = () => {
                socket.send(JSON.stringify({ target, ports }));
                progressText.textContent = `Scanning ${hostCount} host(s) on ${portCount} port(s)...`;
            };
 
            socket.onmessage = (event) => {
                if (event.data === 'DONE') {
                    updateProgress(100);
                    socket.close();
                    finishScan();
                    return;
                }
 
                try {
                    const result = JSON.parse(event.data);
                    scanResults.push(result);
                    
                    // Update Stats
                    uniqueHosts.add(result.target);
                    if (result.service && result.service !== "unknown") {
                        uniqueServices.add(result.service);
                    }
                    
                    statHosts.textContent = uniqueHosts.size;
                    statPorts.textContent = scanResults.length;
                    statServices.textContent = uniqueServices.size;
                    liveLogStatus.textContent = `Finding: ${result.target}:${result.port} (${result.service})`;
                    renderAnalytics();
                    
                    // Update Progress (Approximate since we only get results for open ports)
                    // For a better progress bar, the backend should send a message for every port checked.
                    // For now, we'll increment progress based on results but cap it at 95% until DONE.
                    completedTasks++;
                    const percent = Math.min(95, Math.round((completedTasks / totalTargets) * 100));
                    updateProgress(percent);
 
                    // Sort locally as results arrive
                    scanResults.sort((a, b) => a.target.localeCompare(b.target) || a.port - b.port);
                    renderTable(scanResults);
                } catch (err) {
                    console.error("Error parsing WS message:", err);
                }
            };
 
            socket.onerror = (error) => {
                throw new Error("WebSocket error occurred.");
            };
 
            socket.onclose = () => {
                finishScan();
            };
 
        } catch (error) {
            errorMessage.textContent = error.message;
            errorMessage.classList.remove('hidden');
            finishScan();
        }
    });
 
    function showError(msg) {
        errorMessage.textContent = msg;
        errorMessage.classList.remove('hidden');
        errorMessage.style.animation = 'shake 0.5s cubic-bezier(.36,.07,.19,.97) both';
        setTimeout(() => errorMessage.style.animation = '', 500);
    }
 
    const historyList = document.getElementById('history-list');
 
    async function fetchHistory() {
        try {
            const response = await fetch('/api/history');
            if (response.ok) {
                const history = await response.json();
                renderHistory(history);
            }
        } catch (err) {
            console.error("Failed to fetch history:", err);
        }
    }
 
    function renderHistory(history) {
        historyList.innerHTML = '';
        if (history.length === 0) {
            historyList.innerHTML = '<p style="color: var(--text-muted); font-size: 0.9rem;">No recent scans.</p>';
            return;
        }
 
        history.forEach(item => {
            const div = document.createElement('div');
            div.className = 'history-item';
            div.innerHTML = `
                <div class="history-info">
                    <strong>${item.target}</strong>
                    <span>${new Date(item.timestamp).toLocaleString()}</span>
                </div>
                <button class="view-btn">View</button>
            `;
            div.querySelector('.view-btn').addEventListener('click', () => {
                scanResults = item.results;
                renderTable(scanResults);
                resultsContainer.classList.remove('hidden');
                document.getElementById('target').value = item.target;
                window.scrollTo({ top: resultsContainer.offsetTop - 50, behavior: 'smooth' });
            });
            historyList.appendChild(div);
        });
    }
 
    fetchHistory();
 
 
    function finishScan() {
        btnText.textContent = 'Initiate Scan';
        loader.classList.add('hidden');
        form.querySelector('button').disabled = false;
        fetchHistory(); // Refresh history after scan
    }
 
    function renderAnalytics() {
        if (scanResults.length === 0) {
            scanAnalytics.classList.add('hidden');
            return;
        }
        scanAnalytics.classList.remove('hidden');
        
        const counts = {};
        scanResults.forEach(res => {
            const svc = res.service || "unknown";
            counts[svc] = (counts[svc] || 0) + 1;
        });
        
        const sorted = Object.entries(counts).sort((a, b) => b[1] - a[1]);
        const max = Math.max(...Object.values(counts));
        
        serviceList.innerHTML = '';
        sorted.forEach(([name, count]) => {
            const percent = Math.round((count / max) * 100);
            const item = document.createElement('div');
            item.className = 'service-item';
            item.innerHTML = `
                <div class="service-name">${name}</div>
                <div class="service-bar-track"><div class="service-bar-fill" style="width: ${percent}%"></div></div>
                <div class="service-count">${count}</div>
            `;
            serviceList.appendChild(item);
        });
    }
 
    function updateProgress(percent) {
        progressFill.style.width = `${percent}%`;
        progressPercent.textContent = `${percent}%`;
        if (percent === 100) {
            progressText.textContent = "Scan complete!";
        }
    }
});
