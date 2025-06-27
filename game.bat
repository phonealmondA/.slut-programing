@echo off
title SLUT Technology - Game Generation Demo
color 0A

echo.
echo ** SLUT Technology Game Generation Demo **
echo >> Simple Language Understanding Technology
echo >> From .slut intentions to playable Asteroids game
echo.

echo Step 1: Compiling the enhanced transpiler...
cargo build --release
if errorlevel 1 (
    echo !! Failed to compile transpiler
    pause
    exit /b
)

echo.
echo Step 2: Running the complete asteroids game intention program...
echo.

REM Use the complete game file from paste.txt
if not exist "asteroids_complete.slut" (
    echo !! Please save the content from paste.txt as asteroids_complete.slut
    echo    The complete game file is needed for proper game generation.
    pause
    exit /b
)

echo Running SLUT technology on asteroids_complete.slut...
echo.
cargo run --release -- asteroids_complete.slut

echo.
echo ** Demo complete! **
echo.

REM Check for generated game files
set GAME_NAME=asteroids_complete

if exist "%GAME_NAME%.exe" (
    echo ✓ Game executable generated: %GAME_NAME%.exe
    echo ✓ Launcher script: run_%GAME_NAME%.bat
    echo.
    set /p play="Would you like to play the generated game now? (y/n): "
    if /i "!play!"=="y" (
        echo Starting the game...
        if exist "run_%GAME_NAME%.bat" (
            start run_%GAME_NAME%.bat
        ) else (
            echo Starting game directly...
            start %GAME_NAME%.exe
        )
    )
) else (
    echo Game generation completed but executable not found.
    echo Check the generated_games/%GAME_NAME%/ directory for source code.
    echo.
    echo Possible reasons:
    echo   1. Game compilation failed (check macroquad dependency)
    echo   2. Game source generated but not compiled
    echo   3. Game detection didn't trigger
)

echo.
echo This demonstrates the complete workflow:
echo   1. Write intentions in .slut file ✓
echo   2. SLUT technology analyzes and executes
echo   3. System detects game variables  
echo   4. User confirms game generation (y/n prompt)
echo   5. Generates complete Rust game project
echo   6. Compiles to standalone executable
echo   7. Creates launcher scripts and documentation
echo.
echo ** Intention-driven programming achieved! **
pause