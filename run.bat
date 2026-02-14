cd "D:\music\backup" /d
cargo run
git add .
git commit -m "auto backup"
git push
set /p DUMMY=Hit ENTER to exit...