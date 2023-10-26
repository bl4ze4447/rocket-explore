/*
1) Make the copy content function that is recursive for directory until it hits files
1) self.files() in select action should be a HashSet
1) Make it work for the linux file structure
1) Watcher that notices when to update directory content and perform the re-sort
2) Add text manipulation for textbox'es
5) Add multiple tabs support
6) Make the Dir_content smoother when having a lot of elements.
6) Create different view modes (tile/grid)
7) Implement dir_content_details_ui()


!!!!!!!!!!!!!) WHEN RIGHT CLICKING A FILE, THE CONTEXT MENU OPENS, OK RIGHT? NOOOOOO, IF ANOTHER PIECE OF UI GETS IN THAT PLACE
THEN TO THAT PIECE OF UI THE CODE WILL APPLY, IMPLEMENT IT SO THAT WHEN RIGHTCLICKING IT SAVES THAT SAVED FILE
AND WHEN PERFORMING AN ACTION, IT PULLS THAT INFO FROM IT AND THEN IT CLEARS IT, MAKING IT "UNSELECTED" (DONE)
3) Implement the Display Path for UNIX Devices
1) if both cut and copy then context menu which chooses which one to perform (DONE)
1) Write my own opener library (DONE ?)
2) Implement the (go back to parent directory) for going back after we entered the device list UI (DONE)
4) Add support for relative paths (This cant be done, relative to what dummie?)
5) Implement a MODAL Window for better error handling (DONE)
*/