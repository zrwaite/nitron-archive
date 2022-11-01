# pylint: disable=unused-argument
from ariadne import convert_kwargs_to_snake_case #ObjectType
from api.models.user import User
from database import db
from errors import APIError
from .query import query


def list_users():
    try:
        users = [user.to_dict() for user in User.query.all()]
        return users
    except Exception as error:
        raise APIError(str(error))


@query.field('listUsers')
def list_users_resolver(obj, info):
    return listUsers()


@convert_kwargs_to_snake_case
@query.field('getUser')
def get_user_resolver(obj, info, username):
    try:
        user = User.query.filter(User.username == username).scalar()
        if user:
            return user
        else:
            raise APIError("user not found")
    except AttributeError as error:
        raise APIError(str(error))