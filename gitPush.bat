@echo off
title Quantum Git Push Automation - Advanced
color 0A
setlocal enabledelayedexpansion

echo.
echo ** Quantum Git Push Automation - Advanced Edition **
echo.

REM Check if we're in a git repository
git status >nul 2>&1
if errorlevel 1 (
    echo !! Error: Not in a git repository
    echo    Make sure you're in the quantum_slut_transpiler folder
    echo    Current directory: %cd%
    echo.
    echo ** Checking for .git folder...
    if exist ".git" (
        echo    .git folder found - trying git init
        git init
    ) else (
        echo    No .git folder found
        echo    Options:
        echo    1. Run 'git init' to initialize repository
        echo    2. Clone from existing repository
        echo    3. Navigate to correct folder
    )
    pause
    exit /b
)

REM Show current repository info
echo ** Repository Information: **
for /f "tokens=*" %%i in ('git remote get-url origin 2^>nul') do echo    Remote: %%i
for /f "tokens=*" %%i in ('git branch --show-current') do echo    Current Branch: %%i
echo    Current Directory: %cd%
echo.

REM Show current status with color coding
echo ** Current Status: **
git status --short
if errorlevel 1 (
    echo    No changes detected
) else (
    echo    Changes detected above
)
echo.

REM Menu for branch operations
echo ** Branch Operations: **
echo    1. Create new branch
echo    2. Switch existing branch  
echo    3. Continue with current branch
echo.
set /p branch_choice="Choose option (1/2/3): "

if "%branch_choice%"=="1" goto create_branch
if "%branch_choice%"=="2" goto switch_branch
if "%branch_choice%"=="3" goto add_commit
echo Invalid choice, using current branch...
goto add_commit

:create_branch
set /p branch_name="Enter new branch name: "
if "%branch_name%"=="" (
    echo !! Branch name cannot be empty
    pause
    exit /b
)
git checkout -b %branch_name%
if errorlevel 1 (
    echo !! Error creating branch
    pause
    exit /b
)
echo ** Created and switched to new branch: %branch_name%
echo.
goto add_commit

:switch_branch
echo.
echo ** Available branches: **
git branch -a
echo.
set /p target_branch="Enter branch name to switch to: "
if "%target_branch%"=="" (
    echo !! Branch name cannot be empty
    pause
    exit /b
)
git checkout %target_branch%
if errorlevel 1 (
    echo !! Error switching to branch: %target_branch%
    pause
    exit /b
)
echo ** Switched to branch: %target_branch%
echo.

:add_commit
REM Check for changes
git diff --quiet && git diff --cached --quiet
if not errorlevel 1 (
    echo ** No changes to commit **
    echo.
    set /p force_push="Push anyway (sync with remote)? (y/n): "
    if /i "!force_push!"=="y" goto push_only
    if /i "!force_push!"=="yes" goto push_only
    goto end
)

REM Add all changes
echo ** Adding all changes to staging...
git add .

REM Show detailed changes
echo.
echo ** Files to be committed: **
git diff --cached --name-status
echo.
echo ** Summary of changes: **
git diff --cached --stat
echo.

REM Commit message with suggestions
echo ** Commit Message Options: **
echo    1. Quick commit (auto-generated message)
echo    2. Custom commit message
echo    3. Feature commit (new feature)
echo    4. Fix commit (bug fix)
echo    5. Update commit (improvements)
echo.
set /p commit_choice="Choose option (1-5): "

if "%commit_choice%"=="1" goto quick_commit
if "%commit_choice%"=="2" goto custom_commit
if "%commit_choice%"=="3" goto feature_commit
if "%commit_choice%"=="4" goto fix_commit
if "%commit_choice%"=="5" goto update_commit
goto custom_commit

:quick_commit
for /f "tokens=*" %%i in ('git branch --show-current') do set current_branch=%%i
set commit_msg=Update quantum consciousness code on !current_branch!
goto do_commit

:custom_commit
set /p commit_msg="Enter custom commit message: "
goto do_commit

:feature_commit
set /p feature_name="Enter feature name: "
set commit_msg=feat: add %feature_name% to quantum consciousness system
goto do_commit

:fix_commit
set /p fix_desc="Enter fix description: "
set commit_msg=fix: %fix_desc%
goto do_commit

:update_commit
set /p update_desc="Enter update description: "
set commit_msg=update: %update_desc%
goto do_commit

:do_commit
if "%commit_msg%"=="" (
    echo !! Commit message cannot be empty
    pause
    exit /b
)

echo ** Committing changes: "%commit_msg%"
git commit -m "%commit_msg%"
if errorlevel 1 (
    echo !! Error during commit
    pause
    exit /b
)

:push_only
echo.
echo ** Push Options: **
echo    1. Push to current branch
echo    2. Force push (be careful!)
echo    3. Skip push
echo.
set /p push_choice="Choose option (1-3): "

if "%push_choice%"=="2" goto force_push
if "%push_choice%"=="3" goto end
goto normal_push

:normal_push
echo ** Pushing to remote repository...
git push
if errorlevel 1 (
    echo.
    echo ** Setting upstream for new branch...
    for /f "tokens=*" %%i in ('git branch --show-current') do set current_branch=%%i
    git push --set-upstream origin !current_branch!
    if errorlevel 1 (
        echo !! Error during push - check network connection
        pause
        exit /b
    )
)
goto success

:force_push
echo ** WARNING: FORCE PUSHING (this can overwrite remote changes!)
set /p confirm="Are you absolutely sure? (type YES): "
if "!confirm!"=="YES" (
    git push --force
    goto success
) else (
    echo ** Cancelled force push
    goto end
)

:success
echo.
echo ** SUCCESS! **
echo ** Quantum consciousness code successfully updated in the cloud! **
echo ** Available at: **
for /f "tokens=*" %%i in ('git remote get-url origin 2^>nul') do echo    %%i
echo.

:end
echo ** Git automation complete! **
echo.
pause