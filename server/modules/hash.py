import bcrypt


def hash_password(password) -> bytes:
    return bcrypt.hashpw(password.encode('utf8'), bcrypt.gensalt())


def check_password(password, hash) -> bool:
    return bcrypt.checkpw(password.encode('utf8'), hash)
