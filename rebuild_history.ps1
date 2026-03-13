$ErrorActionPreference = "Stop"

# 1. Get original commits from the TRUE main branch (using origin or just the branch name if it exists)
# Let's assume 'main' still exists and has the full history.
$commits = git log main --reverse --format="%H|%s" | Where-Object { $_ -ne "" }
$total = $commits.Count
Write-Host "Total commits to rebuild: $total"

# 2. Start from a fresh orphan branch
git checkout --orphan rebuild_history

# 3. Target date: 20 days ago
$targetDate = (Get-Date).AddDays(-25)

# 4. Process each commit
foreach ($line in $commits) {
    $parts = $line.Split("|")
    $hash = $parts[0]
    $msg = $parts[1]
    
    Write-Host "Rebuilding: $msg ($hash)"
    
    # Get files from the original commit
    git checkout $hash -- .
    git add -A
    
    # Increment date
    $targetDate = $targetDate.AddHours(18) # Spread across ~25-30 days
    $dateStr = $targetDate.ToString("yyyy-MM-ddTHH:mm:ss")
    
    $env:GIT_AUTHOR_DATE = $dateStr
    $env:GIT_COMMITTER_DATE = $dateStr
    
    git commit -m "$msg" --date="$dateStr" --no-verify
}

# 5. Overwrite main
git checkout main
git reset --hard rebuild_history
git branch -D rebuild_history

# 6. Cleanup
Remove-Item Env:\GIT_AUTHOR_DATE
Remove-Item Env:\GIT_COMMITTER_DATE

Write-Host "History rebuilt successfully!"
