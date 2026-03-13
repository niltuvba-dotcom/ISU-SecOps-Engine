$ErrorActionPreference = "Stop"

# 1. Get the current branch
$currentBranch = git rev-parse --abbrev-ref HEAD
Write-Host "Current branch: $currentBranch"

# 2. Get all commits in chronological order
$commits = git log --reverse --format="%H" | Where-Object { $_ -ne "" }
$total = $commits.Count
Write-Host "Total commits to redistribute: $total"

# 3. Create a temporary branch from the first commit's parent (or start fresh)
# Since we want to rewrite the WHOLE history, we'll start from an orphan or just the first commit.
# Safest: Create a new orphan branch for the first commit, or just branch from first and amend it.
$firstCommit = $commits[0]
git checkout --orphan temp_history $firstCommit

# 4. Set the start date to 20 days ago
$targetDate = (Get-Date).AddDays(-20)

# 5. Amend the first commit with the start date
$dateStr = $targetDate.ToString("yyyy-MM-ddTHH:mm:ss")
$env:GIT_AUTHOR_DATE = $dateStr
$env:GIT_COMMITTER_DATE = $dateStr
git commit --amend --no-edit --date="$dateStr"
Write-Host "Re-dated first commit to $dateStr"

# 6. Cherry-pick the rest
for ($i = 1; $i -lt $total; $i++) {
    $commitHash = $commits[$i]
    Write-Host "Processing commit $i/$($total-1): $commitHash"
    
    # Cherry-pick without committing yet
    # We use -n (no-commit) to handle metadata manually, or just cherry-pick and amend.
    # Cherry-pick is easier.
    git cherry-pick $commitHash
    
    # Increment date by 1-2 days
    $targetDate = $targetDate.AddHours(32) # Spread it out
    $dateStr = $targetDate.ToString("yyyy-MM-ddTHH:mm:ss")
    
    $env:GIT_AUTHOR_DATE = $dateStr
    $env:GIT_COMMITTER_DATE = $dateStr
    git commit --amend --no-edit --date="$dateStr"
}

# 7. Force update the original branch
git checkout $currentBranch
git reset --hard temp_history
git branch -D temp_history

# 8. Cleanup env vars
Remove-Item Env:\GIT_AUTHOR_DATE
Remove-Item Env:\GIT_COMMITTER_DATE

Write-Host "Success! History redistributed over 20 days."
