document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('scan-form');
    const btnText = document.querySelector('.btn-text');
    const loader = document.querySelector('.loader');
    const resultsContainer = document.getElementById('results-container');
    const resultsBody = document.getElementById('results-body');
    const errorMessage = document.getElementById('error-message');

    let scanResults = [];
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
 
    btnExportJson.addEventListener('click', () => {
        if (scanResults.length === 0) return;
        const blob = new Blob([JSON.stringify(scanResults, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `secops_scan_${new Date().toISOString().slice(0,10)}.json`;
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
        a.download = `secops_scan_${new Date().toISOString().slice(0,10)}.csv`;
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
 
        const target = document.getElementById('target').value;
        const ports = document.getElementById('ports').value;
 
        // Reset UI
        resultsContainer.classList.add('hidden');
        errorMessage.classList.add('hidden');
        btnText.textContent = 'Scanning...';
        loader.classList.remove('hidden');
        form.querySelector('button').disabled = true;
        searchInput.value = ''; // Reset search
        
        scanResults = [];
        renderTable(scanResults);
        resultsContainer.classList.remove('hidden');
 
        try {
            const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
            const wsUrl = `${protocol}//${window.location.host}/api/ws/fingerprint`;
            const socket = new WebSocket(wsUrl);
 
            socket.onopen = () => {
                socket.send(JSON.stringify({ target, ports }));
            };
 
            socket.onmessage = (event) => {
                if (event.data === 'DONE') {
                    socket.close();
                    finishScan();
                    return;
                }
 
                try {
                    const result = JSON.parse(event.data);
                    scanResults.push(result);
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
 
    form.addEventListener('submit', async (e) => {
        // ... (previous logic)
        // ... update fetchHistory() inside onmessage or onclose
    });
 
    function finishScan() {
        btnText.textContent = 'Initiate Scan';
        loader.classList.add('hidden');
        form.querySelector('button').disabled = false;
        fetchHistory(); // Refresh history after scan
    }
});
