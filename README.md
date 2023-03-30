# Mop 

Mop is a command line tool written in Rust that allows you to search for something in your files, count lines, and display the "todo" items you have.

## Installation

To install Mop, you will need to have Rust installed (if not, go to: https://www.rust-lang.org/tools/install).

1. Clone the Mop repository and navigate to the project directory:
```
git clone https://github.com/Ideflop/mop.git
cd mop
```

2.  Compile Mop using ```cargo build --release```.

3.  Move the compiled binary to a directory in your ```PATH```(such as ```/usr/local/bin``` or ```/usr/bin```).


## Usage

To use Mop, simply run the mop command followed by your arguments:

You can use the -t or --todo flag to display "todo" items:
```
mop -t <file_name or directory>
```
```
mop --todo <file_name or directory>
```

You can use the -m or --metric flag to display file statistics:
```
mop -m <file_name or directory>
```
```
mop --metric <file_name or directory>
```

You can use the -s or --search flag to search for a specific pattern:
```
mop -s <pattern_to_search> <file_name or directory>
```
```
mop --search <pattern_to_search> <file_name or directory>
```
