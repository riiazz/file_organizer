# File Organizer

A command-line tool written in Rust to organize files by extension. It asks for a folder path, an extension, and a target folder, then moves all matching files into the chosen folder.

## Features

- Prompts for a folder and validates its existence
- Supports extensions: pdf, jpg, jpeg, png, txt
- Creates the target folder if it does not exist
- Uses default folders if none is provided:
  - pdf, txt → Doc
  - jpg, jpeg, png → Pictures
  - others → Others

## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/<your-username>/<your-repo>.git
   cd <your-repo>
2. Build the project:
   ```
   cargo build --release
4. Run:
   ```
   cargo run
   ```

## Example:
```
Folder name:
> /home/user/downloads

Extension (pdf, jpg, png, txt):
> pdf

Target folder:
>                                 # (press Enter to use default)

Moved report.pdf
Moved notes.pdf
Successfully moved pdf files to Doc directory!
