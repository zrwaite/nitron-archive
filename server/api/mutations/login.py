# pylint: disable=unused-argument
from datetime import date
from ariadne import convert_kwargs_to_snake_case
from database import db
from api.models.user import User
from modules.hash import check_password
from errors import APIError
from .mutation import mutation


@convert_kwargs_to_snake_case
@mutation.field('login')
def login_resolver(obj, info, data):
    try:
        user = User.query.filter(User.username == data['username']).scalar()
        if user:
            valid_password = check_password(data['password'], user.hash)
            if valid_password:
                return {
                    "user": user,
                    "token": ""
                }
            else:
                raise APIError("invalid password")
        else:
            raise APIError("user not found")
    except AttributeError as error:
        raise APIError(str(error))
