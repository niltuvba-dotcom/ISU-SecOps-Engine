document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('scan-form');
    const btnText = document.querySelector('.btn-text');
    const loader = document.querySelector('.loader');
    const resultsContainer = document.getElementById('results-container');
    const resultsBody = document.getElementById('results-body');
    const errorMessage = document.getElementById('error-message');

    let scanResults = [];
    const searchInput = document.getElementById('search-input');
 
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
 
        try {
            const response = await fetch('/api/fingerprint', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ target, ports })
            });
 
            if (!response.ok) {
                throw new Error(`Server returned ${response.status}: ${await response.text()}`);
            }
 
            scanResults = await response.json();
            renderTable(scanResults);
            resultsContainer.classList.remove('hidden');
 
        } catch (error) {
            errorMessage.textContent = error.message;
            errorMessage.classList.remove('hidden');
        } finally {
            btnText.textContent = 'Initiate Scan';
            loader.classList.add('hidden');
            form.querySelector('button').disabled = false;
        }
    });
});
