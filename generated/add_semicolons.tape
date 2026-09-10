Output src/generated/add_semicolons.mp4
Require hx

Hide
Set Shell "bash"
Set FontSize 20
Set Width 1200
Set Height 600
Set Padding 0
Set Theme "Catppuccin Mocha"
Set TypingSpeed 150ms
Type "hx -c src/generated/helix-config.toml src/generated/add_semicolons.js"
Enter
Type ":cd src/generated"
Enter
Show

Type "%"
Ctrl+"s"
Type "A"
Type ";"
Escape

Escape
Type ","

Hide
Type ":w!"
Enter
Show

Sleep 2s