
def extract_command(command):
    """
    Extracts a command from a string.

    Args:
        command (str): The string containing the command.

    Returns:
        str: The extracted command."""
    parts = command.split()
    if parts:
        return parts
    
def sys_nano(file_path):
    """
    Opens a file in nano editor.

    Args:
        file_path (str): The path to the file to open.
    """
    os.system(f"nano {file_path}")

def sys_cd(directory):
    """
    Changes the current working directory.

    Args:
        directory (str): The directory to change to.
    """
    os.chdir(directory)

def sys_ls():
    """
    Lists the contents of the current directory.
    """
    os.system("ls -la")

def sys_pwd():
    """
    Prints the current working directory.
    """
    os.system("pwd")

def run_command(command):
    """
    Runs a command using the Term class.

    Args:
        command (str): The command to run.
    """
    coms = {
        "cesnos":sys_nano,"nano":sys_nano,
        "muwenos":sys_cd,"cd":sys_cd,
        "grosserlrar":sys_ls,"ls":sys_ls,
        "grosser":sys_pwd,"pwd":sys_pwd,
    }
    bash = extract_command(command)
    if len(bash) > 0:
        if bash[0] in coms:
            coms[bash[0]](*bash[1:])
