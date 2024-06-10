import argparse
from colored_text import colored


class CustomArgumentParser(argparse.ArgumentParser):
    def error(self, message):
        colored.print_red(f"Error: {message}")
        self.print_usage()
        exit()


parser = CustomArgumentParser(description="Single-threaded Python client")
parser.add_argument(
    "-u",
    "--url",
    type=str,
    default="http://127.0.0.1:3000/",
    help="URL of the server. Default http://127.0.0.1:3000/",
)

subparsers = parser.add_subparsers(dest="command", required=True)

check_parser = subparsers.add_parser("check-local-file")
quarantine_parser = subparsers.add_parser("quarantine-local-file")

check_parser.add_argument(
    "-r", "--remote-path", required=True, type=str, help="Path to remote file to check"
)

quarantine_parser.add_argument(
    "-r",
    "--remote-path",
    required=True,
    type=str,
    help="Paath to remote file to quarantine",
)

args = parser.parse_args()
