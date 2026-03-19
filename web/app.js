document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('scan-form');
    const btnText = document.querySelector('.btn-text');
    const loader = document.querySelector('.loader');
    const resultsContainer = document.getElementById('results-container');
    const resultsBody = document.getElementById('results-body');
    const errorMessage = document.getElementById('error-message');

    let scanResults = [];
    const searchInput = document.getElementById('search-input');
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
 
    function finishScan() {
        btnText.textContent = 'Initiate Scan';
        loader.classList.add('hidden');
        form.querySelector('button').disabled = false;
    }
});
