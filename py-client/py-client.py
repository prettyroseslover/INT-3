from colored_text import colored
from parse import args
import ptstart_int_3
import requests


def main():
    match args.command:
        case "check-local-file":
            signature = input().encode()
            if len(signature) > 1024:
                colored.print_red("Signature must not exceed 1 KiB")
                return
            check_local_file(args.url, args.remote_path, signature)

        case "quarantine-local-file":
            quarantine_local_file(args.url, args.remote_path)


def check_local_file(url, remote_path, signature):
    json = ptstart_int_3.serialize_check(remote_path, signature)
    response = send_request(json, url)
    match response.status_code:
        case 200:
            colored.print_green(response.json()["offset"])
        case _:
            colored.print_red(
                f"Response with status {response.status_code}: {response.text}"
            )


def quarantine_local_file(url, remote_path):
    json = ptstart_int_3.serialize_quarantine(remote_path)
    response = send_request(json, url)
    match response.status_code:
        case 200:
            colored.print_green(response.json()["message"])
        case _:
            colored.print_red(
                f"Response with status {response.status_code}: {response.text}"
            )


def send_request(json, url):
    return requests.post(url, data=json, headers={"Content-Type": "application/json"})


if __name__ == "__main__":
    main()
