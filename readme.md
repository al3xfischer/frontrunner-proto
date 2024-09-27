# frontrunner-proto

Reading times from a serial port and storing them in a msacees db.  

DB-Tables can not be configured.  
This project is a prototype and will be removed once the we are done with the full version of ourn new tool


## Ports 

`frontrunner.exe list`


## Operation

`frontrunner.exe run -p COM[X] -d [db_path]`

Optional parameter: `-b 4800` typically we don't change this 

## Help

`frontrunner.exe` executes the `help` command  by default if no command are used  
`frontrunner.exe help`

For further help to a specific command use `--help` or `-h`.  
For example: `frontunner.exe run --help` 