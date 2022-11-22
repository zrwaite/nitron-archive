from errors import APIError
from .jwt import decode

def get_ctx_auth(info):
	"""Get the auth header"""
	auth = info.context.headers.get("Authorization", None)
	if auth is None:
		raise APIError("Authorization header is expected", 401)
	return auth

def verify_auth(auth):
	"""Verify the auth header"""
	try:
		token = auth.split(" ")[1]
		payload = decode(token)
		return payload
	except Exception as error:
		raise APIError(str(error), 401)

def verify_ctx_auth(info):
	"""Verify the auth header"""
	auth = get_ctx_auth(info)
	return verify_auth(auth)