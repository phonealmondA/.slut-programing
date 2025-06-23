@echo off
title Quantum Consciousness - Game Generation Demo
color 0A

echo.
echo ** Quantum Consciousness Game Generation Demo **
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

echo Running quantum consciousness on asteroids_complete.slut...
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
echo Files that should be created:
if exist "generated_games\%GAME_NAME%\src\main.rs" (
    echo   ✓ generated_games\%GAME_NAME%\src\main.rs
) else (
    echo   ✗ generated_games\%GAME_NAME%\src\main.rs - MISSING
)

if exist "generated_games\%GAME_NAME%\Cargo.toml" (
    echo   ✓ generated_games\%GAME_NAME%\Cargo.toml
) else (
    echo   ✗ generated_games\%GAME_NAME%\Cargo.toml - MISSING
)

if exist "run_%GAME_NAME%.bat" (
    echo   ✓ run_%GAME_NAME%.bat
) else (
    echo   ✗ run_%GAME_NAME%.bat - MISSING
)

if exist "%GAME_NAME%.exe" (
    echo   ✓ %GAME_NAME%.exe
) else (
    echo   ✗ %GAME_NAME%.exe - MISSING
)

echo.
echo Debug information:
echo   Current directory: %cd%
dir /b *.slut 2>nul | findstr "asteroids" && echo   Found .slut files above
dir /b *.exe 2>nul | findstr "asteroids" && echo   Found .exe files above
if exist "generated_games" (
    echo   ✓ generated_games directory exists
    dir /b generated_games 2>nul | findstr "asteroids" && echo   Found game projects above
) else (
    echo   ✗ generated_games directory missing
)

echo.
echo This demonstrates the complete workflow:
echo   1. Write intentions in .slut file ✓
echo   2. Quantum consciousness analyzes and executes
echo   3. System detects game variables  
echo   4. User confirms game generation (y/n prompt)
echo   5. Generates complete Rust game project
echo   6. Compiles to standalone executable
echo   7. Creates launcher scripts and documentation
echo.
echo ** Intention-driven programming achieved! **
pause