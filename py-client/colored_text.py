class colored:
    GREEN = "\033[92m"
    RED = "\033[91m"
    ENDC = "\033[0m"

    def print_red(text):
        print(f"{colored.RED}{text}{colored.ENDC}")

    def print_green(text):
        print(f"{colored.GREEN}{text}{colored.ENDC}")
