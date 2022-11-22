from config import env
from errors import APIError
import jwt

def encode(payload):
	return jwt.encode(payload, env['JWT_SECRET'], algorithm='HS256')

def decode(token):
	try: 
		body = jwt.decode(token, env['JWT_SECRET'], algorithms=['HS256'])
		return body
	except jwt.ExpiredSignatureError:
		raise APIError('Token expired', 401)
	except jwt.InvalidTokenError:
		raise APIError('Invalid token', 401)
	except Exception as error:
		raise APIError('JWT error: ' + str(error), 401)