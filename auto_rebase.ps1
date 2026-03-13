$ErrorActionPreference = "Continue"

# 1. Create a Sequence Editor script to change 'pick' to 'edit'
$seqScript = @"
`$content = Get-Content `$args[0]
`$newContent = `$content -replace '^pick ', 'edit '
`$newContent | Set-Content `$args[0]
"@
$seqScript | Out-File ".\seq_editor.ps1"

# 2. Setup environment for rebase
$env:GIT_SEQUENCE_EDITOR = "powershell -ExecutionPolicy Bypass -File .\seq_editor.ps1"

# 3. Base date: going back 15 days
$base_date = (Get-Date).AddDays(-15)

Write-Host "Starting interactive rebase..."
# This starts the rebase and immediately pauses at the first commit
git rebase -i --root

# 4. Loop through edits
$maxLoops = 50
$loopCnt = 0

while ($loopCnt -lt $maxLoops) {
    $loopCnt++
    # Check git status to see if we are in rebase
    $status = git status
    if ($status -notmatch "interactive rebase in progress" -and $status -notmatch "rebase in progress") {
        Write-Host "Rebase finished or not found."
        break
    }
    
    # We are stopped at a commit, amend it!
    $date_str = $base_date.ToString("yyyy-MM-ddTHH:mm:ss")
    $env:GIT_COMMITTER_DATE = $date_str
    
    Write-Host "Amending commit to date: $date_str"
    git commit --amend --no-edit --date=$date_str
    
    # Increase time by roughly 1 day
    $base_date = $base_date.AddHours(22)
    
    # Continue to next commit
    git rebase --continue
}

Remove-Item ".\seq_editor.ps1" -ErrorAction SilentlyContinue
Write-Host "Done!"
