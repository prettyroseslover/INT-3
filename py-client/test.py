import ptstart_int_3


def test_rust_integration_check_local_file():
    remote_path = "/home/utente/test.txt"
    byte = b"ll"
    signature = ",".join(map(str, list(byte)))
    expected = f'{{"command1":"CheckLocalFile","params":{{"path":"{remote_path}","signature":[{signature}]}}}}'
    got = ptstart_int_3.serialize_check(remote_path, byte)
    assert got == expected


def test_rust_integration_quarantine_local_file():
    remote_path = "/home/utente/test.txt"
    expected = (
        f'{{"command1":"QuarantineLocalFile","params":{{"path":"{remote_path}"}}}}'
    )
    got = ptstart_int_3.serialize_quarantine(remote_path)
    assert got == expected
