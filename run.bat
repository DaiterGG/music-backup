cd "D:\music\backup" /d
cargo run
git commit -a -m "auto backup"
git push
set /p DUMMY=Hit ENTER to exit...